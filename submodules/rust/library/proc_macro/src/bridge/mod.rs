mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: ops :: { Bound , Range } ;}
mkuse!{use std :: sync :: Once ;}
mkuse!{use std :: { fmt , marker , mem , panic , thread } ;}
mkuse!{use crate :: { Delimiter , Level , Spacing } ;}
mkitem!{# [doc = " Higher-order macro describing the server RPC API, allowing automatic"] # [doc = " generation of type-safe Rust APIs, both client-side and server-side."] # [doc = ""] # [doc = " `with_api!(MySelf, my_self, my_macro)` expands to:"] # [doc = " ```rust,ignore (pseudo-code)"] # [doc = " my_macro! {"] # [doc = "     // ..."] # [doc = "     Literal {"] # [doc = "         // ..."] # [doc = "         fn character(ch: char) -> MySelf::Literal;"] # [doc = "         // ..."] # [doc = "         fn span(my_self: &MySelf::Literal) -> MySelf::Span;"] # [doc = "         fn set_span(my_self: &mut MySelf::Literal, span: MySelf::Span);"] # [doc = "     },"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The first two arguments serve to customize the arguments names"] # [doc = " and argument/return types, to enable several different usecases:"] # [doc = ""] # [doc = " If `my_self` is just `self`, then each `fn` signature can be used"] # [doc = " as-is for a method. If it's anything else (`self_` in practice),"] # [doc = " then the signatures don't have a special `self` argument, and"] # [doc = " can, therefore, have a different one introduced."] # [doc = ""] # [doc = " If `MySelf` is just `Self`, then the types are only valid inside"] # [doc = " a trait or a trait impl, where the trait has associated types"] # [doc = " for each of the API types. If non-associated types are desired,"] # [doc = " a module name (`self` in practice) can be used instead of `Self`."] macro_rules ! with_api { ($ S : ident , $ self : ident , $ m : ident) => { $ m ! { FreeFunctions { fn drop ($ self : $ S :: FreeFunctions) ; fn injected_env_var (var : & str) -> Option < String >; fn track_env_var (var : & str , value : Option <& str >) ; fn track_path (path : & str) ; fn literal_from_str (s : & str) -> Result < Literal <$ S :: Span , $ S :: Symbol >, () >; fn emit_diagnostic (diagnostic : Diagnostic <$ S :: Span >) ; } , TokenStream { fn drop ($ self : $ S :: TokenStream) ; fn clone ($ self : &$ S :: TokenStream) -> $ S :: TokenStream ; fn is_empty ($ self : &$ S :: TokenStream) -> bool ; fn expand_expr ($ self : &$ S :: TokenStream) -> Result <$ S :: TokenStream , () >; fn from_str (src : & str) -> $ S :: TokenStream ; fn to_string ($ self : &$ S :: TokenStream) -> String ; fn from_token_tree (tree : TokenTree <$ S :: TokenStream , $ S :: Span , $ S :: Symbol >,) -> $ S :: TokenStream ; fn concat_trees (base : Option <$ S :: TokenStream >, trees : Vec < TokenTree <$ S :: TokenStream , $ S :: Span , $ S :: Symbol >>,) -> $ S :: TokenStream ; fn concat_streams (base : Option <$ S :: TokenStream >, streams : Vec <$ S :: TokenStream >,) -> $ S :: TokenStream ; fn into_trees ($ self : $ S :: TokenStream) -> Vec < TokenTree <$ S :: TokenStream , $ S :: Span , $ S :: Symbol >>; } , Span { fn debug ($ self : $ S :: Span) -> String ; fn parent ($ self : $ S :: Span) -> Option <$ S :: Span >; fn source ($ self : $ S :: Span) -> $ S :: Span ; fn byte_range ($ self : $ S :: Span) -> Range < usize >; fn start ($ self : $ S :: Span) -> $ S :: Span ; fn end ($ self : $ S :: Span) -> $ S :: Span ; fn line ($ self : $ S :: Span) -> usize ; fn column ($ self : $ S :: Span) -> usize ; fn file ($ self : $ S :: Span) -> String ; fn local_file ($ self : $ S :: Span) -> Option < String >; fn join ($ self : $ S :: Span , other : $ S :: Span) -> Option <$ S :: Span >; fn subspan ($ self : $ S :: Span , start : Bound < usize >, end : Bound < usize >) -> Option <$ S :: Span >; fn resolved_at ($ self : $ S :: Span , at : $ S :: Span) -> $ S :: Span ; fn source_text ($ self : $ S :: Span) -> Option < String >; fn save_span ($ self : $ S :: Span) -> usize ; fn recover_proc_macro_span (id : usize) -> $ S :: Span ; } , Symbol { fn normalize_and_validate_ident (string : & str) -> Result <$ S :: Symbol , () >; } , } } ; }}
mkitem!{macro_rules ! with_api_handle_types { ($ m : ident) => { $ m ! { 'owned : FreeFunctions , TokenStream , 'interned : Span , } } ; }}
mkitem!{macro_rules ! reverse_encode { ($ writer : ident ;) => { } ; ($ writer : ident ; $ first : ident $ (, $ rest : ident) *) => { reverse_encode ! ($ writer ; $ ($ rest) ,*) ; $ first . encode (& mut $ writer , & mut ()) ; } }}
mkitem!{macro_rules ! reverse_decode { ($ reader : ident , $ s : ident ;) => { } ; ($ reader : ident , $ s : ident ; $ first : ident : $ first_ty : ty $ (, $ rest : ident : $ rest_ty : ty) *) => { reverse_decode ! ($ reader , $ s ; $ ($ rest : $ rest_ty) ,*) ; let $ first = <$ first_ty >:: decode (& mut $ reader , $ s) ; } }}
mkmod!{arena, { 
                getname!(arena);
                getsrc!(arena);
                getpath!(arena);
                get_deps!(arena);
                get_crates!(arena);
                mkinclude!(arena);
                 
            }}
mkmod!{buffer, { 
                getname!(buffer);
                getsrc!(buffer);
                getpath!(buffer);
                get_deps!(buffer);
                get_crates!(buffer);
                mkinclude!(buffer);
                 
            }}
mkmod!{client, { 
                getname!(client);
                getsrc!(client);
                getpath!(client);
                get_deps!(client);
                get_crates!(client);
                mkinclude!(client);
                 
            }}
mkmod!{closure, { 
                getname!(closure);
                getsrc!(closure);
                getpath!(closure);
                get_deps!(closure);
                get_crates!(closure);
                mkinclude!(closure);
                 
            }}
mkmod!{fxhash, { 
                getname!(fxhash);
                getsrc!(fxhash);
                getpath!(fxhash);
                get_deps!(fxhash);
                get_crates!(fxhash);
                mkinclude!(fxhash);
                 
            }}
mkmod!{handle, { 
                getname!(handle);
                getsrc!(handle);
                getpath!(handle);
                get_deps!(handle);
                get_crates!(handle);
                mkinclude!(handle);
                 
            }}
mkmod!{rpc, { 
                getname!(rpc);
                getsrc!(rpc);
                getpath!(rpc);
                get_deps!(rpc);
                get_crates!(rpc);
                mkinclude!(rpc);
                 
            }}
mkmod!{selfless_reify, { 
                getname!(selfless_reify);
                getsrc!(selfless_reify);
                getpath!(selfless_reify);
                get_deps!(selfless_reify);
                get_crates!(selfless_reify);
                mkinclude!(selfless_reify);
                 
            }}
mkmod!{server, { 
                getname!(server);
                getsrc!(server);
                getpath!(server);
                get_deps!(server);
                get_crates!(server);
                mkinclude!(server);
                 
            }}
mkmod!{symbol, { 
                getname!(symbol);
                getsrc!(symbol);
                getpath!(symbol);
                get_deps!(symbol);
                get_crates!(symbol);
                mkinclude!(symbol);
                 
            }}
mkuse!{use buffer :: Buffer ;}
mkuse!{pub use rpc :: PanicMessage ;}
mkuse!{use rpc :: { Decode , DecodeMut , Encode , Reader , Writer } ;}
mkitem!{mkstruct!{# [doc = " Configuration for establishing an active connection between a server and a"] # [doc = " client.  The server creates the bridge config (`run_server` in `server.rs`),"] # [doc = " then passes it to the client through the function pointer in the `run` field"] # [doc = " of `client::Client`. The client constructs a local `Bridge` from the config"] # [doc = " in TLS during its execution (`Bridge::{enter, with}` in `client.rs`)."] # [repr (C)] pub struct BridgeConfig < 'a > { # [doc = " Buffer used to pass initial input to the client."] input : Buffer , # [doc = " Server-side function that the client uses to make requests."] dispatch : closure :: Closure < 'a , Buffer , Buffer > , # [doc = " If 'true', always invoke the default panic hook"] force_show_panics : bool , _marker : marker :: PhantomData < * mut () > , }}}
mkmod!{api_tags, { 
                getname!(api_tags);
                getsrc!(api_tags);
                getpath!(api_tags);
                get_deps!(api_tags);
                get_crates!(api_tags);
                mkinclude!(api_tags);
                mkuse!{use super :: rpc :: { DecodeMut , Encode , Reader , Writer } ;}
mkitem!{macro_rules ! declare_tags { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) *;) * }) ,* $ (,) ?) => { $ (pub (super) enum $ name { $ ($ method) ,* } rpc_encode_decode ! (enum $ name { $ ($ method) ,* }) ;) * pub (super) enum Method { $ ($ name ($ name)) ,* } rpc_encode_decode ! (enum Method { $ ($ name (m)) ,* }) ; } }}
mkitem!{with_api ! (self , self , declare_tags) ;} 
            }}
