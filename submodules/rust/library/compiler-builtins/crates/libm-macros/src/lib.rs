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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkmod!{enums, { 
                getname!(enums);
                getsrc!(enums);
                getpath!(enums);
                get_deps!(enums);
                get_crates!(enums);
                mkinclude!(enums);
                 
            }}
mkmod!{parse, { 
                getname!(parse);
                getsrc!(parse);
                getpath!(parse);
                get_deps!(parse);
                get_crates!(parse);
                mkinclude!(parse);
                 
            }}
mkmod!{shared, { 
                getname!(shared);
                getsrc!(shared);
                getpath!(shared);
                get_deps!(shared);
                get_crates!(shared);
                mkinclude!(shared);
                 
            }}
mkuse!{use parse :: { Invocation , StructuredInput } ;}
mkuse!{use proc_macro as pm ;}
mkuse!{use proc_macro2 :: { self as pm2 , Span } ;}
mkuse!{use quote :: { ToTokens , quote } ;}
mkuse!{pub (crate) use shared :: { ALL_OPERATIONS , FloatTy , MathOpInfo , Ty } ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: visit_mut :: VisitMut ;}
mkuse!{use syn :: { Ident , ItemEnum } ;}
mkitem!{const KNOWN_TYPES : & [& str] = & ["FTy" , "CFn" , "CArgs" , "CRet" , "RustFn" , "RustArgs" , "RustRet" , "public" ,] ;}

macro_rules! function_enum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function function_enum in module {}", module_path!());
    };
}

mkfn!{
    function_enum_introspect!();
    # [doc = " Populate an enum with a variant representing function. Names are in upper camel case."] # [doc = ""] # [doc = " Applied to an empty enum. Expects one attribute `#[function_enum(BaseName)]` that provides"] # [doc = " the name of the `BaseName` enum."] # [proc_macro_attribute] pub fn function_enum (attributes : pm :: TokenStream , tokens : pm :: TokenStream) -> pm :: TokenStream { let item = syn :: parse_macro_input ! (tokens as ItemEnum) ; let res = enums :: function_enum (item , attributes . into ()) ; match res { Ok (ts) => ts , Err (e) => e . into_compile_error () , } . into () }
}

macro_rules! base_name_enum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function base_name_enum in module {}", module_path!());
    };
}

mkfn!{
    base_name_enum_introspect!();
    # [doc = " Create an enum representing all possible base names, with names in upper camel case."] # [doc = ""] # [doc = " Applied to an empty enum."] # [proc_macro_attribute] pub fn base_name_enum (attributes : pm :: TokenStream , tokens : pm :: TokenStream) -> pm :: TokenStream { let item = syn :: parse_macro_input ! (tokens as ItemEnum) ; let res = enums :: base_name_enum (item , attributes . into ()) ; match res { Ok (ts) => ts , Err (e) => e . into_compile_error () , } . into () }
}

macro_rules! for_each_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function for_each_function in module {}", module_path!());
    };
}

