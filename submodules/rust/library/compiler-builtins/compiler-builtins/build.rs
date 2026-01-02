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
mkmod!{configure, { 
                getname!(configure);
                getsrc!(configure);
                getpath!(configure);
                get_deps!(configure);
                get_crates!(configure);
                mkinclude!(configure);
                 
            }}
mkuse!{use std :: env ;}
mkuse!{use configure :: { Target , configure_aliases } ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { println ! ("cargo::rerun-if-changed=build.rs") ; println ! ("cargo::rerun-if-changed=configure.rs") ; let target = Target :: from_env () ; let cwd = env :: current_dir () . unwrap () ; configure_check_cfg () ; configure_aliases (& target) ; configure_libm (& target) ; println ! ("cargo:compiler-rt={}" , cwd . join ("compiler-rt") . display ()) ; println ! ("cargo::rustc-check-cfg=cfg(kernel_user_helpers)") ; println ! ("cargo::rustc-check-cfg=cfg(feature, values(\"mem-unaligned\"))") ; if target . os == "emscripten" { return ; } if target . os == "openbsd" { println ! ("cargo:rustc-link-search=native=/usr/lib") ; println ! ("cargo:rustc-link-lib=compiler_rt") ; return ; } if (target . triple . contains ("wasm") && ! target . triple . contains ("wasi")) || (target . triple . contains ("sgx") && target . triple . contains ("fortanix")) || target . triple . contains ("-none") || target . triple . contains ("nvptx") || target . triple . contains ("uefi") || target . triple . contains ("xous") { println ! ("cargo:rustc-cfg=feature=\"mem\"") ; } if target . arch . contains ("x86_64") || target . arch . contains ("x86") || target . arch . contains ("aarch64") || target . arch . contains ("bpf") { println ! ("cargo:rustc-cfg=feature=\"mem-unaligned\"") ; } let llvm_target = target . triple . split ('-') . collect :: < Vec < _ > > () ; if ! cfg ! (feature = "mangled-names") && cfg ! (feature = "c") { if ! target . arch . contains ("nvptx") { # [cfg (feature = "c")] c :: compile (& llvm_target , & target) ; } } if llvm_target [0] == "armv4t" || llvm_target [0] == "armv5te" || target . triple == "arm-linux-androideabi" { println ! ("cargo:rustc-cfg=kernel_user_helpers") } }
}

macro_rules! configure_libm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function configure_libm in module {}", module_path!());
    };
}

mkfn!{
    configure_libm_introspect!();
    # [doc = " Run configuration for `libm` since it is included directly."] # [doc = ""] # [doc = " Much of this is copied from `libm/configure.rs`."] fn configure_libm (target : & Target) { println ! ("cargo:rustc-check-cfg=cfg(intrinsics_enabled)") ; println ! ("cargo:rustc-check-cfg=cfg(arch_enabled)") ; println ! ("cargo:rustc-check-cfg=cfg(optimizations_enabled)") ; println ! ("cargo:rustc-check-cfg=cfg(feature, values(\"unstable-public-internals\"))") ; println ! ("cargo:rustc-cfg=intrinsics_enabled") ; if ! cfg ! (feature = "no-asm") { println ! ("cargo:rustc-cfg=arch_enabled") ; } println ! ("cargo:rustc-check-cfg=cfg(optimizations_enabled)") ; if ! matches ! (target . opt_level . as_str () , "0" | "1") { println ! ("cargo:rustc-cfg=optimizations_enabled") ; } println ! ("cargo:rustc-env=CFG_CARGO_FEATURES={:?}" , target . cargo_features) ; println ! ("cargo:rustc-env=CFG_OPT_LEVEL={}" , target . opt_level) ; println ! ("cargo:rustc-env=CFG_TARGET_FEATURES={:?}" , target . features) ; println ! ("cargo:rustc-cfg=feature=\"unstable-intrinsics\"") ; }
}

macro_rules! configure_check_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function configure_check_cfg in module {}", module_path!());
    };
}

