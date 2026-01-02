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
mkuse!{use super :: { fabs , get_high_word , scalbn , sqrt , with_set_high_word , with_set_low_word } ;}
mkitem!{const BP : [f64 ; 2] = [1.0 , 1.5] ;}
mkitem!{const DP_H : [f64 ; 2] = [0.0 , 5.84962487220764160156e-01] ;}
mkitem!{const DP_L : [f64 ; 2] = [0.0 , 1.35003920212974897128e-08] ;}
mkitem!{const TWO53 : f64 = 9007199254740992.0 ;}
mkitem!{const HUGE : f64 = 1.0e300 ;}
mkitem!{const TINY : f64 = 1.0e-300 ;}
mkitem!{const L1 : f64 = 5.99999999999994648725e-01 ;}
mkitem!{const L2 : f64 = 4.28571428578550184252e-01 ;}
mkitem!{const L3 : f64 = 3.33333329818377432918e-01 ;}
mkitem!{const L4 : f64 = 2.72728123808534006489e-01 ;}
mkitem!{const L5 : f64 = 2.30660745775561754067e-01 ;}
mkitem!{const L6 : f64 = 2.06975017800338417784e-01 ;}
mkitem!{const P1 : f64 = 1.66666666666666019037e-01 ;}
mkitem!{const P2 : f64 = - 2.77777777770155933842e-03 ;}
mkitem!{const P3 : f64 = 6.61375632143793436117e-05 ;}
mkitem!{const P4 : f64 = - 1.65339022054652515390e-06 ;}
mkitem!{const P5 : f64 = 4.13813679705723846039e-08 ;}
mkitem!{const LG2 : f64 = 6.93147180559945286227e-01 ;}
mkitem!{const LG2_H : f64 = 6.93147182464599609375e-01 ;}
mkitem!{const LG2_L : f64 = - 1.90465429995776804525e-09 ;}
mkitem!{const OVT : f64 = 8.0085662595372944372e-017 ;}
mkitem!{const CP : f64 = 9.61796693925975554329e-01 ;}
mkitem!{const CP_H : f64 = 9.61796700954437255859e-01 ;}
mkitem!{const CP_L : f64 = - 7.02846165095275826516e-09 ;}
mkitem!{const IVLN2 : f64 = 1.44269504088896338700e+00 ;}
mkitem!{const IVLN2_H : f64 = 1.44269502162933349609e+00 ;}
mkitem!{const IVLN2_L : f64 = 1.92596299112661746887e-08 ;}

macro_rules! pow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pow in module {}", module_path!());
    };
}

