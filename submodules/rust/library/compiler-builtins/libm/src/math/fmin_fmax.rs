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

macro_rules! fminf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf16 in module {}", module_path!());
    };
}

mkfn!{
    fminf16_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmin (x , y) }
}

macro_rules! fminf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf in module {}", module_path!());
    };
}

mkfn!{
    fminf_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf (x : f32 , y : f32) -> f32 { super :: generic :: fmin (x , y) }
}

macro_rules! fmin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin in module {}", module_path!());
    };
}

mkfn!{
    fmin_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmin (x : f64 , y : f64) -> f64 { super :: generic :: fmin (x , y) }
}

macro_rules! fminf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminf128 in module {}", module_path!());
    };
}

mkfn!{
    fminf128_introspect!();
    # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmin (x , y) }
}

macro_rules! fmaxf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf16 in module {}", module_path!());
    };
}

mkfn!{
    fmaxf16_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmax (x , y) }
}

macro_rules! fmaxf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf in module {}", module_path!());
    };
}

mkfn!{
    fmaxf_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf (x : f32 , y : f32) -> f32 { super :: generic :: fmax (x , y) }
}

macro_rules! fmax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax in module {}", module_path!());
    };
}

mkfn!{
    fmax_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmax (x : f64 , y : f64) -> f64 { super :: generic :: fmax (x , y) }
}

macro_rules! fmaxf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaxf128 in module {}", module_path!());
    };
}

mkfn!{
    fmaxf128_introspect!();
    # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `maxNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaxf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmax (x , y) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use crate :: support :: { Float , Hexf } ;}

macro_rules! fmin_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_test_introspect!();
    fn fmin_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ZERO) , (F :: ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: ZERO , F :: INFINITY , F :: ZERO) , (F :: ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ZERO , F :: INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ZERO) , (F :: ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: ONE , F :: INFINITY , F :: ONE) , (F :: ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ONE) , (F :: NEG_ONE , F :: ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: ZERO) , (F :: INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: INFINITY , F :: ONE , F :: ONE) , (F :: INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fmin({}, {})" , Hexf (x) , Hexf (y)) ; } assert_eq ! (f (F :: ZERO , F :: NEG_ZERO) , F :: ZERO) ; assert_eq ! (f (F :: NEG_ZERO , F :: ZERO) , F :: ZERO) ; assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmin_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmin_spec_tests_f16 () { fmin_spec_test :: < f16 > (fminf16) ; }
}

macro_rules! fmin_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f32_introspect!();
    # [test] fn fmin_spec_tests_f32 () { fmin_spec_test :: < f32 > (fminf) ; }
}

macro_rules! fmin_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f64_introspect!();
    # [test] fn fmin_spec_tests_f64 () { fmin_spec_test :: < f64 > (fmin) ; }
}

macro_rules! fmin_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmin_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmin_spec_tests_f128 () { fmin_spec_test :: < f128 > (fminf128) ; }
}

