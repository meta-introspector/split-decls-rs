mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
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
    #[doc = " Ensures only doc comment attributes are used"] fn check_attributes (attrs : Vec < Attribute >) -> Result < Vec < Attribute > > { let inner = | attr : Attribute | { if ! attr . path () . is_ident ("doc") { Err (Error :: new (attr . span () , "attributes not supported on queries")) } else if attr . style != AttrStyle :: Outer { Err (Error :: new (attr . span () , "attributes must be outer attributes (`///`), not inner attributes" ,)) } else { Ok (attr) } } ; attrs . into_iter () . map (inner) . collect () }
}
mkitem!{mkstruct!{#[doc = " A compiler query. `query ... { ... }`"] struct Query { doc_comments : Vec < Attribute > , modifiers : QueryModifiers , name : Ident , key : Pat , arg : Type , result : ReturnType , }}}
mkitem!{mkimpl!{impl Parse for Query { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut doc_comments = check_attributes (input . call (Attribute :: parse_outer) ?) ? ; input . parse :: < kw :: query > () ? ; let name : Ident = input . parse () ? ; let arg_content ; parenthesized ! (arg_content in input) ; let key = Pat :: parse_single (& arg_content) ? ; arg_content . parse :: < Token ! [:] > () ? ; let arg = arg_content . parse () ? ; let _ = arg_content . parse :: < Option < Token ! [,] > > () ? ; let result = input . parse () ? ; let content ; braced ! (content in input) ; let modifiers = parse_query_modifiers (& content) ? ; if doc_comments . is_empty () { doc_comments . push (doc_comment_from_desc (& modifiers . desc . 1) ?) ; } Ok (Query { doc_comments , modifiers , name , key , arg , result }) } }}}
mkitem!{mkstruct!{#[doc = " A type used to greedily parse another type until the input is empty."] struct List < T > (Vec < T >) ;}}
mkitem!{mkimpl!{impl < T : Parse > Parse for List < T > { fn parse (input : ParseStream < '_ >) -> Result < Self > { let mut list = Vec :: new () ; while ! input . is_empty () { list . push (input . parse () ?) ; } Ok (List (list)) } }}}
mkitem!{mkstruct!{struct QueryModifiers { #[doc = " The description of the query."] desc : (Option < Ident > , Punctuated < Expr , Token ! [,] >) , #[doc = " Use this type for the in-memory cache."] arena_cache : Option < Ident > , #[doc = " Cache the query to disk if the `Block` returns true."] cache : Option < (Option < Pat > , Block) > , #[doc = " A cycle error for this query aborting the compilation with a fatal error."] fatal_cycle : Option < Ident > , #[doc = " A cycle error results in a delay_bug call"] cycle_delay_bug : Option < Ident > , #[doc = " A cycle error results in a stashed cycle error that can be unstashed and canceled later"] cycle_stash : Option < Ident > , #[doc = " Don't hash the result, instead just mark a query red if it runs"] no_hash : Option < Ident > , #[doc = " Generate a dep node based on the dependencies of the query"] anon : Option < Ident > , #[doc = " Always evaluate the query, ignoring its dependencies"] eval_always : Option < Ident > , #[doc = " Whether the query has a call depth limit"] depth_limit : Option < Ident > , #[doc = " Use a separate query provider for local and extern crates"] separate_provide_extern : Option < Ident > , #[doc = " Generate a `feed` method to set the query's value from another query."] feedable : Option < Ident > , #[doc = " When this query is called via `tcx.ensure_ok()`, it returns"] #[doc = " `Result<(), ErrorGuaranteed>` instead of `()`. If the query needs to"] #[doc = " be executed, and that execution returns an error, the error result is"] #[doc = " returned to the caller."] #[doc = ""] #[doc = " If execution is skipped, a synthetic `Ok(())` is returned, on the"] #[doc = " assumption that a query with all-green inputs must have succeeded."] #[doc = ""] #[doc = " Can only be applied to queries with a return value of"] #[doc = " `Result<_, ErrorGuaranteed>`."] return_result_from_ensure_ok : Option < Ident > , }}}

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
    fn doc_comment_from_desc (list : & Punctuated < Expr , token :: Comma >) -> Result < Attribute > { use :: syn :: * ; let mut iter = list . iter () ; let format_str : String = match iter . next () { Some (& Expr :: Lit (ExprLit { lit : Lit :: Str (ref lit_str) , .. })) => { lit_str . value () . replace ("`{}`" , "{}") } _ => return Err (Error :: new (list . span () , "Expected a string literal")) , } ; let mut fmt_fragments = format_str . split ("{}") ; let mut doc_string = fmt_fragments . next () . unwrap () . to_string () ; iter . map (:: quote :: ToTokens :: to_token_stream) . zip (fmt_fragments) . for_each (| (tts , next_fmt_fragment) | { use :: core :: fmt :: Write ; write ! (& mut doc_string , " `{}` {}" , tts . to_string () . replace (" . " , ".") , next_fmt_fragment ,) . unwrap () ; } ,) ; let doc_string = format ! ("[query description - consider adding a doc-comment!] {doc_string}") ; Ok (parse_quote ! { #[doc = # doc_string] }) }
}

macro_rules! add_query_desc_cached_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_query_desc_cached_impl in module {}", module_path!());
    };
}

mkfn!{
    add_query_desc_cached_impl_introspect!();
    #[doc = " Add the impl of QueryDescription for the query to `impls` if one is requested"] fn add_query_desc_cached_impl (query : & Query , descs : & mut proc_macro2 :: TokenStream , cached : & mut proc_macro2 :: TokenStream ,) { let Query { name , key , modifiers , .. } = & query ; let ra_hint = quote ! { let crate :: query :: Providers { # name : _ , .. } ; } ; let cache = if let Some ((args , expr)) = modifiers . cache . as_ref () { let tcx = args . as_ref () . map (| t | quote ! { # t }) . unwrap_or_else (| | quote ! { _ }) ; quote ! { #[allow (unused_variables , unused_braces , rustc :: pass_by_value)] #[inline] pub fn # name <'tcx > (# tcx : TyCtxt <'tcx >, # key : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint # expr } } } else { quote ! { #[allow (rustc :: pass_by_value)] #[inline] pub fn # name <'tcx > (_ : TyCtxt <'tcx >, _ : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint false } } } ; let (tcx , desc) = & modifiers . desc ; let tcx = tcx . as_ref () . map_or_else (| | quote ! { _ } , | t | quote ! { # t }) ; let desc = quote ! { #[allow (unused_variables)] pub fn # name <'tcx > (tcx : TyCtxt <'tcx >, key : crate :: query :: queries ::# name :: Key <'tcx >) -> String { let (# tcx , # key) = (tcx , key) ; :: rustc_middle :: ty :: print :: with_no_trimmed_paths ! (format ! (# desc)) } } ; descs . extend (quote ! { # desc }) ; cached . extend (quote ! { # cache }) ; }
}

macro_rules! rustc_queries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_queries in module {}", module_path!());
    };
}

mkfn!{
    rustc_queries_introspect!();
    pub (super) fn rustc_queries (input : TokenStream) -> TokenStream { let queries = parse_macro_input ! (input as List < Query >) ; let mut query_stream = quote ! { } ; let mut query_description_stream = quote ! { } ; let mut query_cached_stream = quote ! { } ; let mut feedable_queries = quote ! { } ; let mut errors = quote ! { } ; macro_rules ! assert { ($ cond : expr , $ span : expr , $ ($ tt : tt) +) => { if !$ cond { errors . extend (Error :: new ($ span , format ! ($ ($ tt) +)) . into_compile_error () ,) ; } } } for query in queries . 0 { let Query { name , arg , modifiers , .. } = & query ; let result_full = & query . result ; let result = match query . result { ReturnType :: Default => quote ! { -> () } , _ => quote ! { # result_full } , } ; let mut attributes = Vec :: new () ; macro_rules ! passthrough { ($ ($ modifier : ident) ,+ $ (,) ?) => { $ (if let Some ($ modifier) = & modifiers .$ modifier { attributes . push (quote ! { (#$ modifier) }) ; } ;) + } } passthrough ! (fatal_cycle , arena_cache , cycle_delay_bug , cycle_stash , no_hash , anon , eval_always , depth_limit , separate_provide_extern , return_result_from_ensure_ok ,) ; if modifiers . cache . is_some () { attributes . push (quote ! { (cache) }) ; } if modifiers . cache . is_some () { attributes . push (quote ! { (cache) }) ; } let span = name . span () ; let attribute_stream = quote_spanned ! { span => # (# attributes) ,* } ; let doc_comments = & query . doc_comments ; query_stream . extend (quote ! { # (# doc_comments) * [# attribute_stream] fn # name (# arg) # result , }) ; if let Some (feedable) = & modifiers . feedable { assert ! (modifiers . anon . is_none () , feedable . span () , "Query {name} cannot be both `feedable` and `anon`.") ; assert ! (modifiers . eval_always . is_none () , feedable . span () , "Query {name} cannot be both `feedable` and `eval_always`.") ; feedable_queries . extend (quote ! { [# attribute_stream] fn # name (# arg) # result , }) ; } add_query_desc_cached_impl (& query , & mut query_description_stream , & mut query_cached_stream) ; } TokenStream :: from (quote ! { #[doc = " Higher-order macro that invokes the specified macro with a prepared"] #[doc = " list of all query signatures (including modifiers)."] #[doc = ""] #[doc = " This allows multiple simpler macros to each have access to the list"] #[doc = " of queries."] #[macro_export] macro_rules ! rustc_with_all_queries { ($ macro : ident ! $ ([$ ($ extra_fake_queries : tt) *]) ?) => { $ macro ! { $ ($ ($ extra_fake_queries) *) ? # query_stream } } } macro_rules ! rustc_feedable_queries { ($ macro : ident !) => { $ macro ! (# feedable_queries) ; } } pub mod descs { use super ::*; # query_description_stream } pub mod cached { use super ::*; # query_cached_stream } # errors }) }
}