mkfn!{
    configure_check_cfg_introspect!();
    # [doc = " Emit directives for features we expect to support that aren't in `Cargo.toml`."] # [doc = ""] # [doc = " These are mostly cfg elements emitted by this `build.rs`."] fn configure_check_cfg () { const HAS_OPTIMIZED_C : & [& str] = & ["__ashldi3" , "__ashlsi3" , "__ashrdi3" , "__ashrsi3" , "__bswapsi2" , "__bswapdi2" , "__bswapti2" , "__divdi3" , "__divsi3" , "__divmoddi4" , "__divmodsi4" , "__divmodsi4" , "__divmodti4" , "__lshrdi3" , "__lshrsi3" , "__moddi3" , "__modsi3" , "__muldi3" , "__udivdi3" , "__udivmoddi4" , "__udivmodsi4" , "__udivsi3" , "__umoddi3" , "__umodsi3" ,] ; let mut aarch_atomic = Vec :: new () ; for aarch_op in ["cas" , "ldadd" , "ldclr" , "ldeor" , "ldset" , "swp"] { let op_sizes = if aarch_op == "cas" { [1 , 2 , 4 , 8 , 16] . as_slice () } else { [1 , 2 , 4 , 8] . as_slice () } ; for op_size in op_sizes { for ordering in ["relax" , "acq" , "rel" , "acq_rel"] { aarch_atomic . push (format ! ("__aarch64_{aarch_op}{op_size}_{ordering}")) ; } } } for fn_name in HAS_OPTIMIZED_C . iter () . copied () . chain (aarch_atomic . iter () . map (| s | s . as_str ())) { println ! ("cargo::rustc-check-cfg=cfg({fn_name}, values(\"optimized-c\"))" ,) ; } println ! ("cargo::rustc-check-cfg=cfg(target_feature, values(\"vis3\"))") ; println ! ("cargo::rustc-check-cfg=cfg(feature, values(\"checked\"))") ; println ! ("cargo::rustc-check-cfg=cfg(assert_no_panic)") ; }
}
mkmod!{c, { 
                getname!(c);
                getsrc!(c);
                getpath!(c);
                get_deps!(c);
                get_crates!(c);
                mkinclude!(c);
                mkuse!{use std :: collections :: { BTreeMap , HashSet } ;}
mkuse!{use std :: env ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use super :: Target ;}
mkitem!{mkstruct!{struct Sources { map : BTreeMap < & 'static str , & 'static str > , }}}
mkitem!{mkimpl!{impl Sources { fn new () -> Sources { Sources { map : BTreeMap :: new () , } } fn extend (& mut self , sources : & [(& 'static str , & 'static str)]) { for (symbol , src) in sources { if src . contains ("/") { self . map . insert (symbol , src) ; } else { if ! self . map . contains_key (symbol) { self . map . insert (symbol , src) ; } } } } fn remove (& mut self , symbols : & [& str]) { for symbol in symbols { self . map . remove (* symbol) . unwrap () ; } } }}}

macro_rules! compile_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compile in module {}", module_path!());
    };
}

