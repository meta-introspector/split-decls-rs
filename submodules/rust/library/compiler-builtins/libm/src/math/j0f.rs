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
mkuse!{use super :: { cosf , fabsf , logf , sinf , sqrtf } ;}
mkitem!{const INVSQRTPI : f32 = 5.6418961287e-01 ;}
mkitem!{const TPI : f32 = 6.3661974669e-01 ;}

macro_rules! common_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function common in module {}", module_path!());
    };
}

mkfn!{
    common_introspect!();
    fn common (ix : u32 , x : f32 , y0 : bool) -> f32 { let z : f32 ; let s : f32 ; let mut c : f32 ; let mut ss : f32 ; let mut cc : f32 ; s = sinf (x) ; c = cosf (x) ; if y0 { c = - c ; } cc = s + c ; if ix < 0x7f000000 { ss = s - c ; z = - cosf (2.0 * x) ; if s * c < 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x58800000 { if y0 { ss = - ss ; } cc = pzerof (x) * cc - qzerof (x) * ss ; } } return INVSQRTPI * cc / sqrtf (x) ; }
}
mkitem!{const R02 : f32 = 1.5625000000e-02 ;}
mkitem!{const R03 : f32 = - 1.8997929874e-04 ;}
mkitem!{const R04 : f32 = 1.8295404516e-06 ;}
mkitem!{const R05 : f32 = - 4.6183270541e-09 ;}
mkitem!{const S01 : f32 = 1.5619102865e-02 ;}
mkitem!{const S02 : f32 = 1.1692678527e-04 ;}
mkitem!{const S03 : f32 = 5.1354652442e-07 ;}
mkitem!{const S04 : f32 = 1.1661400734e-09 ;}

macro_rules! j0f_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function j0f in module {}", module_path!());
    };
}

mkfn!{
    j0f_introspect!();
    # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j0f (mut x : f32) -> f32 { let z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 / (x * x) ; } x = fabsf (x) ; if ix >= 0x40000000 { return common (ix , x , false) ; } if ix >= 0x3a000000 { z = x * x ; r = z * (R02 + z * (R03 + z * (R04 + z * R05))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * S04))) ; return (1.0 + x / 2.0) * (1.0 - x / 2.0) + z * (r / s) ; } if ix >= 0x21800000 { x = 0.25 * x * x ; } return 1.0 - x ; }
}
mkitem!{const U00 : f32 = - 7.3804296553e-02 ;}
mkitem!{const U01 : f32 = 1.7666645348e-01 ;}
mkitem!{const U02 : f32 = - 1.3818567619e-02 ;}
mkitem!{const U03 : f32 = 3.4745343146e-04 ;}
mkitem!{const U04 : f32 = - 3.8140706238e-06 ;}
mkitem!{const U05 : f32 = 1.9559013964e-08 ;}
mkitem!{const U06 : f32 = - 3.9820518410e-11 ;}
mkitem!{const V01 : f32 = 1.2730483897e-02 ;}
mkitem!{const V02 : f32 = 7.6006865129e-05 ;}
mkitem!{const V03 : f32 = 2.5915085189e-07 ;}
mkitem!{const V04 : f32 = 4.4111031494e-10 ;}

macro_rules! y0f_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function y0f in module {}", module_path!());
    };
}

mkfn!{
    y0f_introspect!();
    # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y0f (x : f32) -> f32 { let z : f32 ; let u : f32 ; let v : f32 ; let ix : u32 ; ix = x . to_bits () ; if (ix & 0x7fffffff) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7f800000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true) ; } if ix >= 0x39000000 { z = x * x ; u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06))))) ; v = 1.0 + z * (V01 + z * (V02 + z * (V03 + z * V04))) ; return u / v + TPI * (j0f (x) * logf (x)) ; } return U00 + TPI * logf (x) ; }
}
mkitem!{const PR8 : [f32 ; 6] = [0.0000000000e+00 , - 7.0312500000e-02 , - 8.0816707611e+00 , - 2.5706311035e+02 , - 2.4852163086e+03 , - 5.2530439453e+03 ,] ;}
mkitem!{const PS8 : [f32 ; 5] = [1.1653436279e+02 , 3.8337448730e+03 , 4.0597855469e+04 , 1.1675296875e+05 , 4.7627726562e+04 ,] ;}
mkitem!{const PR5 : [f32 ; 6] = [- 1.1412546255e-11 , - 7.0312492549e-02 , - 4.1596107483e+00 , - 6.7674766541e+01 , - 3.3123129272e+02 , - 3.4643338013e+02 ,] ;}
mkitem!{const PS5 : [f32 ; 5] = [6.0753936768e+01 , 1.0512523193e+03 , 5.9789707031e+03 , 9.6254453125e+03 , 2.4060581055e+03 ,] ;}
mkitem!{const PR3 : [f32 ; 6] = [- 2.5470459075e-09 , - 7.0311963558e-02 , - 2.4090321064e+00 , - 2.1965976715e+01 , - 5.8079170227e+01 , - 3.1447946548e+01 ,] ;}
mkitem!{const PS3 : [f32 ; 5] = [3.5856033325e+01 , 3.6151397705e+02 , 1.1936077881e+03 , 1.1279968262e+03 , 1.7358093262e+02 ,] ;}
mkitem!{const PR2 : [f32 ; 6] = [- 8.8753431271e-08 , - 7.0303097367e-02 , - 1.4507384300e+00 , - 7.6356959343e+00 , - 1.1193166733e+01 , - 3.2336456776e+00 ,] ;}
mkitem!{const PS2 : [f32 ; 5] = [2.2220300674e+01 , 1.3620678711e+02 , 2.7047027588e+02 , 1.5387539673e+02 , 1.4657617569e+01 ,] ;}

