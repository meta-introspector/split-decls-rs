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
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use crate :: * ;}
mkitem!{mkstruct!{#[doc = " Generic helper for libm functions, abstracting over f32 and f64. <br/>"] #[doc = " # Type Parameter:"] #[doc = " - `T`: Either `f32` or `f64`"] #[doc = ""] #[doc = " # Examples"] #[doc = " ```rust"] #[doc = " use libm::{self, Libm};"] #[doc = ""] #[doc = " const PI_F32: f32 = 3.1415927410e+00;"] #[doc = " const PI_F64: f64 = 3.1415926535897931160e+00;"] #[doc = ""] #[doc = " assert!(Libm::<f32>::cos(0.0f32) == libm::cosf(0.0));"] #[doc = " assert!(Libm::<f32>::sin(PI_F32) == libm::sinf(PI_F32));"] #[doc = ""] #[doc = " assert!(Libm::<f64>::cos(0.0f64) == libm::cos(0.0));"] #[doc = " assert!(Libm::<f64>::sin(PI_F64) == libm::sin(PI_F64));"] #[doc = " ```"] pub struct Libm < T > (PhantomData < T >) ;}}
mkitem!{macro_rules ! libm_helper { ($ t : ident , funcs : $ funcs : tt) => { impl Libm <$ t > { #! [allow (unused_parens)] libm_helper ! { $ funcs } } } ; ({ $ ($ func : tt ;) * }) => { $ (libm_helper ! { $ func }) * } ; ((fn $ func : ident ($ ($ arg : ident : $ arg_typ : ty) ,*) -> ($ ($ ret_typ : ty) ,*) ; => $ libm_fn : ident)) => { #[inline (always)] pub fn $ func ($ ($ arg : $ arg_typ) ,*) -> ($ ($ ret_typ) ,*) { $ libm_fn ($ ($ arg) ,*) } } ; }}
mkitem!{libm_helper ! { f32 , funcs : { (fn acos (x : f32) -> (f32) ; => acosf) ; (fn acosh (x : f32) -> (f32) ; => acoshf) ; (fn asin (x : f32) -> (f32) ; => asinf) ; (fn asinh (x : f32) -> (f32) ; => asinhf) ; (fn atan (x : f32) -> (f32) ; => atanf) ; (fn atan2 (y : f32 , x : f32) -> (f32) ; => atan2f) ; (fn atanh (x : f32) -> (f32) ; => atanhf) ; (fn cbrt (x : f32) -> (f32) ; => cbrtf) ; (fn ceil (x : f32) -> (f32) ; => ceilf) ; (fn copysign (x : f32 , y : f32) -> (f32) ; => copysignf) ; (fn cos (x : f32) -> (f32) ; => cosf) ; (fn cosh (x : f32) -> (f32) ; => coshf) ; (fn erf (x : f32) -> (f32) ; => erff) ; (fn erfc (x : f32) -> (f32) ; => erfcf) ; (fn exp (x : f32) -> (f32) ; => expf) ; (fn exp10 (x : f32) -> (f32) ; => exp10f) ; (fn exp2 (x : f32) -> (f32) ; => exp2f) ; (fn expm1 (x : f32) -> (f32) ; => expm1f) ; (fn fabs (x : f32) -> (f32) ; => fabsf) ; (fn fdim (x : f32 , y : f32) -> (f32) ; => fdimf) ; (fn floor (x : f32) -> (f32) ; => floorf) ; (fn fma (x : f32 , y : f32 , z : f32) -> (f32) ; => fmaf) ; (fn fmax (x : f32 , y : f32) -> (f32) ; => fmaxf) ; (fn fmin (x : f32 , y : f32) -> (f32) ; => fminf) ; (fn fmod (x : f32 , y : f32) -> (f32) ; => fmodf) ; (fn frexp (x : f32) -> (f32 , i32) ; => frexpf) ; (fn hypot (x : f32 , y : f32) -> (f32) ; => hypotf) ; (fn ilogb (x : f32) -> (i32) ; => ilogbf) ; (fn j0 (x : f32) -> (f32) ; => j0f) ; (fn j1 (x : f32) -> (f32) ; => j1f) ; (fn jn (n : i32 , x : f32) -> (f32) ; => jnf) ; (fn ldexp (x : f32 , n : i32) -> (f32) ; => ldexpf) ; (fn lgamma (x : f32) -> (f32) ; => lgammaf) ; (fn lgamma_r (x : f32) -> (f32 , i32) ; => lgammaf_r) ; (fn log (x : f32) -> (f32) ; => logf) ; (fn log10 (x : f32) -> (f32) ; => log10f) ; (fn log1p (x : f32) -> (f32) ; => log1pf) ; (fn log2 (x : f32) -> (f32) ; => log2f) ; (fn modf (x : f32) -> (f32 , f32) ; => modff) ; (fn nextafter (x : f32 , y : f32) -> (f32) ; => nextafterf) ; (fn pow (x : f32 , y : f32) -> (f32) ; => powf) ; (fn remainder (x : f32 , y : f32) -> (f32) ; => remainderf) ; (fn remquo (x : f32 , y : f32) -> (f32 , i32) ; => remquof) ; (fn rint (x : f32) -> (f32) ; => rintf) ; (fn round (x : f32) -> (f32) ; => roundf) ; (fn roundeven (x : f32) -> (f32) ; => roundevenf) ; (fn scalbn (x : f32 , n : i32) -> (f32) ; => scalbnf) ; (fn sin (x : f32) -> (f32) ; => sinf) ; (fn sincos (x : f32) -> (f32 , f32) ; => sincosf) ; (fn sinh (x : f32) -> (f32) ; => sinhf) ; (fn sqrt (x : f32) -> (f32) ; => sqrtf) ; (fn tan (x : f32) -> (f32) ; => tanf) ; (fn tanh (x : f32) -> (f32) ; => tanhf) ; (fn tgamma (x : f32) -> (f32) ; => tgammaf) ; (fn trunc (x : f32) -> (f32) ; => truncf) ; (fn y0 (x : f32) -> (f32) ; => y0f) ; (fn y1 (x : f32) -> (f32) ; => y1f) ; (fn yn (n : i32 , x : f32) -> (f32) ; => ynf) ; } }}
mkitem!{libm_helper ! { f64 , funcs : { (fn acos (x : f64) -> (f64) ; => acos) ; (fn acosh (x : f64) -> (f64) ; => acosh) ; (fn asin (x : f64) -> (f64) ; => asin) ; (fn asinh (x : f64) -> (f64) ; => asinh) ; (fn atan (x : f64) -> (f64) ; => atan) ; (fn atan2 (y : f64 , x : f64) -> (f64) ; => atan2) ; (fn atanh (x : f64) -> (f64) ; => atanh) ; (fn cbrt (x : f64) -> (f64) ; => cbrt) ; (fn ceil (x : f64) -> (f64) ; => ceil) ; (fn copysign (x : f64 , y : f64) -> (f64) ; => copysign) ; (fn cos (x : f64) -> (f64) ; => cos) ; (fn cosh (x : f64) -> (f64) ; => cosh) ; (fn erf (x : f64) -> (f64) ; => erf) ; (fn erfc (x : f64) -> (f64) ; => erfc) ; (fn exp (x : f64) -> (f64) ; => exp) ; (fn exp10 (x : f64) -> (f64) ; => exp10) ; (fn exp2 (x : f64) -> (f64) ; => exp2) ; (fn expm1 (x : f64) -> (f64) ; => expm1) ; (fn fabs (x : f64) -> (f64) ; => fabs) ; (fn fdim (x : f64 , y : f64) -> (f64) ; => fdim) ; (fn floor (x : f64) -> (f64) ; => floor) ; (fn fma (x : f64 , y : f64 , z : f64) -> (f64) ; => fma) ; (fn fmax (x : f64 , y : f64) -> (f64) ; => fmax) ; (fn fmaximum (x : f64 , y : f64) -> (f64) ; => fmaximum) ; (fn fmaximum_num (x : f64 , y : f64) -> (f64) ; => fmaximum_num) ; (fn fmaximum_numf (x : f32 , y : f32) -> (f32) ; => fmaximum_numf) ; (fn fmaximumf (x : f32 , y : f32) -> (f32) ; => fmaximumf) ; (fn fmin (x : f64 , y : f64) -> (f64) ; => fmin) ; (fn fminimum (x : f64 , y : f64) -> (f64) ; => fminimum) ; (fn fminimum_num (x : f64 , y : f64) -> (f64) ; => fminimum_num) ; (fn fminimum_numf (x : f32 , y : f32) -> (f32) ; => fminimum_numf) ; (fn fminimumf (x : f32 , y : f32) -> (f32) ; => fminimumf) ; (fn fmod (x : f64 , y : f64) -> (f64) ; => fmod) ; (fn frexp (x : f64) -> (f64 , i32) ; => frexp) ; (fn hypot (x : f64 , y : f64) -> (f64) ; => hypot) ; (fn ilogb (x : f64) -> (i32) ; => ilogb) ; (fn j0 (x : f64) -> (f64) ; => j0) ; (fn j1 (x : f64) -> (f64) ; => j1) ; (fn jn (n : i32 , x : f64) -> (f64) ; => jn) ; (fn ldexp (x : f64 , n : i32) -> (f64) ; => ldexp) ; (fn lgamma (x : f64) -> (f64) ; => lgamma) ; (fn lgamma_r (x : f64) -> (f64 , i32) ; => lgamma_r) ; (fn log (x : f64) -> (f64) ; => log) ; (fn log10 (x : f64) -> (f64) ; => log10) ; (fn log1p (x : f64) -> (f64) ; => log1p) ; (fn log2 (x : f64) -> (f64) ; => log2) ; (fn modf (x : f64) -> (f64 , f64) ; => modf) ; (fn nextafter (x : f64 , y : f64) -> (f64) ; => nextafter) ; (fn pow (x : f64 , y : f64) -> (f64) ; => pow) ; (fn remainder (x : f64 , y : f64) -> (f64) ; => remainder) ; (fn remquo (x : f64 , y : f64) -> (f64 , i32) ; => remquo) ; (fn rint (x : f64) -> (f64) ; => rint) ; (fn round (x : f64) -> (f64) ; => round) ; (fn roundevem (x : f64) -> (f64) ; => roundeven) ; (fn scalbn (x : f64 , n : i32) -> (f64) ; => scalbn) ; (fn sin (x : f64) -> (f64) ; => sin) ; (fn sincos (x : f64) -> (f64 , f64) ; => sincos) ; (fn sinh (x : f64) -> (f64) ; => sinh) ; (fn sqrt (x : f64) -> (f64) ; => sqrt) ; (fn tan (x : f64) -> (f64) ; => tan) ; (fn tanh (x : f64) -> (f64) ; => tanh) ; (fn tgamma (x : f64) -> (f64) ; => tgamma) ; (fn trunc (x : f64) -> (f64) ; => trunc) ; (fn y0 (x : f64) -> (f64) ; => y0) ; (fn y1 (x : f64) -> (f64) ; => y1) ; (fn yn (n : i32 , x : f64) -> (f64) ; => yn) ; } }}
mkitem!{#[cfg (f16_enabled)] libm_helper ! { f16 , funcs : { (fn ceil (x : f16) -> (f16) ; => ceilf16) ; (fn copysign (x : f16 , y : f16) -> (f16) ; => copysignf16) ; (fn fabs (x : f16) -> (f16) ; => fabsf16) ; (fn fdim (x : f16 , y : f16) -> (f16) ; => fdimf16) ; (fn floor (x : f16) -> (f16) ; => floorf16) ; (fn fmax (x : f16 , y : f16) -> (f16) ; => fmaxf16) ; (fn fmaximum_num (x : f16 , y : f16) -> (f16) ; => fmaximum_numf16) ; (fn fmaximumf16 (x : f16 , y : f16) -> (f16) ; => fmaximumf16) ; (fn fmin (x : f16 , y : f16) -> (f16) ; => fminf16) ; (fn fminimum (x : f16 , y : f16) -> (f16) ; => fminimumf16) ; (fn fminimum_num (x : f16 , y : f16) -> (f16) ; => fminimum_numf16) ; (fn fmod (x : f16 , y : f16) -> (f16) ; => fmodf16) ; (fn ldexp (x : f16 , n : i32) -> (f16) ; => ldexpf16) ; (fn rint (x : f16) -> (f16) ; => rintf16) ; (fn round (x : f16) -> (f16) ; => roundf16) ; (fn roundeven (x : f16) -> (f16) ; => roundevenf16) ; (fn scalbn (x : f16 , n : i32) -> (f16) ; => scalbnf16) ; (fn sqrtf (x : f16) -> (f16) ; => sqrtf16) ; (fn truncf (x : f16) -> (f16) ; => truncf16) ; } }}
mkitem!{#[cfg (f128_enabled)] libm_helper ! { f128 , funcs : { (fn ceil (x : f128) -> (f128) ; => ceilf128) ; (fn copysign (x : f128 , y : f128) -> (f128) ; => copysignf128) ; (fn fabs (x : f128) -> (f128) ; => fabsf128) ; (fn fdim (x : f128 , y : f128) -> (f128) ; => fdimf128) ; (fn floor (x : f128) -> (f128) ; => floorf128) ; (fn fma (x : f128 , y : f128 , z : f128) -> (f128) ; => fmaf128) ; (fn fmax (x : f128 , y : f128) -> (f128) ; => fmaxf128) ; (fn fmaximum (x : f128 , y : f128) -> (f128) ; => fmaximumf128) ; (fn fmaximum_num (x : f128 , y : f128) -> (f128) ; => fmaximum_numf128) ; (fn fmin (x : f128 , y : f128) -> (f128) ; => fminf128) ; (fn fminimum (x : f128 , y : f128) -> (f128) ; => fminimumf128) ; (fn fminimum_num (x : f128 , y : f128) -> (f128) ; => fminimum_numf128) ; (fn fmod (x : f128 , y : f128) -> (f128) ; => fmodf128) ; (fn ldexp (x : f128 , n : i32) -> (f128) ; => ldexpf128) ; (fn rint (x : f128) -> (f128) ; => rintf128) ; (fn round (x : f128) -> (f128) ; => roundf128) ; (fn roundeven (x : f128) -> (f128) ; => roundevenf128) ; (fn scalbn (x : f128 , n : i32) -> (f128) ; => scalbnf128) ; (fn sqrt (x : f128) -> (f128) ; => sqrtf128) ; (fn trunc (x : f128) -> (f128) ; => truncf128) ; } }}