macro_rules! fmax_spec_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_test in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_test_introspect!();
    fn fmax_spec_test < F : Float > (f : impl Fn (F , F) -> F) { let cases = [(F :: ZERO , F :: ZERO , F :: ZERO) , (F :: ZERO , F :: ONE , F :: ONE) , (F :: ZERO , F :: NEG_ONE , F :: ZERO) , (F :: ZERO , F :: INFINITY , F :: INFINITY) , (F :: ZERO , F :: NEG_INFINITY , F :: ZERO) , (F :: ZERO , F :: NAN , F :: ZERO) , (F :: ZERO , F :: NEG_NAN , F :: ZERO) , (F :: NEG_ZERO , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: ONE , F :: ONE) , (F :: NEG_ZERO , F :: NEG_ONE , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: INFINITY , F :: INFINITY) , (F :: NEG_ZERO , F :: NEG_INFINITY , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NAN , F :: NEG_ZERO) , (F :: NEG_ZERO , F :: NEG_NAN , F :: NEG_ZERO) , (F :: ONE , F :: ZERO , F :: ONE) , (F :: ONE , F :: NEG_ZERO , F :: ONE) , (F :: ONE , F :: ONE , F :: ONE) , (F :: ONE , F :: NEG_ONE , F :: ONE) , (F :: ONE , F :: INFINITY , F :: INFINITY) , (F :: ONE , F :: NEG_INFINITY , F :: ONE) , (F :: ONE , F :: NAN , F :: ONE) , (F :: ONE , F :: NEG_NAN , F :: ONE) , (F :: NEG_ONE , F :: ZERO , F :: ZERO) , (F :: NEG_ONE , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_ONE , F :: ONE , F :: ONE) , (F :: NEG_ONE , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_ONE , F :: INFINITY , F :: INFINITY) , (F :: NEG_ONE , F :: NEG_INFINITY , F :: NEG_ONE) , (F :: NEG_ONE , F :: NAN , F :: NEG_ONE) , (F :: NEG_ONE , F :: NEG_NAN , F :: NEG_ONE) , (F :: INFINITY , F :: ZERO , F :: INFINITY) , (F :: INFINITY , F :: NEG_ZERO , F :: INFINITY) , (F :: INFINITY , F :: ONE , F :: INFINITY) , (F :: INFINITY , F :: NEG_ONE , F :: INFINITY) , (F :: INFINITY , F :: INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NEG_INFINITY , F :: INFINITY) , (F :: INFINITY , F :: NAN , F :: INFINITY) , (F :: INFINITY , F :: NEG_NAN , F :: INFINITY) , (F :: NEG_INFINITY , F :: ZERO , F :: ZERO) , (F :: NEG_INFINITY , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_INFINITY , F :: ONE , F :: ONE) , (F :: NEG_INFINITY , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_INFINITY , F :: INFINITY , F :: INFINITY) , (F :: NEG_INFINITY , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NAN , F :: NEG_INFINITY) , (F :: NEG_INFINITY , F :: NEG_NAN , F :: NEG_INFINITY) , (F :: NAN , F :: ZERO , F :: ZERO) , (F :: NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NAN , F :: ONE , F :: ONE) , (F :: NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NAN , F :: INFINITY , F :: INFINITY) , (F :: NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) , (F :: NAN , F :: NAN , F :: NAN) , (F :: NEG_NAN , F :: ZERO , F :: ZERO) , (F :: NEG_NAN , F :: NEG_ZERO , F :: NEG_ZERO) , (F :: NEG_NAN , F :: ONE , F :: ONE) , (F :: NEG_NAN , F :: NEG_ONE , F :: NEG_ONE) , (F :: NEG_NAN , F :: INFINITY , F :: INFINITY) , (F :: NEG_NAN , F :: NEG_INFINITY , F :: NEG_INFINITY) ,] ; for (x , y , res) in cases { let val = f (x , y) ; assert_biteq ! (val , res , "fmax({}, {})" , Hexf (x) , Hexf (y)) ; } assert_eq ! (f (F :: ZERO , F :: NEG_ZERO) , F :: ZERO) ; assert_eq ! (f (F :: NEG_ZERO , F :: ZERO) , F :: ZERO) ; assert ! (f (F :: NAN , F :: NEG_NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NAN) . is_nan ()) ; assert ! (f (F :: NEG_NAN , F :: NEG_NAN) . is_nan ()) ; }
}

macro_rules! fmax_spec_tests_f16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f16 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f16_introspect!();
    # [test] # [cfg (f16_enabled)] fn fmax_spec_tests_f16 () { fmax_spec_test :: < f16 > (fmaxf16) ; }
}

macro_rules! fmax_spec_tests_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f32 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f32_introspect!();
    # [test] fn fmax_spec_tests_f32 () { fmax_spec_test :: < f32 > (fmaxf) ; }
}

macro_rules! fmax_spec_tests_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f64 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f64_introspect!();
    # [test] fn fmax_spec_tests_f64 () { fmax_spec_test :: < f64 > (fmax) ; }
}

macro_rules! fmax_spec_tests_f128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax_spec_tests_f128 in module {}", module_path!());
    };
}

mkfn!{
    fmax_spec_tests_f128_introspect!();
    # [test] # [cfg (f128_enabled)] fn fmax_spec_tests_f128 () { fmax_spec_test :: < f128 > (fmaxf128) ; }
} 
            }}