mkfn!{
    pow_introspect!();
    # [doc = " Returns `x` to the power of `y` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn pow (x : f64 , y : f64) -> f64 { let t1 : f64 ; let t2 : f64 ; let (hx , lx) : (i32 , u32) = ((x . to_bits () >> 32) as i32 , x . to_bits () as u32) ; let (hy , ly) : (i32 , u32) = ((y . to_bits () >> 32) as i32 , y . to_bits () as u32) ; let mut ix : i32 = hx & 0x7fffffff_i32 ; let iy : i32 = hy & 0x7fffffff_i32 ; if ((iy as u32) | ly) == 0 { return 1.0 ; } if hx == 0x3ff00000 && lx == 0 { return 1.0 ; } if ix > 0x7ff00000 || (ix == 0x7ff00000 && lx != 0) || iy > 0x7ff00000 || (iy == 0x7ff00000 && ly != 0) { return x + y ; } let mut yisint : i32 = 0 ; let mut k : i32 ; let mut j : i32 ; if hx < 0 { if iy >= 0x43400000 { yisint = 2 ; } else if iy >= 0x3ff00000 { k = (iy >> 20) - 0x3ff ; if k > 20 { j = (ly >> (52 - k)) as i32 ; if (j << (52 - k)) == (ly as i32) { yisint = 2 - (j & 1) ; } } else if ly == 0 { j = iy >> (20 - k) ; if (j << (20 - k)) == iy { yisint = 2 - (j & 1) ; } } } } if ly == 0 { if iy == 0x7ff00000 { return if ((ix - 0x3ff00000) | (lx as i32)) == 0 { 1.0 } else if ix >= 0x3ff00000 { if hy >= 0 { y } else { 0.0 } } else { if hy >= 0 { 0.0 } else { - y } } ; } if iy == 0x3ff00000 { return if hy >= 0 { x } else { 1.0 / x } ; } if hy == 0x40000000 { return x * x ; } if hy == 0x3fe00000 { if hx >= 0 { return sqrt (x) ; } } } let mut ax : f64 = fabs (x) ; if lx == 0 { if ix == 0x7ff00000 || ix == 0 || ix == 0x3ff00000 { let mut z : f64 = ax ; if hy < 0 { z = 1.0 / z ; } if hx < 0 { if ((ix - 0x3ff00000) | yisint) == 0 { z = (z - z) / (z - z) ; } else if yisint == 1 { z = - z ; } } return z ; } } let mut s : f64 = 1.0 ; if hx < 0 { if yisint == 0 { return (x - x) / (x - x) ; } if yisint == 1 { s = - 1.0 ; } } if iy > 0x41e00000 { if iy > 0x43f00000 { if ix <= 0x3fefffff { return if hy < 0 { HUGE * HUGE } else { TINY * TINY } ; } if ix >= 0x3ff00000 { return if hy > 0 { HUGE * HUGE } else { TINY * TINY } ; } } if ix < 0x3fefffff { return if hy < 0 { s * HUGE * HUGE } else { s * TINY * TINY } ; } if ix > 0x3ff00000 { return if hy > 0 { s * HUGE * HUGE } else { s * TINY * TINY } ; } let t : f64 = ax - 1.0 ; let w : f64 = (t * t) * (0.5 - t * (0.3333333333333333333333 - t * 0.25)) ; let u : f64 = IVLN2_H * t ; let v : f64 = t * IVLN2_L - w * IVLN2 ; t1 = with_set_low_word (u + v , 0) ; t2 = v - (t1 - u) ; } else { let mut n : i32 = 0 ; if ix < 0x00100000 { ax *= TWO53 ; n -= 53 ; ix = get_high_word (ax) as i32 ; } n += (ix >> 20) - 0x3ff ; j = ix & 0x000fffff ; let k : i32 ; ix = j | 0x3ff00000 ; if j <= 0x3988E { k = 0 ; } else if j < 0xBB67A { k = 1 ; } else { k = 0 ; n += 1 ; ix -= 0x00100000 ; } ax = with_set_high_word (ax , ix as u32) ; let u : f64 = ax - i ! (BP , k as usize) ; let v : f64 = 1.0 / (ax + i ! (BP , k as usize)) ; let ss : f64 = u * v ; let s_h = with_set_low_word (ss , 0) ; let t_h : f64 = with_set_high_word (0.0 , ((ix as u32 >> 1) | 0x20000000) + 0x00080000 + ((k as u32) << 18) ,) ; let t_l : f64 = ax - (t_h - i ! (BP , k as usize)) ; let s_l : f64 = v * ((u - s_h * t_h) - s_h * t_l) ; let s2 : f64 = ss * ss ; let mut r : f64 = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6))))) ; r += s_l * (s_h + ss) ; let s2 : f64 = s_h * s_h ; let t_h : f64 = with_set_low_word (3.0 + s2 + r , 0) ; let t_l : f64 = r - ((t_h - 3.0) - s2) ; let u : f64 = s_h * t_h ; let v : f64 = s_l * t_h + t_l * ss ; let p_h : f64 = with_set_low_word (u + v , 0) ; let p_l = v - (p_h - u) ; let z_h : f64 = CP_H * p_h ; let z_l : f64 = CP_L * p_h + p_l * CP + i ! (DP_L , k as usize) ; let t : f64 = n as f64 ; t1 = with_set_low_word (((z_h + z_l) + i ! (DP_H , k as usize)) + t , 0) ; t2 = z_l - (((t1 - t) - i ! (DP_H , k as usize)) - z_h) ; } let y1 : f64 = with_set_low_word (y , 0) ; let p_l : f64 = (y - y1) * t1 + y * t2 ; let mut p_h : f64 = y1 * t1 ; let z : f64 = p_l + p_h ; let mut j : i32 = (z . to_bits () >> 32) as i32 ; let i : i32 = z . to_bits () as i32 ; if j >= 0x40900000 { if (j - 0x40900000) | i != 0 { return s * HUGE * HUGE ; } if p_l + OVT > z - p_h { return s * HUGE * HUGE ; } } else if (j & 0x7fffffff) >= 0x4090cc00 { if (((j as u32) - 0xc090cc00) | (i as u32)) != 0 { return s * TINY * TINY ; } if p_l <= z - p_h { return s * TINY * TINY ; } } let i : i32 = j & 0x7fffffff_i32 ; k = (i >> 20) - 0x3ff ; let mut n : i32 = 0 ; if i > 0x3fe00000 { n = j + (0x00100000 >> (k + 1)) ; k = ((n & 0x7fffffff) >> 20) - 0x3ff ; let t : f64 = with_set_high_word (0.0 , (n & ! (0x000fffff >> k)) as u32) ; n = ((n & 0x000fffff) | 0x00100000) >> (20 - k) ; if j < 0 { n = - n ; } p_h -= t ; } let t : f64 = with_set_low_word (p_l + p_h , 0) ; let u : f64 = t * LG2_H ; let v : f64 = (p_l - (t - p_h)) * LG2 + t * LG2_L ; let mut z : f64 = u + v ; let w : f64 = v - (z - u) ; let t : f64 = z * z ; let t1 : f64 = z - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5)))) ; let r : f64 = (z * t1) / (t1 - 2.0) - (w + z * w) ; z = 1.0 - (r - z) ; j = get_high_word (z) as i32 ; j += n << 20 ; if (j >> 20) <= 0 { z = scalbn (z , n) ; } else { z = with_set_high_word (z , j as u32) ; } s * z }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkitem!{extern crate core ;}
mkuse!{use self :: core :: f64 :: consts :: { E , PI } ;}
mkuse!{use super :: pow ;}
mkitem!{const POS_ZERO : & [f64] = & [0.0] ;}
mkitem!{const NEG_ZERO : & [f64] = & [- 0.0] ;}
mkitem!{const POS_ONE : & [f64] = & [1.0] ;}
mkitem!{const NEG_ONE : & [f64] = & [- 1.0] ;}
mkitem!{const POS_FLOATS : & [f64] = & [99.0 / 70.0 , E , PI] ;}
mkitem!{const NEG_FLOATS : & [f64] = & [- 99.0 / 70.0 , - E , - PI] ;}
mkitem!{const POS_SMALL_FLOATS : & [f64] = & [(1.0 / 2.0) , f64 :: MIN_POSITIVE , f64 :: EPSILON] ;}
mkitem!{const NEG_SMALL_FLOATS : & [f64] = & [- (1.0 / 2.0) , - f64 :: MIN_POSITIVE , - f64 :: EPSILON] ;}
mkitem!{const POS_EVENS : & [f64] = & [2.0 , 6.0 , 8.0 , 10.0 , 22.0 , 100.0 , f64 :: MAX] ;}
mkitem!{const NEG_EVENS : & [f64] = & [f64 :: MIN , - 100.0 , - 22.0 , - 10.0 , - 8.0 , - 6.0 , - 2.0] ;}
mkitem!{const POS_ODDS : & [f64] = & [3.0 , 7.0] ;}
mkitem!{const NEG_ODDS : & [f64] = & [- 7.0 , - 3.0] ;}
mkitem!{const NANS : & [f64] = & [f64 :: NAN] ;}
mkitem!{const POS_INF : & [f64] = & [f64 :: INFINITY] ;}
mkitem!{const NEG_INF : & [f64] = & [f64 :: NEG_INFINITY] ;}
mkitem!{const ALL : & [& [f64]] = & [POS_ZERO , NEG_ZERO , NANS , NEG_SMALL_FLOATS , POS_SMALL_FLOATS , NEG_FLOATS , POS_FLOATS , NEG_EVENS , POS_EVENS , NEG_ODDS , POS_ODDS , NEG_INF , POS_INF , NEG_ONE , POS_ONE ,] ;}
mkitem!{const POS : & [& [f64]] = & [POS_ZERO , POS_ODDS , POS_ONE , POS_FLOATS , POS_EVENS , POS_INF] ;}
mkitem!{const NEG : & [& [f64]] = & [NEG_ZERO , NEG_ODDS , NEG_ONE , NEG_FLOATS , NEG_EVENS , NEG_INF] ;}

macro_rules! pow_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pow_test in module {}", module_path!());
    };
}