mkfn!{
    for_each_function_introspect!();
    # [doc = " Do something for each function present in this crate."] # [doc = ""] # [doc = " Takes a callback macro and invokes it multiple times, once for each function that"] # [doc = " this crate exports. This makes it easy to create generic tests, benchmarks, or other checks"] # [doc = " and apply it to each symbol."] # [doc = ""] # [doc = " Additionally, the `extra` and `fn_extra` patterns can make use of magic identifiers:"] # [doc = ""] # [doc = " - `MACRO_FN_NAME`: gets replaced with the name of the function on that invocation."] # [doc = " - `MACRO_FN_NAME_NORMALIZED`: similar to the above, but removes sufixes so e.g. `sinf` becomes"] # [doc = "   `sin`, `cosf128` becomes `cos`, etc."] # [doc = ""] # [doc = " Invoke as:"] # [doc = ""] # [doc = " ```"] # [doc = " // Macro that is invoked once per function"] # [doc = " macro_rules! callback_macro {"] # [doc = "     ("] # [doc = "         // Name of that function"] # [doc = "         fn_name: $fn_name:ident,"] # [doc = "         // The basic float type for this function (e.g. `f32`, `f64`)"] # [doc = "         FTy: $FTy:ty,"] # [doc = "         // Function signature of the C version (e.g. `fn(f32, &mut f32) -> f32`)"] # [doc = "         CFn: $CFn:ty,"] # [doc = "         // A tuple representing the C version's arguments (e.g. `(f32, &mut f32)`)"] # [doc = "         CArgs: $CArgs:ty,"] # [doc = "         // The C version's return type (e.g. `f32`)"] # [doc = "         CRet: $CRet:ty,"] # [doc = "         // Function signature of the Rust version (e.g. `fn(f32) -> (f32, f32)`)"] # [doc = "         RustFn: $RustFn:ty,"] # [doc = "         // A tuple representing the Rust version's arguments (e.g. `(f32,)`)"] # [doc = "         RustArgs: $RustArgs:ty,"] # [doc = "         // The Rust version's return type (e.g. `(f32, f32)`)"] # [doc = "         RustRet: $RustRet:ty,"] # [doc = "         // True if this is part of `libm`'s public API"] # [doc = "         public: $public:expr,"] # [doc = "         // Attributes for the current function, if any"] # [doc = "         attrs: [$($attr:meta),*],"] # [doc = "         // Extra tokens passed directly (if any)"] # [doc = "         extra: [$extra:ident],"] # [doc = "         // Extra function-tokens passed directly (if any)"] # [doc = "         fn_extra: $fn_extra:expr,"] # [doc = "     ) => { };"] # [doc = " }"] # [doc = ""] # [doc = " // All fields except for `callback` are optional."] # [doc = " libm_macros::for_each_function! {"] # [doc = "     // The macro to invoke as a callback"] # [doc = "     callback: callback_macro,"] # [doc = "     // Which types to include either as a list (`[CFn, RustFn, RustArgs]`) or \"all\""] # [doc = "     emit_types: all,"] # [doc = "     // Functions to skip, i.e. `callback` shouldn't be called at all for these."] # [doc = "     skip: [sin, cos],"] # [doc = "     // Attributes passed as `attrs` for specific functions. For example, here the invocation"] # [doc = "     // with `sinf` and that with `cosf` will both get `meta1` and `meta2`, but no others will."] # [doc = "     //"] # [doc = "     // Note that `f16_enabled` and `f128_enabled` will always get emitted regardless of whether"] # [doc = "     // or not this is specified."] # [doc = "     attributes: ["] # [doc = "         #[meta1]"] # [doc = "         #[meta2]"] # [doc = "         [sinf, cosf],"] # [doc = "     ],"] # [doc = "     // Any tokens that should be passed directly to all invocations of the callback. This can"] # [doc = "     // be used to pass local variables or other things the macro needs access to."] # [doc = "     extra: [foo],"] # [doc = "     // Similar to `extra`, but allow providing a pattern for only specific functions. Uses"] # [doc = "     // a simplified match-like syntax."] # [doc = "     fn_extra: match MACRO_FN_NAME {"] # [doc = "         hypot | hypotf => |x| x.hypot(),"] # [doc = "         // `ALL_*` magic matchers also work to extract specific types"] # [doc = "         ALL_F64 => |x| x,"] # [doc = "         // The default pattern gets applied to everything that did not match"] # [doc = "         _ => |x| x,"] # [doc = "     },"] # [doc = " }"] # [doc = " ```"] # [proc_macro] pub fn for_each_function (tokens : pm :: TokenStream) -> pm :: TokenStream { let input = syn :: parse_macro_input ! (tokens as Invocation) ; let res = StructuredInput :: from_fields (input) . and_then (| mut s_in | validate (& mut s_in) . map (| fn_list | (s_in , fn_list))) . and_then (| (s_in , fn_list) | expand (s_in , & fn_list)) ; match res { Ok (ts) => ts . into () , Err (e) => e . into_compile_error () . into () , } }
}

macro_rules! validate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate in module {}", module_path!());
    };
}

