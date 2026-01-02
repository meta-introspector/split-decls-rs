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
    fn common (ix : u32 , x : f32 , y1 : bool , sign : bool) -> f32 { let z : f64 ; let mut s : f64 ; let c : f64 ; let mut ss : f64 ; let mut cc : f64 ; s = sinf (x) as f64 ; if y1 { s = - s ; } c = cosf (x) as f64 ; cc = s - c ; if ix < 0x7f000000 { ss = - s - c ; z = cosf (2.0 * x) as f64 ; if s * c > 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x58800000 { if y1 { ss = - ss ; } cc = (ponef (x) as f64) * cc - (qonef (x) as f64) * ss ; } } if sign { cc = - cc ; } return (((INVSQRTPI as f64) * cc) / (sqrtf (x) as f64)) as f32 ; }
}
mkitem!{const R00 : f32 = - 6.2500000000e-02 ;}
mkitem!{const R01 : f32 = 1.4070566976e-03 ;}
mkitem!{const R02 : f32 = - 1.5995563444e-05 ;}
mkitem!{const R03 : f32 = 4.9672799207e-08 ;}
mkitem!{const S01 : f32 = 1.9153760746e-02 ;}
mkitem!{const S02 : f32 = 1.8594678841e-04 ;}
mkitem!{const S03 : f32 = 1.1771846857e-06 ;}
mkitem!{const S04 : f32 = 5.0463624390e-09 ;}
mkitem!{const S05 : f32 = 1.2354227016e-11 ;}

macro_rules! j1f_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function j1f in module {}", module_path!());
    };
}

mkfn!{
    j1f_introspect!();
    # [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j1f (x : f32) -> f32 { let mut z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; let sign : bool ; ix = x . to_bits () ; sign = (ix >> 31) != 0 ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 / (x * x) ; } if ix >= 0x40000000 { return common (ix , fabsf (x) , false , sign) ; } if ix >= 0x39000000 { z = x * x ; r = z * (R00 + z * (R01 + z * (R02 + z * R03))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05)))) ; z = 0.5 + r / s ; } else { z = 0.5 ; } return z * x ; }
}
mkitem!{const U0 : [f32 ; 5] = [- 1.9605709612e-01 , 5.0443872809e-02 , - 1.9125689287e-03 , 2.3525259166e-05 , - 9.1909917899e-08 ,] ;}
mkitem!{const V0 : [f32 ; 5] = [1.9916731864e-02 , 2.0255257550e-04 , 1.3560879779e-06 , 6.2274145840e-09 , 1.6655924903e-11 ,] ;}

macro_rules! y1f_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function y1f in module {}", module_path!());
    };
}

mkfn!{
    y1f_introspect!();
    # [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y1f (x : f32) -> f32 { let z : f32 ; let u : f32 ; let v : f32 ; let ix : u32 ; ix = x . to_bits () ; if (ix & 0x7fffffff) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7f800000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true , false) ; } if ix < 0x33000000 { return - TPI / x ; } z = x * x ; u = U0 [0] + z * (U0 [1] + z * (U0 [2] + z * (U0 [3] + z * U0 [4]))) ; v = 1.0 + z * (V0 [0] + z * (V0 [1] + z * (V0 [2] + z * (V0 [3] + z * V0 [4])))) ; return x * (u / v) + TPI * (j1f (x) * logf (x) - 1.0 / x) ; }
}
mkitem!{const PR8 : [f32 ; 6] = [0.0000000000e+00 , 1.1718750000e-01 , 1.3239480972e+01 , 4.1205184937e+02 , 3.8747453613e+03 , 7.9144794922e+03 ,] ;}
mkitem!{const PS8 : [f32 ; 5] = [1.1420736694e+02 , 3.6509309082e+03 , 3.6956207031e+04 , 9.7602796875e+04 , 3.0804271484e+04 ,] ;}
mkitem!{const PR5 : [f32 ; 6] = [1.3199052094e-11 , 1.1718749255e-01 , 6.8027510643e+00 , 1.0830818176e+02 , 5.1763616943e+02 , 5.2871520996e+02 ,] ;}
mkitem!{const PS5 : [f32 ; 5] = [5.9280597687e+01 , 9.9140142822e+02 , 5.3532670898e+03 , 7.8446904297e+03 , 1.5040468750e+03 ,] ;}
mkitem!{const PR3 : [f32 ; 6] = [3.0250391081e-09 , 1.1718686670e-01 , 3.9329774380e+00 , 3.5119403839e+01 , 9.1055007935e+01 , 4.8559066772e+01 ,] ;}
mkitem!{const PS3 : [f32 ; 5] = [3.4791309357e+01 , 3.3676245117e+02 , 1.0468714600e+03 , 8.9081134033e+02 , 1.0378793335e+02 ,] ;}
mkitem!{const PR2 : [f32 ; 6] = [1.0771083225e-07 , 1.1717621982e-01 , 2.3685150146e+00 , 1.2242610931e+01 , 1.7693971634e+01 , 5.0735230446e+00 ,] ;}
mkitem!{const PS2 : [f32 ; 5] = [2.1436485291e+01 , 1.2529022980e+02 , 2.3227647400e+02 , 1.1767937469e+02 , 8.3646392822e+00 ,] ;}