mkfn!{
    compile_introspect!();
    # [doc = " Compile intrinsics from the compiler-rt C source code"] pub fn compile (llvm_target : & [& str] , target : & Target) { let mut consider_float_intrinsics = true ; let cfg = & mut cc :: Build :: new () ; if target . arch == "aarch64" { let cflags_key = String :: from ("CFLAGS_") + & (target . triple . replace ("-" , "_")) ; if let Ok (cflags_value) = env :: var (cflags_key) { if cflags_value . contains ("+nofp") || cflags_value . contains ("+nosimd") { consider_float_intrinsics = false ; } } } cfg . define ("COMPILER_RT_HAS_FLOAT16" , None) ; cfg . warnings (false) ; if target . env == "msvc" { cfg . flag ("/Zl") ; cfg . define ("__func__" , Some ("__FUNCTION__")) ; } else { cfg . flag ("-fno-builtin") ; cfg . flag ("-fvisibility=hidden") ; cfg . flag ("-ffreestanding") ; cfg . flag_if_supported ("-fomit-frame-pointer") ; cfg . define ("VISIBILITY_HIDDEN" , None) ; if let "aarch64" | "arm64ec" = target . arch . as_str () { } else { cfg . flag ("-Werror=implicit-function-declaration") ; } } if target . os == "uefi" { let out_dir = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; let include_dir = out_dir . join ("include") ; if ! include_dir . exists () { fs :: create_dir (& include_dir) . unwrap () ; } fs :: write (include_dir . join ("stdlib.h") , "#include <stddef.h>") . unwrap () ; cfg . flag (& format ! ("-I{}" , include_dir . to_str () . unwrap ())) ; } let mut sources = Sources :: new () ; sources . extend (& [("__absvdi2" , "absvdi2.c") , ("__absvsi2" , "absvsi2.c") , ("__addvdi3" , "addvdi3.c") , ("__addvsi3" , "addvsi3.c") , ("__cmpdi2" , "cmpdi2.c") , ("__int_util" , "int_util.c") , ("__mulvdi3" , "mulvdi3.c") , ("__mulvsi3" , "mulvsi3.c") , ("__negdi2" , "negdi2.c") , ("__negvdi2" , "negvdi2.c") , ("__negvsi2" , "negvsi2.c") , ("__paritydi2" , "paritydi2.c") , ("__paritysi2" , "paritysi2.c") , ("__popcountdi2" , "popcountdi2.c") , ("__popcountsi2" , "popcountsi2.c") , ("__subvdi3" , "subvdi3.c") , ("__subvsi3" , "subvsi3.c") , ("__ucmpdi2" , "ucmpdi2.c") ,]) ; if consider_float_intrinsics { sources . extend (& [("__divdc3" , "divdc3.c") , ("__divsc3" , "divsc3.c") , ("__muldc3" , "muldc3.c") , ("__mulsc3" , "mulsc3.c") , ("__negdf2" , "negdf2.c") , ("__negsf2" , "negsf2.c") ,]) ; } if target . vendor != "apple" || target . arch != "x86" { sources . extend (& [("__absvti2" , "absvti2.c") , ("__addvti3" , "addvti3.c") , ("__cmpti2" , "cmpti2.c") , ("__ffsti2" , "ffsti2.c") , ("__mulvti3" , "mulvti3.c") , ("__negti2" , "negti2.c") , ("__parityti2" , "parityti2.c") , ("__popcountti2" , "popcountti2.c") , ("__subvti3" , "subvti3.c") , ("__ucmpti2" , "ucmpti2.c") ,]) ; if consider_float_intrinsics { sources . extend (& [("__negvti2" , "negvti2.c")]) ; } } if target . vendor == "apple" { sources . extend (& [("atomic_flag_clear" , "atomic_flag_clear.c") , ("atomic_flag_clear_explicit" , "atomic_flag_clear_explicit.c") , ("atomic_flag_test_and_set" , "atomic_flag_test_and_set.c") , ("atomic_flag_test_and_set_explicit" , "atomic_flag_test_and_set_explicit.c" ,) , ("atomic_signal_fence" , "atomic_signal_fence.c") , ("atomic_thread_fence" , "atomic_thread_fence.c") ,]) ; } if target . env != "msvc" { if target . arch == "x86" { sources . extend (& [("__ashldi3" , "i386/ashldi3.S") , ("__ashrdi3" , "i386/ashrdi3.S") , ("__divdi3" , "i386/divdi3.S") , ("__lshrdi3" , "i386/lshrdi3.S") , ("__moddi3" , "i386/moddi3.S") , ("__muldi3" , "i386/muldi3.S") , ("__udivdi3" , "i386/udivdi3.S") , ("__umoddi3" , "i386/umoddi3.S") ,]) ; } } if target . arch == "arm" && target . vendor != "apple" && target . env != "msvc" { sources . extend (& [("__aeabi_div0" , "arm/aeabi_div0.c") , ("__aeabi_drsub" , "arm/aeabi_drsub.c") , ("__aeabi_frsub" , "arm/aeabi_frsub.c") , ("__bswapdi2" , "arm/bswapdi2.S") , ("__bswapsi2" , "arm/bswapsi2.S") , ("__divmodsi4" , "arm/divmodsi4.S") , ("__divsi3" , "arm/divsi3.S") , ("__modsi3" , "arm/modsi3.S") , ("__switch16" , "arm/switch16.S") , ("__switch32" , "arm/switch32.S") , ("__switch8" , "arm/switch8.S") , ("__switchu8" , "arm/switchu8.S") , ("__sync_synchronize" , "arm/sync_synchronize.S") , ("__udivmodsi4" , "arm/udivmodsi4.S") , ("__udivsi3" , "arm/udivsi3.S") , ("__umodsi3" , "arm/umodsi3.S") ,]) ; if target . os == "freebsd" { sources . extend (& [("__clear_cache" , "clear_cache.c")]) ; } if ! llvm_target [0] . starts_with ("thumbeb") && ! llvm_target [0] . starts_with ("armeb") { sources . extend (& [("__aeabi_cdcmp" , "arm/aeabi_cdcmp.S") , ("__aeabi_cdcmpeq_check_nan" , "arm/aeabi_cdcmpeq_check_nan.c") , ("__aeabi_cfcmp" , "arm/aeabi_cfcmp.S") , ("__aeabi_cfcmpeq_check_nan" , "arm/aeabi_cfcmpeq_check_nan.c") ,]) ; } } if llvm_target [0] == "armv7" { sources . extend (& [("__sync_fetch_and_add_4" , "arm/sync_fetch_and_add_4.S") , ("__sync_fetch_and_add_8" , "arm/sync_fetch_and_add_8.S") , ("__sync_fetch_and_and_4" , "arm/sync_fetch_and_and_4.S") , ("__sync_fetch_and_and_8" , "arm/sync_fetch_and_and_8.S") , ("__sync_fetch_and_max_4" , "arm/sync_fetch_and_max_4.S") , ("__sync_fetch_and_max_8" , "arm/sync_fetch_and_max_8.S") , ("__sync_fetch_and_min_4" , "arm/sync_fetch_and_min_4.S") , ("__sync_fetch_and_min_8" , "arm/sync_fetch_and_min_8.S") , ("__sync_fetch_and_nand_4" , "arm/sync_fetch_and_nand_4.S") , ("__sync_fetch_and_nand_8" , "arm/sync_fetch_and_nand_8.S") , ("__sync_fetch_and_or_4" , "arm/sync_fetch_and_or_4.S") , ("__sync_fetch_and_or_8" , "arm/sync_fetch_and_or_8.S") , ("__sync_fetch_and_sub_4" , "arm/sync_fetch_and_sub_4.S") , ("__sync_fetch_and_sub_8" , "arm/sync_fetch_and_sub_8.S") , ("__sync_fetch_and_umax_4" , "arm/sync_fetch_and_umax_4.S") , ("__sync_fetch_and_umax_8" , "arm/sync_fetch_and_umax_8.S") , ("__sync_fetch_and_umin_4" , "arm/sync_fetch_and_umin_4.S") , ("__sync_fetch_and_umin_8" , "arm/sync_fetch_and_umin_8.S") , ("__sync_fetch_and_xor_4" , "arm/sync_fetch_and_xor_4.S") , ("__sync_fetch_and_xor_8" , "arm/sync_fetch_and_xor_8.S") ,]) ; } if llvm_target . last () . unwrap () . ends_with ("eabihf") { if ! llvm_target [0] . starts_with ("thumbv7em") && ! llvm_target [0] . starts_with ("thumbv8m.main") { sources . extend (& [("__fixdfsivfp" , "arm/fixdfsivfp.S") , ("__fixunsdfsivfp" , "arm/fixunsdfsivfp.S") , ("__floatsidfvfp" , "arm/floatsidfvfp.S") , ("__floatunssidfvfp" , "arm/floatunssidfvfp.S") ,]) ; } sources . extend (& [("__fixsfsivfp" , "arm/fixsfsivfp.S") , ("__fixunssfsivfp" , "arm/fixunssfsivfp.S") , ("__floatsisfvfp" , "arm/floatsisfvfp.S") , ("__floatunssisfvfp" , "arm/floatunssisfvfp.S") , ("__floatunssisfvfp" , "arm/floatunssisfvfp.S") , ("__restore_vfp_d8_d15_regs" , "arm/restore_vfp_d8_d15_regs.S") , ("__save_vfp_d8_d15_regs" , "arm/save_vfp_d8_d15_regs.S") , ("__negdf2vfp" , "arm/negdf2vfp.S") , ("__negsf2vfp" , "arm/negsf2vfp.S") ,]) ; } if (target . arch == "aarch64" || target . arch == "arm64ec") && consider_float_intrinsics { sources . extend (& [("__fe_getround" , "fp_mode.c") , ("__fe_raise_inexact" , "fp_mode.c") ,]) ; if target . os != "windows" && target . os != "cygwin" { sources . extend (& [("__multc3" , "multc3.c")]) ; } } if target . arch == "mips" || target . arch == "riscv32" || target . arch == "riscv64" { sources . extend (& [("__bswapsi2" , "bswapsi2.c")]) ; } if target . arch == "mips64" { sources . extend (& [("__fe_getround" , "fp_mode.c")]) ; } if target . arch == "loongarch64" { sources . extend (& [("__fe_getround" , "fp_mode.c")]) ; } if llvm_target [0] == "thumbv6m" || llvm_target [0] == "thumbv8m.base" || target . os == "uefi" { let mut to_remove = Vec :: new () ; for (k , v) in sources . map . iter () { if v . ends_with (".S") { to_remove . push (* k) ; } } sources . remove (& to_remove) ; } if llvm_target [0] == "thumbv7m" || llvm_target [0] == "thumbv7em" { sources . remove (& ["__aeabi_cdcmp" , "__aeabi_cfcmp"]) ; } if target . os == "android" || target . os == "cygwin" { sources . extend (& [("__emutls_get_address" , "emutls.c")]) ; } if target . os == "android" { cfg . define ("LONG_BIT" , "(8 * sizeof(long))") ; } if target . env == "ohos" { sources . extend (& [("__emutls_get_address" , "emutls.c")]) ; } let link_against_prebuilt_rt = env :: var_os ("LLVM_COMPILER_RT_LIB") . is_some () ; let root = match env :: var_os ("RUST_COMPILER_RT_ROOT") { Some (s) => PathBuf :: from (s) , None if link_against_prebuilt_rt => PathBuf :: new () , None => { panic ! ("RUST_COMPILER_RT_ROOT is not set. You may need to run \
                    `ci/download-compiler-rt.sh`.") ; } } ; if ! link_against_prebuilt_rt && ! root . exists () { panic ! ("RUST_COMPILER_RT_ROOT={} does not exist" , root . display ()) ; } cfg . flag_if_supported (& format ! ("-ffile-prefix-map={}=." , root . display ())) ; let src_dir = root . join ("lib/builtins") ; if target . arch == "aarch64" && target . env != "msvc" && target . os != "uefi" { build_aarch64_out_of_line_atomics_libraries (& src_dir , cfg , link_against_prebuilt_rt) ; let cpu_model_src = if src_dir . join ("cpu_model.c") . exists () { "cpu_model.c" } else { "cpu_model/aarch64.c" } ; sources . extend (& [("__aarch64_have_lse_atomics" , cpu_model_src)]) ; } let mut added_sources = HashSet :: new () ; for (sym , src) in sources . map . iter () { let src = src_dir . join (src) ; if ! link_against_prebuilt_rt && added_sources . insert (src . clone ()) { cfg . file (& src) ; println ! ("cargo:rerun-if-changed={}" , src . display ()) ; } println ! ("cargo:rustc-cfg={}=\"optimized-c\"" , sym) ; } if link_against_prebuilt_rt { let rt_builtins_ext = PathBuf :: from (env :: var_os ("LLVM_COMPILER_RT_LIB") . unwrap ()) ; if ! rt_builtins_ext . exists () { panic ! ("LLVM_COMPILER_RT_LIB={} does not exist" , rt_builtins_ext . display ()) ; } if let Some (dir) = rt_builtins_ext . parent () { println ! ("cargo::rustc-link-search=native={}" , dir . display ()) ; } if let Some (lib) = rt_builtins_ext . file_name () { println ! ("cargo::rustc-link-lib=static:+verbatim={}" , lib . to_str () . unwrap ()) ; } } else { cfg . compile ("libcompiler-rt.a") ; } }
}