mkfn!{
    validate_introspect!();
    # [doc = " Check for any input that is structurally correct but has other problems."] # [doc = ""] # [doc = " Returns the list of function names that we should expand for."] fn validate (input : & mut StructuredInput) -> syn :: Result < Vec < & 'static MathOpInfo > > { if let Some (map) = & mut input . fn_extra { for (name , ty) in [("ALL_F16" , FloatTy :: F16) , ("ALL_F32" , FloatTy :: F32) , ("ALL_F64" , FloatTy :: F64) , ("ALL_F128" , FloatTy :: F128) ,] { let Some (k) = map . keys () . find (| key | * key == name) else { continue ; } ; let key = k . clone () ; let val = map . remove (& key) . unwrap () ; for op in ALL_OPERATIONS . iter () . filter (| op | op . float_ty == ty) { map . insert (Ident :: new (op . name , key . span ()) , val . clone ()) ; } } } let attr_mentions = input . attributes . iter () . flat_map (| map_list | map_list . iter ()) . flat_map (| attr_map | attr_map . names . iter ()) ; let only_mentions = input . only . iter () . flat_map (| only_list | only_list . iter ()) ; let fn_extra_mentions = input . fn_extra . iter () . flat_map (| v | v . keys ()) . filter (| name | * name != "_") ; let all_mentioned_fns = input . skip . iter () . chain (only_mentions) . chain (attr_mentions) . chain (fn_extra_mentions) ; for mentioned in all_mentioned_fns { if ! ALL_OPERATIONS . iter () . any (| func | mentioned == func . name) { let e = syn :: Error :: new (mentioned . span () , format ! ("unrecognized function name `{mentioned}`") ,) ; return Err (e) ; } } if ! input . skip . is_empty () && input . only . is_some () { let e = syn :: Error :: new (input . only_span . unwrap () , "only one of `skip` or `only` may be specified" ,) ; return Err (e) ; } let mut fn_list = Vec :: new () ; for func in ALL_OPERATIONS . iter () { let fn_name = func . name ; if input . only . as_ref () . is_some_and (| only | ! only . iter () . any (| o | o == fn_name)) { continue ; } if input . skip . iter () . any (| s | s == fn_name) { continue ; } if input . skip_f16_f128 && (func . float_ty == FloatTy :: F16 || func . float_ty == FloatTy :: F128) { continue ; } fn_list . push (func) ; } let mut add_all_types = false ; for ty in & input . emit_types { let ty_name = ty . to_string () ; if ty_name == "all" { add_all_types = true ; continue ; } if ! KNOWN_TYPES . contains (& ty_name . as_str ()) { let e = syn :: Error :: new (ty_name . span () , format ! ("unrecognized type identifier `{ty_name}`") ,) ; return Err (e) ; } } if add_all_types { if input . emit_types . len () > 1 { let e = syn :: Error :: new (input . emit_types_span . unwrap () , "if `all` is specified, no other type identifiers may be given" ,) ; return Err (e) ; } input . emit_types . clear () ; for ty in KNOWN_TYPES { let ident = Ident :: new (ty , Span :: call_site ()) ; input . emit_types . push (ident) ; } } if let Some (map) = & input . fn_extra && ! map . keys () . any (| key | key == "_") { let mut fns_not_covered = Vec :: new () ; for func in & fn_list { if ! map . keys () . any (| key | key == func . name) { fns_not_covered . push (func) ; } } if ! fns_not_covered . is_empty () { let e = syn :: Error :: new (input . fn_extra_span . unwrap () , format ! ("`fn_extra`: no default `_` pattern specified and the following \
                     patterns are not covered: {fns_not_covered:#?}") ,) ; return Err (e) ; } } ; Ok (fn_list) }
}

macro_rules! expand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand in module {}", module_path!());
    };
}