mkfn!{
    pow_test_introspect!();
    fn pow_test (base : f64 , exponent : f64 , expected : f64) { let res = pow (base , exponent) ; assert ! (if expected . is_nan () { res . is_nan () } else { pow (base , exponent) == expected } , "{base} ** {exponent} was {res} instead of {expected}" ,) ; }
}

macro_rules! test_sets_as_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_sets_as_base in module {}", module_path!());
    };
}

mkfn!{
    test_sets_as_base_introspect!();
    fn test_sets_as_base (sets : & [& [f64]] , exponent : f64 , expected : f64) { sets . iter () . for_each (| s | s . iter () . for_each (| val | pow_test (* val , exponent , expected))) ; }
}

macro_rules! test_sets_as_exponent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_sets_as_exponent in module {}", module_path!());
    };
}

mkfn!{
    test_sets_as_exponent_introspect!();
    fn test_sets_as_exponent (base : f64 , sets : & [& [f64]] , expected : f64) { sets . iter () . for_each (| s | s . iter () . for_each (| val | pow_test (base , * val , expected))) ; }
}

macro_rules! test_sets_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_sets in module {}", module_path!());
    };
}

mkfn!{
    test_sets_introspect!();
    fn test_sets (sets : & [& [f64]] , computed : & dyn Fn (f64) -> f64 , expected : & dyn Fn (f64) -> f64) { sets . iter () . for_each (| s | { s . iter () . for_each (| val | { let exp = expected (* val) ; let res = computed (* val) ; # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] let exp = force_eval ! (exp) ; # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] let res = force_eval ! (res) ; assert ! (if exp . is_nan () { res . is_nan () } else { exp == res } , "test for {val} was {res} instead of {exp}" ,) ; }) }) ; }
}

macro_rules! zero_as_exponent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zero_as_exponent in module {}", module_path!());
    };
}

mkfn!{
    zero_as_exponent_introspect!();
    # [test] fn zero_as_exponent () { test_sets_as_base (ALL , 0.0 , 1.0) ; test_sets_as_base (ALL , - 0.0 , 1.0) ; }
}

macro_rules! one_as_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function one_as_base in module {}", module_path!());
    };
}

mkfn!{
    one_as_base_introspect!();
    # [test] fn one_as_base () { test_sets_as_exponent (1.0 , ALL , 1.0) ; }
}

macro_rules! nan_inputs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nan_inputs in module {}", module_path!());
    };
}

