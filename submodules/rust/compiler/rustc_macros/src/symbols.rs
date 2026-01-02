mkuse!{use std :: collections :: HashMap ;}
mkuse!{use proc_macro2 :: { Span , TokenStream } ;}
mkuse!{use quote :: quote ;}
mkuse!{use syn :: parse :: { Parse , ParseStream , Result } ;}
mkuse!{use syn :: punctuated :: Punctuated ;}
mkuse!{use syn :: { Expr , Ident , Lit , LitStr , Macro , Token , braced } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{kw, { 
                getname!(kw);
                getsrc!(kw);
                getpath!(kw);
                get_deps!(kw);
                get_crates!(kw);
                mkinclude!(kw);
                mkitem!{syn :: custom_keyword ! (Keywords) ;}
mkitem!{syn :: custom_keyword ! (Symbols) ;} 
            }}
mkitem!{mkstruct!{struct Keyword { name : Ident , value : LitStr , }}}
mkitem!{mkimpl!{impl Parse for Keyword { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; input . parse :: < Token ! [:] > () ? ; let value = input . parse () ? ; Ok (Keyword { name , value }) } }}}
mkitem!{mkstruct!{struct Symbol { name : Ident , value : Value , }}}
mkitem!{mkenum!{enum Value { SameAsName , String (LitStr) , Env (LitStr , Macro) , Unsupported (Expr) , }}}
mkitem!{mkimpl!{impl Parse for Symbol { fn parse (input : ParseStream < '_ >) -> Result < Self > { let name = input . parse () ? ; let colon_token : Option < Token ! [:] > = input . parse () ? ; let value = if colon_token . is_some () { input . parse () ? } else { Value :: SameAsName } ; Ok (Symbol { name , value }) } }}}
mkitem!{mkimpl!{impl Parse for Value { fn parse (input : ParseStream < '_ >) -> Result < Self > { let expr : Expr = input . parse () ? ; match & expr { Expr :: Lit (expr) => { if let Lit :: Str (lit) = & expr . lit { return Ok (Value :: String (lit . clone ())) ; } } Expr :: Macro (expr) => { if expr . mac . path . is_ident ("env") && let Ok (lit) = expr . mac . parse_body () { return Ok (Value :: Env (lit , expr . mac . clone ())) ; } } _ => { } } Ok (Value :: Unsupported (expr)) } }}}
mkitem!{mkstruct!{struct Input { keywords : Punctuated < Keyword , Token ! [,] > , symbols : Punctuated < Symbol , Token ! [,] > , }}}
mkitem!{mkimpl!{impl Parse for Input { fn parse (input : ParseStream < '_ >) -> Result < Self > { input . parse :: < kw :: Keywords > () ? ; let content ; braced ! (content in input) ; let keywords = Punctuated :: parse_terminated (& content) ? ; input . parse :: < kw :: Symbols > () ? ; let content ; braced ! (content in input) ; let symbols = Punctuated :: parse_terminated (& content) ? ; Ok (Input { keywords , symbols }) } }}}
mkitem!{mkstruct!{# [derive (Default)] struct Errors { list : Vec < syn :: Error > , }}}
mkitem!{mkimpl!{impl Errors { fn error (& mut self , span : Span , message : String) { self . list . push (syn :: Error :: new (span , message)) ; } }}}

macro_rules! symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbols in module {}", module_path!());
    };
}

mkfn!{
    symbols_introspect!();
    pub (super) fn symbols (input : TokenStream) -> TokenStream { let (mut output , errors) = symbols_with_errors (input) ; output . extend (errors . into_iter () . map (| e | e . to_compile_error ())) ; output }
}
mkitem!{mkstruct!{struct Predefined { idx : u32 , span_of_name : Span , }}}
mkitem!{mkstruct!{struct Entries { map : HashMap < String , Predefined > , }}}
mkitem!{mkimpl!{impl Entries { fn with_capacity (capacity : usize) -> Self { Entries { map : HashMap :: with_capacity (capacity) } } fn insert (& mut self , span : Span , s : & str , errors : & mut Errors) -> u32 { if let Some (prev) = self . map . get (s) { errors . error (span , format ! ("Symbol `{s}` is duplicated")) ; errors . error (prev . span_of_name , "location of previous definition" . to_string ()) ; prev . idx } else { let idx = self . len () ; self . map . insert (s . to_string () , Predefined { idx , span_of_name : span }) ; idx } } fn len (& self) -> u32 { u32 :: try_from (self . map . len ()) . expect ("way too many symbols") } }}}

macro_rules! symbols_with_errors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbols_with_errors in module {}", module_path!());
    };
}

mkfn!{
    symbols_with_errors_introspect!();
    fn symbols_with_errors (input : TokenStream) -> (TokenStream , Vec < syn :: Error >) { let mut errors = Errors :: default () ; let input : Input = match syn :: parse2 (input) { Ok (input) => input , Err (e) => { errors . list . push (e) ; Input { keywords : Default :: default () , symbols : Default :: default () } } } ; let mut keyword_stream = quote ! { } ; let mut symbols_stream = quote ! { } ; let mut prefill_stream = quote ! { } ; let mut entries = Entries :: with_capacity (input . keywords . len () + input . symbols . len () + 10) ; for keyword in input . keywords . iter () { let name = & keyword . name ; let value = & keyword . value ; let value_string = value . value () ; let idx = entries . insert (keyword . name . span () , & value_string , & mut errors) ; prefill_stream . extend (quote ! { # value , }) ; keyword_stream . extend (quote ! { pub const # name : Symbol = Symbol :: new (# idx) ; }) ; } for symbol in input . symbols . iter () { let name = & symbol . name ; let value = match & symbol . value { Value :: SameAsName => name . to_string () , Value :: String (lit) => lit . value () , Value :: Env (..) => continue , Value :: Unsupported (expr) => { errors . list . push (syn :: Error :: new_spanned (expr , concat ! ("unsupported expression for symbol value; implement support for this in " , file ! () ,) ,)) ; continue ; } } ; let idx = entries . insert (symbol . name . span () , & value , & mut errors) ; prefill_stream . extend (quote ! { # value , }) ; symbols_stream . extend (quote ! { pub const # name : Symbol = Symbol :: new (# idx) ; }) ; } for n in 0 .. 10 { let n = n . to_string () ; entries . insert (Span :: call_site () , & n , & mut errors) ; prefill_stream . extend (quote ! { # n , }) ; } for symbol in & input . symbols { let (env_var , expr) = match & symbol . value { Value :: Env (lit , expr) => (lit , expr) , Value :: SameAsName | Value :: String (_) | Value :: Unsupported (_) => continue , } ; if ! proc_macro :: is_available () { errors . error (Span :: call_site () , "proc_macro::tracked_env is not available in unit test" . to_owned () ,) ; break ; } let value = match proc_macro :: tracked_env :: var (env_var . value ()) { Ok (value) => value , Err (err) => { errors . list . push (syn :: Error :: new_spanned (expr , err)) ; continue ; } } ; let idx = if let Some (prev) = entries . map . get (& value) { prev . idx } else { prefill_stream . extend (quote ! { # value , }) ; entries . insert (symbol . name . span () , & value , & mut errors) } ; let name = & symbol . name ; symbols_stream . extend (quote ! { pub const # name : Symbol = Symbol :: new (# idx) ; }) ; } let symbol_digits_base = entries . map ["0"] . idx ; let predefined_symbols_count = entries . len () ; let output = quote ! { const SYMBOL_DIGITS_BASE : u32 = # symbol_digits_base ; # [doc = " The number of predefined symbols; this is the first index for"] # [doc = " extra pre-interned symbols in an Interner created via"] # [doc = " [`Interner::with_extra_symbols`]."] pub const PREDEFINED_SYMBOLS_COUNT : u32 = # predefined_symbols_count ; # [doc (hidden)] # [allow (non_upper_case_globals)] mod kw_generated { use super :: Symbol ; # keyword_stream } # [allow (non_upper_case_globals)] # [doc (hidden)] pub mod sym_generated { use super :: Symbol ; # symbols_stream } impl Interner { # [doc = " Creates an `Interner` with the predefined symbols from the `symbols!` macro and"] # [doc = " any extra symbols provided by external drivers such as Clippy"] pub (crate) fn with_extra_symbols (extra_symbols : & [&'static str]) -> Self { Interner :: prefill (& [# prefill_stream] , extra_symbols ,) } } } ; (output , errors . list) }
}