mkfn!{
    expand_introspect!();
    # [doc = " Expand our structured macro input into invocations of the callback macro."] fn expand (input : StructuredInput , fn_list : & [& MathOpInfo]) -> syn :: Result < pm2 :: TokenStream > { let mut out = pm2 :: TokenStream :: new () ; let default_ident = Ident :: new ("_" , Span :: call_site ()) ; let callback = input . callback ; for func in fn_list { let fn_name = Ident :: new (func . name , Span :: call_site ()) ; let mut meta_fields = Vec :: new () ; if let Some (attrs) = & input . attributes { let meta_iter = attrs . iter () . filter (| map | map . names . contains (& fn_name)) . flat_map (| map | & map . meta) . map (| v | v . into_token_stream ()) ; meta_fields . extend (meta_iter) ; } if func . rust_sig . args . contains (& Ty :: F16) || func . rust_sig . returns . contains (& Ty :: F16) { let ts = quote ! { cfg (f16_enabled) } ; meta_fields . push (ts) ; } if func . rust_sig . args . contains (& Ty :: F128) || func . rust_sig . returns . contains (& Ty :: F128) { let ts = quote ! { cfg (f128_enabled) } ; meta_fields . push (ts) ; } let meta_field = quote ! { attrs : [# (# meta_fields) ,*] , } ; let extra_field = match input . extra . clone () { Some (mut extra) => { let mut v = MacroReplace :: new (func . name) ; v . visit_expr_mut (& mut extra) ; v . finish () ? ; quote ! { extra : # extra , } } None => pm2 :: TokenStream :: new () , } ; let fn_extra_field = match input . fn_extra { Some (ref map) => { let mut fn_extra = map . get (& fn_name) . or_else (| | map . get (& default_ident)) . unwrap () . clone () ; let mut v = MacroReplace :: new (func . name) ; v . visit_expr_mut (& mut fn_extra) ; v . finish () ? ; quote ! { fn_extra : # fn_extra , } } None => pm2 :: TokenStream :: new () , } ; let base_fty = func . float_ty ; let c_args = & func . c_sig . args ; let c_ret = & func . c_sig . returns ; let rust_args = & func . rust_sig . args ; let rust_ret = & func . rust_sig . returns ; let public = func . public ; let mut ty_fields = Vec :: new () ; for ty in & input . emit_types { let field = match ty . to_string () . as_str () { "FTy" => quote ! { FTy : # base_fty , } , "CFn" => quote ! { CFn : fn (# (# c_args) ,* ,) -> (# (# c_ret) ,*) , } , "CArgs" => quote ! { CArgs : (# (# c_args) ,* ,) , } , "CRet" => quote ! { CRet : (# (# c_ret) ,*) , } , "RustFn" => quote ! { RustFn : fn (# (# rust_args) ,* ,) -> (# (# rust_ret) ,*) , } , "RustArgs" => quote ! { RustArgs : (# (# rust_args) ,* ,) , } , "RustRet" => quote ! { RustRet : (# (# rust_ret) ,*) , } , "public" => quote ! { public : # public , } , _ => unreachable ! ("checked in validation") , } ; ty_fields . push (field) ; } let new = quote ! { # callback ! { fn_name : # fn_name , # (# ty_fields) * # meta_field # extra_field # fn_extra_field } } ; out . extend (new) ; } Ok (out) }
}
mkitem!{mkstruct!{# [doc = " Visitor to replace \"magic\" identifiers that we allow: `MACRO_FN_NAME` and"] # [doc = " `MACRO_FN_NAME_NORMALIZED`."] struct MacroReplace { fn_name : & 'static str , # [doc = " Remove the trailing `f` or `f128` to make"] norm_name : String , error : Option < syn :: Error > , }}}
mkitem!{mkimpl!{impl MacroReplace { fn new (name : & 'static str) -> Self { let norm_name = base_name (name) ; Self { fn_name : name , norm_name : norm_name . to_owned () , error : None , } } fn finish (self) -> syn :: Result < () > { match self . error { Some (e) => Err (e) , None => Ok (()) , } } fn visit_ident_inner (& mut self , i : & mut Ident) { let s = i . to_string () ; if ! s . starts_with ("MACRO") || self . error . is_some () { return ; } match s . as_str () { "MACRO_FN_NAME" => * i = Ident :: new (self . fn_name , i . span ()) , "MACRO_FN_NAME_NORMALIZED" => * i = Ident :: new (& self . norm_name , i . span ()) , _ => { self . error = Some (syn :: Error :: new (i . span () , format ! ("unrecognized meta expression `{s}`") ,)) ; } } } }}}
mkitem!{mkimpl!{impl VisitMut for MacroReplace { fn visit_ident_mut (& mut self , i : & mut Ident) { self . visit_ident_inner (i) ; syn :: visit_mut :: visit_ident_mut (self , i) ; } }}}

macro_rules! base_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function base_name in module {}", module_path!());
    };
}

mkfn!{
    base_name_introspect!();
    # [doc = " Return the unsuffixed version of a function name; e.g. `abs` and `absf` both return `abs`,"] # [doc = " `lgamma_r` and `lgammaf_r` both return `lgamma_r`."] fn base_name (name : & str) -> & str { let known_mappings = & [("erff" , "erf") , ("erf" , "erf") , ("lgammaf_r" , "lgamma_r") , ("modff" , "modf") , ("modf" , "modf") ,] ; match known_mappings . iter () . find (| known | known . 0 == name) { Some (found) => found . 1 , None => name . strip_suffix ("f") . or_else (| | name . strip_suffix ("f16")) . or_else (| | name . strip_suffix ("f128")) . unwrap_or (name) , } }
}
mkitem!{mkimpl!{impl ToTokens for Ty { fn to_tokens (& self , tokens : & mut pm2 :: TokenStream) { let ts = match self { Ty :: F16 => quote ! { f16 } , Ty :: F32 => quote ! { f32 } , Ty :: F64 => quote ! { f64 } , Ty :: F128 => quote ! { f128 } , Ty :: I32 => quote ! { i32 } , Ty :: CInt => quote ! { :: core :: ffi :: c_int } , Ty :: MutF16 => quote ! { &'a mut f16 } , Ty :: MutF32 => quote ! { &'a mut f32 } , Ty :: MutF64 => quote ! { &'a mut f64 } , Ty :: MutF128 => quote ! { &'a mut f128 } , Ty :: MutI32 => quote ! { &'a mut i32 } , Ty :: MutCInt => quote ! { &'a mut core :: ffi :: c_int } , } ; tokens . extend (ts) ; } }}}
mkitem!{mkimpl!{impl ToTokens for FloatTy { fn to_tokens (& self , tokens : & mut pm2 :: TokenStream) { let ts = match self { FloatTy :: F16 => quote ! { f16 } , FloatTy :: F32 => quote ! { f32 } , FloatTy :: F64 => quote ! { f64 } , FloatTy :: F128 => quote ! { f128 } , } ; tokens . extend (ts) ; } }}}