macro_rules! ponef_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ponef in module {}", module_path!());
    };
}

mkfn!{
    ponef_introspect!();
    fn ponef (x : f32) -> f32 { let p : & [f32 ; 6] ; let q : & [f32 ; 5] ; let z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x41000000 { p = & PR8 ; q = & PS8 ; } else if ix >= 0x409173eb { p = & PR5 ; q = & PS5 ; } else if ix >= 0x4036d917 { p = & PR3 ; q = & PS3 ; } else { p = & PR2 ; q = & PS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * q [4])))) ; return 1.0 + r / s ; }
}
mkitem!{const QR8 : [f32 ; 6] = [0.0000000000e+00 , - 1.0253906250e-01 , - 1.6271753311e+01 , - 7.5960174561e+02 , - 1.1849806641e+04 , - 4.8438511719e+04 ,] ;}
mkitem!{const QS8 : [f32 ; 6] = [1.6139537048e+02 , 7.8253862305e+03 , 1.3387534375e+05 , 7.1965775000e+05 , 6.6660125000e+05 , - 2.9449025000e+05 ,] ;}
mkitem!{const QR5 : [f32 ; 6] = [- 2.0897993405e-11 , - 1.0253904760e-01 , - 8.0564479828e+00 , - 1.8366960144e+02 , - 1.3731937256e+03 , - 2.6124443359e+03 ,] ;}
mkitem!{const QS5 : [f32 ; 6] = [8.1276550293e+01 , 1.9917987061e+03 , 1.7468484375e+04 , 4.9851425781e+04 , 2.7948074219e+04 , - 4.7191835938e+03 ,] ;}
mkitem!{const QR3 : [f32 ; 6] = [- 5.0783124372e-09 , - 1.0253783315e-01 , - 4.6101160049e+00 , - 5.7847221375e+01 , - 2.2824453735e+02 , - 2.1921012878e+02 ,] ;}
mkitem!{const QS3 : [f32 ; 6] = [4.7665153503e+01 , 6.7386511230e+02 , 3.3801528320e+03 , 5.5477290039e+03 , 1.9031191406e+03 , - 1.3520118713e+02 ,] ;}
mkitem!{const QR2 : [f32 ; 6] = [- 1.7838172539e-07 , - 1.0251704603e-01 , - 2.7522056103e+00 , - 1.9663616180e+01 , - 4.2325313568e+01 , - 2.1371921539e+01 ,] ;}
mkitem!{const QS2 : [f32 ; 6] = [2.9533363342e+01 , 2.5298155212e+02 , 7.5750280762e+02 , 7.3939318848e+02 , 1.5594900513e+02 , - 4.9594988823e+00 ,] ;}

macro_rules! qonef_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qonef in module {}", module_path!());
    };
}

mkfn!{
    qonef_introspect!();
    fn qonef (x : f32) -> f32 { let p : & [f32 ; 6] ; let q : & [f32 ; 6] ; let s : f32 ; let r : f32 ; let z : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x41000000 { p = & QR8 ; q = & QS8 ; } else if ix >= 0x409173eb { p = & QR5 ; q = & QS5 ; } else if ix >= 0x4036d917 { p = & QR3 ; q = & QS3 ; } else { p = & QR2 ; q = & QS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * (q [4] + z * q [5]))))) ; return (0.375 + r / s) / x ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: { j1f , y1f } ;}

macro_rules! test_j1f_2488_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_j1f_2488 in module {}", module_path!());
    };
}

mkfn!{
    test_j1f_2488_introspect!();
    # [test] fn test_j1f_2488 () { assert_eq ! (j1f (2.4881766_f32) , 0.49999475_f32) ; }
}

macro_rules! test_y1f_2002_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_y1f_2002 in module {}", module_path!());
    };
}

mkfn!{
    test_y1f_2002_introspect!();
    # [test] fn test_y1f_2002 () { let res = y1f (2.0000002_f32) ; if cfg ! (all (target_arch = "x86" , not (target_feature = "sse2"))) && (res == - 0.10703231_f32) { return ; } assert_eq ! (res , - 0.10703229_f32) ; }
} 
            }}