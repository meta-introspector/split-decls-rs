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
mkuse!{use super :: { cos , fabs , get_high_word , get_low_word , log , sin , sqrt } ;}
mkitem!{const INVSQRTPI : f64 = 5.64189583547756279280e-01 ;}
mkitem!{const TPI : f64 = 6.36619772367581382433e-01 ;}

macro_rules! common_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function common in module {}", module_path!());
    };
}

mkfn!{
    common_introspect!();
    fn common (ix : u32 , x : f64 , y0 : bool) -> f64 { let s : f64 ; let mut c : f64 ; let mut ss : f64 ; let mut cc : f64 ; let z : f64 ; s = sin (x) ; c = cos (x) ; if y0 { c = - c ; } cc = s + c ; if ix < 0x7fe00000 { ss = s - c ; z = - cos (2.0 * x) ; if s * c < 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x48000000 { if y0 { ss = - ss ; } cc = pzero (x) * cc - qzero (x) * ss ; } } return INVSQRTPI * cc / sqrt (x) ; }
}
mkitem!{const R02 : f64 = 1.56249999999999947958e-02 ;}
mkitem!{const R03 : f64 = - 1.89979294238854721751e-04 ;}
mkitem!{const R04 : f64 = 1.82954049532700665670e-06 ;}
mkitem!{const R05 : f64 = - 4.61832688532103189199e-09 ;}
mkitem!{const S01 : f64 = 1.56191029464890010492e-02 ;}
mkitem!{const S02 : f64 = 1.16926784663337450260e-04 ;}
mkitem!{const S03 : f64 = 5.13546550207318111446e-07 ;}
mkitem!{const S04 : f64 = 1.16614003333790000205e-09 ;}

macro_rules! j0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function j0 in module {}", module_path!());
    };
}

mkfn!{
    j0_introspect!();
    # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j0 (mut x : f64) -> f64 { let z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x7ff00000 { return 1.0 / (x * x) ; } x = fabs (x) ; if ix >= 0x40000000 { return common (ix , x , false) ; } if ix >= 0x3f200000 { z = x * x ; r = z * (R02 + z * (R03 + z * (R04 + z * R05))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * S04))) ; return (1.0 + x / 2.0) * (1.0 - x / 2.0) + z * (r / s) ; } if ix >= 0x38000000 { x = 0.25 * x * x ; } return 1.0 - x ; }
}
mkitem!{const U00 : f64 = - 7.38042951086872317523e-02 ;}
mkitem!{const U01 : f64 = 1.76666452509181115538e-01 ;}
mkitem!{const U02 : f64 = - 1.38185671945596898896e-02 ;}
mkitem!{const U03 : f64 = 3.47453432093683650238e-04 ;}
mkitem!{const U04 : f64 = - 3.81407053724364161125e-06 ;}
mkitem!{const U05 : f64 = 1.95590137035022920206e-08 ;}
mkitem!{const U06 : f64 = - 3.98205194132103398453e-11 ;}
mkitem!{const V01 : f64 = 1.27304834834123699328e-02 ;}
mkitem!{const V02 : f64 = 7.60068627350353253702e-05 ;}
mkitem!{const V03 : f64 = 2.59150851840457805467e-07 ;}
mkitem!{const V04 : f64 = 4.41110311332675467403e-10 ;}

macro_rules! y0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function y0 in module {}", module_path!());
    };
}

mkfn!{
    y0_introspect!();
    # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y0 (x : f64) -> f64 { let z : f64 ; let u : f64 ; let v : f64 ; let ix : u32 ; let lx : u32 ; ix = get_high_word (x) ; lx = get_low_word (x) ; if ((ix << 1) | lx) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7ff00000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true) ; } if ix >= 0x3e400000 { z = x * x ; u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06))))) ; v = 1.0 + z * (V01 + z * (V02 + z * (V03 + z * V04))) ; return u / v + TPI * (j0 (x) * log (x)) ; } return U00 + TPI * log (x) ; }
}
mkitem!{const PR8 : [f64 ; 6] = [0.00000000000000000000e+00 , - 7.03124999999900357484e-02 , - 8.08167041275349795626e+00 , - 2.57063105679704847262e+02 , - 2.48521641009428822144e+03 , - 5.25304380490729545272e+03 ,] ;}
mkitem!{const PS8 : [f64 ; 5] = [1.16534364619668181717e+02 , 3.83374475364121826715e+03 , 4.05978572648472545552e+04 , 1.16752972564375915681e+05 , 4.76277284146730962675e+04 ,] ;}
mkitem!{const PR5 : [f64 ; 6] = [- 1.14125464691894502584e-11 , - 7.03124940873599280078e-02 , - 4.15961064470587782438e+00 , - 6.76747652265167261021e+01 , - 3.31231299649172967747e+02 , - 3.46433388365604912451e+02 ,] ;}
mkitem!{const PS5 : [f64 ; 5] = [6.07539382692300335975e+01 , 1.05125230595704579173e+03 , 5.97897094333855784498e+03 , 9.62544514357774460223e+03 , 2.40605815922939109441e+03 ,] ;}
mkitem!{const PR3 : [f64 ; 6] = [- 2.54704601771951915620e-09 , - 7.03119616381481654654e-02 , - 2.40903221549529611423e+00 , - 2.19659774734883086467e+01 , - 5.80791704701737572236e+01 , - 3.14479470594888503854e+01 ,] ;}
mkitem!{const PS3 : [f64 ; 5] = [3.58560338055209726349e+01 , 3.61513983050303863820e+02 , 1.19360783792111533330e+03 , 1.12799679856907414432e+03 , 1.73580930813335754692e+02 ,] ;}
mkitem!{const PR2 : [f64 ; 6] = [- 8.87534333032526411254e-08 , - 7.03030995483624743247e-02 , - 1.45073846780952986357e+00 , - 7.63569613823527770791e+00 , - 1.11931668860356747786e+01 , - 3.23364579351335335033e+00 ,] ;}
mkitem!{const PS2 : [f64 ; 5] = [2.22202997532088808441e+01 , 1.36206794218215208048e+02 , 2.70470278658083486789e+02 , 1.53875394208320329881e+02 , 1.46576176948256193810e+01 ,] ;}