macro_rules! build_aarch64_out_of_line_atomics_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_aarch64_out_of_line_atomics_libraries in module {}", module_path!());
    };
}

mkfn!{
    build_aarch64_out_of_line_atomics_libraries_introspect!();
    fn build_aarch64_out_of_line_atomics_libraries (builtins_dir : & Path , cfg : & mut cc :: Build , link_against_prebuilt_rt : bool ,) { let out_dir = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; let outlined_atomics_file = builtins_dir . join ("aarch64") . join ("lse.S") ; if ! link_against_prebuilt_rt { println ! ("cargo:rerun-if-changed={}" , outlined_atomics_file . display ()) ; } cfg . include (& builtins_dir) ; for instruction_type in & ["cas" , "swp" , "ldadd" , "ldclr" , "ldeor" , "ldset"] { for size in & [1 , 2 , 4 , 8 , 16] { if * size == 16 && * instruction_type != "cas" { continue ; } for (model_number , model_name) in & [(1 , "relax") , (2 , "acq") , (3 , "rel") , (4 , "acq_rel")] { let sym = format ! ("__aarch64_{}{}_{}" , instruction_type , size , model_name) ; println ! ("cargo:rustc-cfg={}=\"optimized-c\"" , sym) ; if link_against_prebuilt_rt { continue ; } let path = out_dir . join (format ! ("lse_{}{}_{}.S" , instruction_type , size , model_name)) ; let mut file = File :: create (& path) . unwrap () ; writeln ! (file , "#define L_{}" , instruction_type) . unwrap () ; writeln ! (file , "#define SIZE {}" , size) . unwrap () ; writeln ! (file , "#define MODEL {}" , model_number) . unwrap () ; writeln ! (file , "#include \"{}\"" , outlined_atomics_file . canonicalize () . unwrap () . display ()) . unwrap () ; drop (file) ; cfg . file (path) ; } } } }
} 
            }}