macro_rules! pzerof_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pzerof in module {}", module_path!());
    };
}

mkfn!{
    pzerof_introspect!();
    fn pzerof (x : f32) -> f32 { let p : & [f32 ; 6] ; let q : & [f32 ; 5] ; let z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x41000000 { p = & PR8 ; q = & PS8 ; } else if ix >= 0x409173eb { p = & PR5 ; q = & PS5 ; } else if ix >= 0x4036d917 { p = & PR3 ; q = & PS3 ; } else { p = & PR2 ; q = & PS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * q [4])))) ; return 1.0 + r / s ; }
}
mkitem!{const QR8 : [f32 ; 6] = [0.0000000000e+00 , 7.3242187500e-02 , 1.1768206596e+01 , 5.5767340088e+02 , 8.8591972656e+03 , 3.7014625000e+04 ,] ;}
mkitem!{const QS8 : [f32 ; 6] = [1.6377603149e+02 , 8.0983447266e+03 , 1.4253829688e+05 , 8.0330925000e+05 , 8.4050156250e+05 , - 3.4389928125e+05 ,] ;}
mkitem!{const QR5 : [f32 ; 6] = [1.8408595828e-11 , 7.3242180049e-02 , 5.8356351852e+00 , 1.3511157227e+02 , 1.0272437744e+03 , 1.9899779053e+03 ,] ;}
mkitem!{const QS5 : [f32 ; 6] = [8.2776611328e+01 , 2.0778142090e+03 , 1.8847289062e+04 , 5.6751113281e+04 , 3.5976753906e+04 , - 5.3543427734e+03 ,] ;}
mkitem!{const QR3 : [f32 ; 6] = [4.3774099900e-09 , 7.3241114616e-02 , 3.3442313671e+00 , 4.2621845245e+01 , 1.7080809021e+02 , 1.6673394775e+02 ,] ;}
mkitem!{const QS3 : [f32 ; 6] = [4.8758872986e+01 , 7.0968920898e+02 , 3.7041481934e+03 , 6.4604252930e+03 , 2.5163337402e+03 , - 1.4924745178e+02 ,] ;}
mkitem!{const QR2 : [f32 ; 6] = [1.5044444979e-07 , 7.3223426938e-02 , 1.9981917143e+00 , 1.4495602608e+01 , 3.1666231155e+01 , 1.6252708435e+01 ,] ;}
mkitem!{const QS2 : [f32 ; 6] = [3.0365585327e+01 , 2.6934811401e+02 , 8.4478375244e+02 , 8.8293585205e+02 , 2.1266638184e+02 , - 5.3109550476e+00 ,] ;}

macro_rules! qzerof_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qzerof in module {}", module_path!());
    };
}

mkfn!{
    qzerof_introspect!();
    fn qzerof (x : f32) -> f32 { let p : & [f32 ; 6] ; let q : & [f32 ; 6] ; let s : f32 ; let r : f32 ; let z : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x41000000 { p = & QR8 ; q = & QS8 ; } else if ix >= 0x409173eb { p = & QR5 ; q = & QS5 ; } else if ix >= 0x4036d917 { p = & QR3 ; q = & QS3 ; } else { p = & QR2 ; q = & QS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * (q [4] + z * q [5]))))) ; return (- 0.125 + r / s) / x ; }
}