mkfn!{
    nan_inputs_introspect!();
    # [test] fn nan_inputs () { test_sets_as_exponent (f64 :: NAN , & ALL [2 ..] , f64 :: NAN) ; test_sets_as_base (& ALL [.. (ALL . len () - 2)] , f64 :: NAN , f64 :: NAN) ; }
}

macro_rules! infinity_as_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infinity_as_base in module {}", module_path!());
    };
}

mkfn!{
    infinity_as_base_introspect!();
    # [test] fn infinity_as_base () { test_sets_as_exponent (f64 :: INFINITY , & POS [1 ..] , f64 :: INFINITY) ; test_sets_as_exponent (f64 :: INFINITY , & NEG [1 ..] , 0.0) ; test_sets_as_exponent (f64 :: NEG_INFINITY , & [POS_ODDS] , f64 :: NEG_INFINITY) ; test_sets (ALL , & | v : f64 | pow (f64 :: NEG_INFINITY , v) , & | v : f64 | { pow (- 0.0 , - v) }) ; }
}

macro_rules! infinity_as_exponent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infinity_as_exponent in module {}", module_path!());
    };
}

mkfn!{
    infinity_as_exponent_introspect!();
    # [test] fn infinity_as_exponent () { test_sets_as_base (& ALL [5 .. (ALL . len () - 2)] , f64 :: INFINITY , f64 :: INFINITY) ; test_sets_as_base (& ALL [5 .. ALL . len () - 2] , f64 :: NEG_INFINITY , 0.0) ; let base_below_one = & [POS_ZERO , NEG_ZERO , NEG_SMALL_FLOATS , POS_SMALL_FLOATS] ; test_sets_as_base (base_below_one , f64 :: INFINITY , 0.0) ; test_sets_as_base (base_below_one , f64 :: NEG_INFINITY , f64 :: INFINITY) ; test_sets_as_base (& [NEG_ONE , POS_ONE] , f64 :: INFINITY , 1.0) ; test_sets_as_base (& [NEG_ONE , POS_ONE] , f64 :: NEG_INFINITY , 1.0) ; }
}