mkitem!{mktrait!{# [doc = " Helper to wrap associated types to allow trait impl dispatch."] # [doc = " That is, normally a pair of impls for `T::Foo` and `T::Bar`"] # [doc = " can overlap, but if the impls are, instead, on types like"] # [doc = " `Marked<T::Foo, Foo>` and `Marked<T::Bar, Bar>`, they can't."] trait Mark { type Unmarked ; fn mark (unmarked : Self :: Unmarked) -> Self ; }}}
mkitem!{mktrait!{# [doc = " Unwrap types wrapped by `Mark::mark` (see `Mark` for details)."] trait Unmark { type Unmarked ; fn unmark (self) -> Self :: Unmarked ; }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash)] struct Marked < T , M > { value : T , _marker : marker :: PhantomData < M > , }}}
mkitem!{mkimpl!{impl < T , M > Mark for Marked < T , M > { type Unmarked = T ; fn mark (unmarked : Self :: Unmarked) -> Self { Marked { value : unmarked , _marker : marker :: PhantomData } } }}}
mkitem!{mkimpl!{impl < T , M > Unmark for Marked < T , M > { type Unmarked = T ; fn unmark (self) -> Self :: Unmarked { self . value } }}}
mkitem!{mkimpl!{impl < 'a , T , M > Unmark for & 'a Marked < T , M > { type Unmarked = & 'a T ; fn unmark (self) -> Self :: Unmarked { & self . value } }}}
mkitem!{mkimpl!{impl < 'a , T , M > Unmark for & 'a mut Marked < T , M > { type Unmarked = & 'a mut T ; fn unmark (self) -> Self :: Unmarked { & mut self . value } }}}
mkitem!{mkimpl!{impl < T : Mark > Mark for Vec < T > { type Unmarked = Vec < T :: Unmarked > ; fn mark (unmarked : Self :: Unmarked) -> Self { unmarked . into_iter () . map (T :: mark) . collect () } }}}
mkitem!{mkimpl!{impl < T : Unmark > Unmark for Vec < T > { type Unmarked = Vec < T :: Unmarked > ; fn unmark (self) -> Self :: Unmarked { self . into_iter () . map (T :: unmark) . collect () } }}}
mkitem!{macro_rules ! mark_noop { ($ ($ ty : ty) ,* $ (,) ?) => { $ (impl Mark for $ ty { type Unmarked = Self ; fn mark (unmarked : Self :: Unmarked) -> Self { unmarked } } impl Unmark for $ ty { type Unmarked = Self ; fn unmark (self) -> Self :: Unmarked { self } }) * } }}
mkitem!{mark_noop ! { () , bool , char , &'_ [u8] , &'_ str , String , u8 , usize , Delimiter , LitKind , Level , Spacing , }}
mkitem!{rpc_encode_decode ! (enum Delimiter { Parenthesis , Brace , Bracket , None , }) ;}
mkitem!{rpc_encode_decode ! (enum Level { Error , Warning , Note , Help , }) ;}
mkitem!{rpc_encode_decode ! (enum Spacing { Alone , Joint , }) ;}
mkitem!{mkenum!{# [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum LitKind { Byte , Char , Integer , Float , Str , StrRaw (u8) , ByteStr , ByteStrRaw (u8) , CStr , CStrRaw (u8) , ErrWithGuar , }}}
mkitem!{rpc_encode_decode ! (enum LitKind { Byte , Char , Integer , Float , Str , StrRaw (n) , ByteStr , ByteStrRaw (n) , CStr , CStrRaw (n) , ErrWithGuar , }) ;}
mkitem!{macro_rules ! mark_compound { (struct $ name : ident <$ ($ T : ident) ,+> { $ ($ field : ident) ,* $ (,) ? }) => { impl <$ ($ T : Mark) ,+> Mark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn mark (unmarked : Self :: Unmarked) -> Self { $ name { $ ($ field : Mark :: mark (unmarked .$ field)) ,* } } } impl <$ ($ T : Unmark) ,+> Unmark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn unmark (self) -> Self :: Unmarked { $ name { $ ($ field : Unmark :: unmark (self .$ field)) ,* } } } } ; (enum $ name : ident <$ ($ T : ident) ,+> { $ ($ variant : ident $ (($ field : ident)) ?) ,* $ (,) ? }) => { impl <$ ($ T : Mark) ,+> Mark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn mark (unmarked : Self :: Unmarked) -> Self { match unmarked { $ ($ name ::$ variant $ (($ field)) ? => { $ name ::$ variant $ ((Mark :: mark ($ field))) ? }) * } } } impl <$ ($ T : Unmark) ,+> Unmark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn unmark (self) -> Self :: Unmarked { match self { $ ($ name ::$ variant $ (($ field)) ? => { $ name ::$ variant $ ((Unmark :: unmark ($ field))) ? }) * } } } } }}
mkitem!{macro_rules ! compound_traits { ($ ($ t : tt) *) => { rpc_encode_decode ! ($ ($ t) *) ; mark_compound ! ($ ($ t) *) ; } ; }}
mkitem!{compound_traits ! (enum Bound < T > { Included (x) , Excluded (x) , Unbounded , }) ;}
mkitem!{compound_traits ! (enum Option < T > { Some (t) , None , }) ;}
mkitem!{compound_traits ! (enum Result < T , E > { Ok (t) , Err (e) , }) ;}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct DelimSpan < Span > { pub open : Span , pub close : Span , pub entire : Span , }}}
mkitem!{mkimpl!{impl < Span : Copy > DelimSpan < Span > { pub fn from_single (span : Span) -> Self { DelimSpan { open : span , close : span , entire : span } } }}}
mkitem!{compound_traits ! (struct DelimSpan < Span > { open , close , entire }) ;}
mkitem!{mkstruct!{# [derive (Clone)] pub struct Group < TokenStream , Span > { pub delimiter : Delimiter , pub stream : Option < TokenStream > , pub span : DelimSpan < Span > , }}}
mkitem!{compound_traits ! (struct Group < TokenStream , Span > { delimiter , stream , span }) ;}
mkitem!{mkstruct!{# [derive (Clone)] pub struct Punct < Span > { pub ch : u8 , pub joint : bool , pub span : Span , }}}
mkitem!{compound_traits ! (struct Punct < Span > { ch , joint , span }) ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , Eq , PartialEq)] pub struct Ident < Span , Symbol > { pub sym : Symbol , pub is_raw : bool , pub span : Span , }}}
mkitem!{compound_traits ! (struct Ident < Span , Symbol > { sym , is_raw , span }) ;}
mkitem!{mkstruct!{# [derive (Clone , Eq , PartialEq)] pub struct Literal < Span , Symbol > { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , pub span : Span , }}}
mkitem!{compound_traits ! (struct Literal < Sp , Sy > { kind , symbol , suffix , span }) ;}
mkitem!{mkenum!{# [derive (Clone)] pub enum TokenTree < TokenStream , Span , Symbol > { Group (Group < TokenStream , Span >) , Punct (Punct < Span >) , Ident (Ident < Span , Symbol >) , Literal (Literal < Span , Symbol >) , }}}
mkitem!{compound_traits ! (enum TokenTree < TokenStream , Span , Symbol > { Group (tt) , Punct (tt) , Ident (tt) , Literal (tt) , }) ;}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct Diagnostic < Span > { pub level : Level , pub message : String , pub spans : Vec < Span > , pub children : Vec < Diagnostic < Span > > , }}}
mkitem!{compound_traits ! (struct Diagnostic < Span > { level , message , spans , children }) ;}
mkitem!{mkstruct!{# [doc = " Globals provided alongside the initial inputs for a macro expansion."] # [doc = " Provides values such as spans which are used frequently to avoid RPC."] # [derive (Clone)] pub struct ExpnGlobals < Span > { pub def_site : Span , pub call_site : Span , pub mixed_site : Span , }}}
mkitem!{compound_traits ! (struct ExpnGlobals < Span > { def_site , call_site , mixed_site }) ;}
mkitem!{compound_traits ! (struct Range < T > { start , end }) ;}