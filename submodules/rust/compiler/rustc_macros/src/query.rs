mkuse!{use proc_macro :: TokenStream ;}
mkuse!{use quote :: { quote , quote_spanned } ;}
mkuse!{use syn :: parse :: { Parse , ParseStream , Result } ;}
mkuse!{use syn :: punctuated :: Punctuated ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: { AttrStyle , Attribute , Block , Error , Expr , Ident , Pat , ReturnType , Token , Type , braced , parenthesized , parse_macro_input , parse_quote , token , } ;}
mkmod!{kw, { 
                getname!(kw);
                getsrc!(kw);
                getpath!(kw);
                get_deps!(kw);
                get_crates!(kw);
                mkinclude!(kw);
                mkitem!{syn :: custom_keyword ! (query) ;} 
            }}

macro_rules! check_attributes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_attributes in module {}", module_path!());
    };
}

mkfn!{
    check_attributes_introspect!();
    # [doc = " Ensures only doc comment attributes are used"] fn check_attributes (attrs : Vec < Attribute >) -> Result < Vec < Attribute > > { let inner = | attr : Attribute | { if ! attr . path () . is_ident ("doc") { Err (Error :: new (attr . span () , "attributes not supported on queries")) } else if attr . style != AttrStyle :: Outer { Err (Error :: new (attr . span () , "attributes must be outer attributes (`///`), not inner attributes" ,)) } else { Ok (attr) } } ; attrs . into_iter () . map (inner) . collect () }
}
mkitem!{mkstruct!{# [doc = " A compiler query. `query ... { ... }`"] struct Query { doc_comments : Vec < Attribute > , modifiers : QueryModifiers , name : Ident , key : Pat , arg : Type , result : ReturnType , }}}
mkitem!{mkimpl!{impl Parse for Query { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut doc_comments = check_attributes (input . call (Attribute :: parse_outer) ?) ? ; input . parse :: < kw :: query > () ? ; let name : Ident = input . parse () ? ; let arg_content ; parenthesized ! (arg_content in input) ; let key = Pat :: parse_single (& arg_content) ? ; arg_content . parse :: < Token ! [:] > () ? ; let arg = arg_content . parse () ? ; let _ = arg_content . parse :: < Option < Token ! [,] > > () ? ; let result = input . parse () ? ; let content ; braced ! (content in input) ; let modifiers = parse_query_modifiers (& content) ? ; if doc_comments . is_empty () { doc_comments . push (doc_comment_from_desc (& modifiers . desc . 1) ?) ; } Ok (Query { doc_comments , modifiers , name , key , arg , result }) } }}}
mkitem!{mkstruct!{# [doc = " A type used to greedily parse another type until the input is empty."] struct List < T > (Vec < T >) ;}}
mkitem!{mkimpl!{impl < T : Parse > Parse for List < T > { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut list = Vec :: new () ; while ! input . is_empty () { list . push (input . parse () ?) ; } Ok (List (list)) } }}}
mkitem!{mkstruct!{struct QueryModifiers { # [doc = " The description of the query."] desc : (Option < Ident > , Punctuated < Expr , Token ! [,] >) , # [doc = " Use this type for the in-memory cache."] arena_cache : Option < Ident > , # [doc = " Cache the query to disk if the `Block` returns true."] cache : Option < (Option < Pat > , Block) > , # [doc = " A cycle error for this query aborting the compilation with a fatal error."] fatal_cycle : Option < Ident > , # [doc = " A cycle error results in a delay_bug call"] cycle_delay_bug : Option < Ident > , # [doc = " A cycle error results in a stashed cycle error that can be unstashed and canceled later"] cycle_stash : Option < Ident > , # [doc = " Don't hash the result, instead just mark a query red if it runs"] no_hash : Option < Ident > , # [doc = " Generate a dep node based on the dependencies of the query"] anon : Option < Ident > , # [doc = " Always evaluate the query, ignoring its dependencies"] eval_always : Option < Ident > , # [doc = " Whether the query has a call depth limit"] depth_limit : Option < Ident > , # [doc = " Use a separate query provider for local and extern crates"] separate_provide_extern : Option < Ident > , # [doc = " Generate a `feed` method to set the query's value from another query."] feedable : Option < Ident > , # [doc = " When this query is called via `tcx.ensure_ok()`, it returns"] # [doc = " `Result<(), ErrorGuaranteed>` instead of `()`. If the query needs to"] # [doc = " be executed, and that execution returns an error, the error result is"] # [doc = " returned to the caller."] # [doc = ""] # [doc = " If execution is skipped, a synthetic `Ok(())` is returned, on the"] # [doc = " assumption that a query with all-green inputs must have succeeded."] # [doc = ""] # [doc = " Can only be applied to queries with a return value of"] # [doc = " `Result<_, ErrorGuaranteed>`."] return_result_from_ensure_ok : Option < Ident > , }}}

macro_rules! parse_query_modifiers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_query_modifiers in module {}", module_path!());
    };
}

mkfn!{
    parse_query_modifiers_introspect!();
    fn parse_query_modifiers (input : ParseStream < '_ >) -> Result < QueryModifiers > { let mut arena_cache = None ; let mut cache = None ; let mut desc = None ; let mut fatal_cycle = None ; let mut cycle_delay_bug = None ; let mut cycle_stash = None ; let mut no_hash = None ; let mut anon = None ; let mut eval_always = None ; let mut depth_limit = None ; let mut separate_provide_extern = None ; let mut feedable = None ; let mut return_result_from_ensure_ok = None ; while ! input . is_empty () { let modifier : Ident = input . parse () ? ; macro_rules ! try_insert { ($ name : ident = $ expr : expr) => { if $ name . is_some () { return Err (Error :: new (modifier . span () , "duplicate modifier")) ; } $ name = Some ($ expr) ; } ; } if modifier == "desc" { let attr_content ; braced ! (attr_content in input) ; let tcx = if attr_content . peek (Token ! [|]) { attr_content . parse :: < Token ! [|] > () ? ; let tcx = attr_content . parse () ? ; attr_content . parse :: < Token ! [|] > () ? ; Some (tcx) } else { None } ; let list = attr_content . parse_terminated (Expr :: parse , Token ! [,]) ? ; try_insert ! (desc = (tcx , list)) ; } else if modifier == "cache_on_disk_if" { let args = if input . peek (token :: Paren) { let args ; parenthesized ! (args in input) ; let tcx = Pat :: parse_single (& args) ? ; Some (tcx) } else { None } ; let block = input . parse () ? ; try_insert ! (cache = (args , block)) ; } else if modifier == "arena_cache" { try_insert ! (arena_cache = modifier) ; } else if modifier == "fatal_cycle" { try_insert ! (fatal_cycle = modifier) ; } else if modifier == "cycle_delay_bug" { try_insert ! (cycle_delay_bug = modifier) ; } else if modifier == "cycle_stash" { try_insert ! (cycle_stash = modifier) ; } else if modifier == "no_hash" { try_insert ! (no_hash = modifier) ; } else if modifier == "anon" { try_insert ! (anon = modifier) ; } else if modifier == "eval_always" { try_insert ! (eval_always = modifier) ; } else if modifier == "depth_limit" { try_insert ! (depth_limit = modifier) ; } else if modifier == "separate_provide_extern" { try_insert ! (separate_provide_extern = modifier) ; } else if modifier == "feedable" { try_insert ! (feedable = modifier) ; } else if modifier == "return_result_from_ensure_ok" { try_insert ! (return_result_from_ensure_ok = modifier) ; } else { return Err (Error :: new (modifier . span () , "unknown query modifier")) ; } } let Some (desc) = desc else { return Err (input . error ("no description provided")) ; } ; Ok (QueryModifiers { arena_cache , cache , desc , fatal_cycle , cycle_delay_bug , cycle_stash , no_hash , anon , eval_always , depth_limit , separate_provide_extern , feedable , return_result_from_ensure_ok , }) }
}

macro_rules! doc_comment_from_desc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function doc_comment_from_desc in module {}", module_path!());
    };
}

mkfn!{
    doc_comment_from_desc_introspect!();
    fn doc_comment_from_desc (list : & Punctuated < Expr , token :: Comma >) -> Result < Attribute > { use :: syn :: * ; let mut iter = list . iter () ; let format_str : String = match iter . next () { Some (& Expr :: Lit (ExprLit { lit : Lit :: Str (ref lit_str) , .. })) => { lit_str . value () . replace ("`{}`" , "{}") } _ => return Err (Error :: new (list . span () , "Expected a string literal")) , } ; let mut fmt_fragments = format_str . split ("{}") ; let mut doc_string = fmt_fragments . next () . unwrap () . to_string () ; iter . map (:: quote :: ToTokens :: to_token_stream) . zip (fmt_fragments) . for_each (| (tts , next_fmt_fragment) | { use :: core :: fmt :: Write ; write ! (& mut doc_string , " `{}` {}" , tts . to_string () . replace (" . " , ".") , next_fmt_fragment ,) . unwrap () ; } ,) ; let doc_string = format ! ("[query description - consider adding a doc-comment!] {doc_string}") ; Ok (parse_quote ! { # [doc = # doc_string] }) }
}

macro_rules! add_query_desc_cached_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_query_desc_cached_impl in module {}", module_path!());
    };
}

mkfn!{
    add_query_desc_cached_impl_introspect!();
    # [doc = " Add the impl of QueryDescription for the query to `impls` if one is requested"] fn add_query_desc_cached_impl (query : & Query , descs : & mut proc_macro2 :: TokenStream , cached : & mut proc_macro2 :: TokenStream ,) { let Query { name , key , modifiers , .. } = & query ; let ra_hint = quote ! { let crate :: query :: Providers { # name : _ , .. } ; } ; let cache = if let Some ((args , expr)) = modifiers . cache . as_ref () { let tcx = args . as_ref () . map (| t | quote ! { # t }) . unwrap_or_else (| | quote ! { _ }) ; quote ! { # [allow (unused_variables , unused_braces , rustc :: pass_by_value)] # [inline] pub fn # name <'tcx > (# tcx : TyCtxt <'tcx >, # key : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint # expr } } } else { quote ! { # [allow (rustc :: pass_by_value)] # [inline] pub fn # name <'tcx > (_ : TyCtxt <'tcx >, _ : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint false } } } ; let (tcx , desc) = & modifiers . desc ; let tcx = tcx . as_ref () . map_or_else (| | quote ! { _ } , | t | quote ! { # t }) ; let desc = quote ! { # [allow (unused_variables)] pub fn # name <'tcx > (tcx : TyCtxt <'tcx >, key : crate :: query :: queries ::# name :: Key <'tcx >) -> String { let (# tcx , # key) = (tcx , key) ; :: rustc_middle :: ty :: print :: with_no_trimmed_paths ! (format ! (# desc)) } } ; descs . extend (quote ! { # desc }) ; cached . extend (quote ! { # cache }) ; }
}

macro_rules! rustc_queries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_queries in module {}", module_path!());
    };
}

mkfn!{
    rustc_queries_introspect!();
    pub (super) fn rustc_queries (input : TokenStream) -> TokenStream { let queries = parse_macro_input ! (input as List < Query >) ; let mut query_stream = quote ! { } ; let mut query_description_stream = quote ! { } ; let mut query_cached_stream = quote ! { } ; let mut feedable_queries = quote ! { } ; let mut errors = quote ! { } ; macro_rules ! assert { ($ cond : expr , $ span : expr , $ ($ tt : tt) +) => { if !$ cond { errors . extend (Error :: new ($ span , format ! ($ ($ tt) +)) . into_compile_error () ,) ; } } } for query in queries . 0 { let Query { name , arg , modifiers , .. } = & query ; let result_full = & query . result ; let result = match query . result { ReturnType :: Default => quote ! { -> () } , _ => quote ! { # result_full } , } ; let mut attributes = Vec :: new () ; macro_rules ! passthrough { ($ ($ modifier : ident) ,+ $ (,) ?) => { $ (if let Some ($ modifier) = & modifiers .$ modifier { attributes . push (quote ! { (#$ modifier) }) ; } ;) + } } passthrough ! (fatal_cycle , arena_cache , cycle_delay_bug , cycle_stash , no_hash , anon , eval_always , depth_limit , separate_provide_extern , return_result_from_ensure_ok ,) ; if modifiers . cache . is_some () { attributes . push (quote ! { (cache) }) ; } if modifiers . cache . is_some () { attributes . push (quote ! { (cache) }) ; } let span = name . span () ; let attribute_stream = quote_spanned ! { span => # (# attributes) ,* } ; let doc_comments = & query . doc_comments ; query_stream . extend (quote ! { # (# doc_comments) * [# attribute_stream] fn # name (# arg) # result , }) ; if let Some (feedable) = & modifiers . feedable { assert ! (modifiers . anon . is_none () , feedable . span () , "Query {name} cannot be both `feedable` and `anon`.") ; assert ! (modifiers . eval_always . is_none () , feedable . span () , "Query {name} cannot be both `feedable` and `eval_always`.") ; feedable_queries . extend (quote ! { [# attribute_stream] fn # name (# arg) # result , }) ; } add_query_desc_cached_impl (& query , & mut query_description_stream , & mut query_cached_stream) ; } TokenStream :: from (quote ! { # [doc = " Higher-order macro that invokes the specified macro with a prepared"] # [doc = " list of all query signatures (including modifiers)."] # [doc = ""] # [doc = " This allows multiple simpler macros to each have access to the list"] # [doc = " of queries."] # [macro_export] macro_rules ! rustc_with_all_queries { ($ macro : ident ! $ ([$ ($ extra_fake_queries : tt) *]) ?) => { $ macro ! { $ ($ ($ extra_fake_queries) *) ? # query_stream } } } macro_rules ! rustc_feedable_queries { ($ macro : ident !) => { $ macro ! (# feedable_queries) ; } } pub mod descs { use super ::*; # query_description_stream } pub mod cached { use super ::*; # query_cached_stream } # errors }) }
}