macro_rules! zero_as_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zero_as_base in module {}", module_path!());
    };
}

mkfn!{
    zero_as_base_introspect!();
    # [test] fn zero_as_base () { test_sets_as_exponent (0.0 , & POS [1 ..] , 0.0) ; test_sets_as_exponent (0.0 , & NEG [1 ..] , f64 :: INFINITY) ; test_sets_as_exponent (- 0.0 , & POS [3 ..] , 0.0) ; test_sets_as_exponent (- 0.0 , & NEG [3 ..] , f64 :: INFINITY) ; test_sets_as_exponent (- 0.0 , & [POS_ODDS] , - 0.0) ; test_sets_as_exponent (- 0.0 , & [NEG_ODDS] , f64 :: NEG_INFINITY) ; }
}

macro_rules! special_cases_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function special_cases in module {}", module_path!());
    };
}

mkfn!{
    special_cases_introspect!();
    # [test] fn special_cases () { test_sets (ALL , & | v : f64 | pow (v , 1.0) , & | v : f64 | v) ; test_sets (ALL , & | v : f64 | pow (v , - 1.0) , & | v : f64 | 1.0 / v) ; [POS_ZERO , NEG_ZERO , POS_ONE , NEG_ONE , POS_EVENS , NEG_EVENS] . iter () . for_each (| int_set | { int_set . iter () . for_each (| int | { test_sets (ALL , & | v : f64 | pow (- v , * int) , & | v : f64 | { pow (- 1.0 , * int) * pow (v , * int) }) ; }) }) ; NEG [1 .. (NEG . len () - 1)] . iter () . for_each (| set | { set . iter () . for_each (| val | { test_sets (& ALL [3 .. 7] , & | v : f64 | pow (* val , v) , & | _ | f64 :: NAN) ; }) }) ; }
}

macro_rules! normal_cases_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function normal_cases in module {}", module_path!());
    };
}

mkfn!{
    normal_cases_introspect!();
    # [test] fn normal_cases () { assert_eq ! (pow (2.0 , 20.0) , (1 << 20) as f64) ; assert_eq ! (pow (- 1.0 , 9.0) , - 1.0) ; assert ! (pow (- 1.0 , 2.2) . is_nan ()) ; assert ! (pow (- 1.0 , - 1.14) . is_nan ()) ; }
} 
            }}