macro_rules! pzero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pzero in module {}", module_path!());
    };
}

mkfn!{
    pzero_introspect!();
    fn pzero (x : f64) -> f64 { let p : & [f64 ; 6] ; let q : & [f64 ; 5] ; let z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x40200000 { p = & PR8 ; q = & PS8 ; } else if ix >= 0x40122E8B { p = & PR5 ; q = & PS5 ; } else if ix >= 0x4006DB6D { p = & PR3 ; q = & PS3 ; } else { p = & PR2 ; q = & PS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * q [4])))) ; return 1.0 + r / s ; }
}
mkitem!{const QR8 : [f64 ; 6] = [0.00000000000000000000e+00 , 7.32421874999935051953e-02 , 1.17682064682252693899e+01 , 5.57673380256401856059e+02 , 8.85919720756468632317e+03 , 3.70146267776887834771e+04 ,] ;}
mkitem!{const QS8 : [f64 ; 6] = [1.63776026895689824414e+02 , 8.09834494656449805916e+03 , 1.42538291419120476348e+05 , 8.03309257119514397345e+05 , 8.40501579819060512818e+05 , - 3.43899293537866615225e+05 ,] ;}
mkitem!{const QR5 : [f64 ; 6] = [1.84085963594515531381e-11 , 7.32421766612684765896e-02 , 5.83563508962056953777e+00 , 1.35111577286449829671e+02 , 1.02724376596164097464e+03 , 1.98997785864605384631e+03 ,] ;}
mkitem!{const QS5 : [f64 ; 6] = [8.27766102236537761883e+01 , 2.07781416421392987104e+03 , 1.88472887785718085070e+04 , 5.67511122894947329769e+04 , 3.59767538425114471465e+04 , - 5.35434275601944773371e+03 ,] ;}
mkitem!{const QR3 : [f64 ; 6] = [4.37741014089738620906e-09 , 7.32411180042911447163e-02 , 3.34423137516170720929e+00 , 4.26218440745412650017e+01 , 1.70808091340565596283e+02 , 1.66733948696651168575e+02 ,] ;}
mkitem!{const QS3 : [f64 ; 6] = [4.87588729724587182091e+01 , 7.09689221056606015736e+02 , 3.70414822620111362994e+03 , 6.46042516752568917582e+03 , 2.51633368920368957333e+03 , - 1.49247451836156386662e+02 ,] ;}
mkitem!{const QR2 : [f64 ; 6] = [1.50444444886983272379e-07 , 7.32234265963079278272e-02 , 1.99819174093815998816e+00 , 1.44956029347885735348e+01 , 3.16662317504781540833e+01 , 1.62527075710929267416e+01 ,] ;}
mkitem!{const QS2 : [f64 ; 6] = [3.03655848355219184498e+01 , 2.69348118608049844624e+02 , 8.44783757595320139444e+02 , 8.82935845112488550512e+02 , 2.12666388511798828631e+02 , - 5.31095493882666946917e+00 ,] ;}

macro_rules! qzero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qzero in module {}", module_path!());
    };
}

mkfn!{
    qzero_introspect!();
    fn qzero (x : f64) -> f64 { let p : & [f64 ; 6] ; let q : & [f64 ; 6] ; let s : f64 ; let r : f64 ; let z : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x40200000 { p = & QR8 ; q = & QS8 ; } else if ix >= 0x40122E8B { p = & QR5 ; q = & QS5 ; } else if ix >= 0x4006DB6D { p = & QR3 ; q = & QS3 ; } else { p = & QR2 ; q = & QS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * (q [4] + z * q [5]))))) ; return (- 0.125 + r / s) / x ; }
}