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
    fn common (ix : u32 , x : f64 , y1 : bool , sign : bool) -> f64 { let z : f64 ; let mut s : f64 ; let c : f64 ; let mut ss : f64 ; let mut cc : f64 ; s = sin (x) ; if y1 { s = - s ; } c = cos (x) ; cc = s - c ; if ix < 0x7fe00000 { ss = - s - c ; z = cos (2.0 * x) ; if s * c > 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x48000000 { if y1 { ss = - ss ; } cc = pone (x) * cc - qone (x) * ss ; } } if sign { cc = - cc ; } return INVSQRTPI * cc / sqrt (x) ; }
}
mkitem!{const R00 : f64 = - 6.25000000000000000000e-02 ;}
mkitem!{const R01 : f64 = 1.40705666955189706048e-03 ;}
mkitem!{const R02 : f64 = - 1.59955631084035597520e-05 ;}
mkitem!{const R03 : f64 = 4.96727999609584448412e-08 ;}
mkitem!{const S01 : f64 = 1.91537599538363460805e-02 ;}
mkitem!{const S02 : f64 = 1.85946785588630915560e-04 ;}
mkitem!{const S03 : f64 = 1.17718464042623683263e-06 ;}
mkitem!{const S04 : f64 = 5.04636257076217042715e-09 ;}
mkitem!{const S05 : f64 = 1.23542274426137913908e-11 ;}

macro_rules! j1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function j1 in module {}", module_path!());
    };
}

mkfn!{
    j1_introspect!();
    #[doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64)."] #[cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j1 (x : f64) -> f64 { let mut z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; let sign : bool ; ix = get_high_word (x) ; sign = (ix >> 31) != 0 ; ix &= 0x7fffffff ; if ix >= 0x7ff00000 { return 1.0 / (x * x) ; } if ix >= 0x40000000 { return common (ix , fabs (x) , false , sign) ; } if ix >= 0x38000000 { z = x * x ; r = z * (R00 + z * (R01 + z * (R02 + z * R03))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05)))) ; z = r / s ; } else { z = x ; } return (0.5 + z) * x ; }
}
mkitem!{const U0 : [f64 ; 5] = [- 1.96057090646238940668e-01 , 5.04438716639811282616e-02 , - 1.91256895875763547298e-03 , 2.35252600561610495928e-05 , - 9.19099158039878874504e-08 ,] ;}
mkitem!{const V0 : [f64 ; 5] = [1.99167318236649903973e-02 , 2.02552581025135171496e-04 , 1.35608801097516229404e-06 , 6.22741452364621501295e-09 , 1.66559246207992079114e-11 ,] ;}

macro_rules! y1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function y1 in module {}", module_path!());
    };
}

mkfn!{
    y1_introspect!();
    #[doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64)."] #[cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y1 (x : f64) -> f64 { let z : f64 ; let u : f64 ; let v : f64 ; let ix : u32 ; let lx : u32 ; ix = get_high_word (x) ; lx = get_low_word (x) ; if (ix << 1) | lx == 0 { return - 1.0 / 0.0 ; } if ix >> 31 != 0 { return 0.0 / 0.0 ; } if ix >= 0x7ff00000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true , false) ; } if ix < 0x3c900000 { return - TPI / x ; } z = x * x ; u = U0 [0] + z * (U0 [1] + z * (U0 [2] + z * (U0 [3] + z * U0 [4]))) ; v = 1.0 + z * (V0 [0] + z * (V0 [1] + z * (V0 [2] + z * (V0 [3] + z * V0 [4])))) ; return x * (u / v) + TPI * (j1 (x) * log (x) - 1.0 / x) ; }
}
mkitem!{const PR8 : [f64 ; 6] = [0.00000000000000000000e+00 , 1.17187499999988647970e-01 , 1.32394806593073575129e+01 , 4.12051854307378562225e+02 , 3.87474538913960532227e+03 , 7.91447954031891731574e+03 ,] ;}
mkitem!{const PS8 : [f64 ; 5] = [1.14207370375678408436e+02 , 3.65093083420853463394e+03 , 3.69562060269033463555e+04 , 9.76027935934950801311e+04 , 3.08042720627888811578e+04 ,] ;}
mkitem!{const PR5 : [f64 ; 6] = [1.31990519556243522749e-11 , 1.17187493190614097638e-01 , 6.80275127868432871736e+00 , 1.08308182990189109773e+02 , 5.17636139533199752805e+02 , 5.28715201363337541807e+02 ,] ;}
mkitem!{const PS5 : [f64 ; 5] = [5.92805987221131331921e+01 , 9.91401418733614377743e+02 , 5.35326695291487976647e+03 , 7.84469031749551231769e+03 , 1.50404688810361062679e+03 ,] ;}
mkitem!{const PR3 : [f64 ; 6] = [3.02503916137373618024e-09 , 1.17186865567253592491e-01 , 3.93297750033315640650e+00 , 3.51194035591636932736e+01 , 9.10550110750781271918e+01 , 4.85590685197364919645e+01 ,] ;}
mkitem!{const PS3 : [f64 ; 5] = [3.47913095001251519989e+01 , 3.36762458747825746741e+02 , 1.04687139975775130551e+03 , 8.90811346398256432622e+02 , 1.03787932439639277504e+02 ,] ;}
mkitem!{const PR2 : [f64 ; 6] = [1.07710830106873743082e-07 , 1.17176219462683348094e-01 , 2.36851496667608785174e+00 , 1.22426109148261232917e+01 , 1.76939711271687727390e+01 , 5.07352312588818499250e+00 ,] ;}
mkitem!{const PS2 : [f64 ; 5] = [2.14364859363821409488e+01 , 1.25290227168402751090e+02 , 2.32276469057162813669e+02 , 1.17679373287147100768e+02 , 8.36463893371618283368e+00 ,] ;}

macro_rules! pone_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pone in module {}", module_path!());
    };
}

mkfn!{
    pone_introspect!();
    fn pone (x : f64) -> f64 { let p : & [f64 ; 6] ; let q : & [f64 ; 5] ; let z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x40200000 { p = & PR8 ; q = & PS8 ; } else if ix >= 0x40122E8B { p = & PR5 ; q = & PS5 ; } else if ix >= 0x4006DB6D { p = & PR3 ; q = & PS3 ; } else { p = & PR2 ; q = & PS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * q [4])))) ; return 1.0 + r / s ; }
}
mkitem!{const QR8 : [f64 ; 6] = [0.00000000000000000000e+00 , - 1.02539062499992714161e-01 , - 1.62717534544589987888e+01 , - 7.59601722513950107896e+02 , - 1.18498066702429587167e+04 , - 4.84385124285750353010e+04 ,] ;}
mkitem!{const QS8 : [f64 ; 6] = [1.61395369700722909556e+02 , 7.82538599923348465381e+03 , 1.33875336287249578163e+05 , 7.19657723683240939863e+05 , 6.66601232617776375264e+05 , - 2.94490264303834643215e+05 ,] ;}
mkitem!{const QR5 : [f64 ; 6] = [- 2.08979931141764104297e-11 , - 1.02539050241375426231e-01 , - 8.05644828123936029840e+00 , - 1.83669607474888380239e+02 , - 1.37319376065508163265e+03 , - 2.61244440453215656817e+03 ,] ;}
mkitem!{const QS5 : [f64 ; 6] = [8.12765501384335777857e+01 , 1.99179873460485964642e+03 , 1.74684851924908907677e+04 , 4.98514270910352279316e+04 , 2.79480751638918118260e+04 , - 4.71918354795128470869e+03 ,] ;}
mkitem!{const QR3 : [f64 ; 6] = [- 5.07831226461766561369e-09 , - 1.02537829820837089745e-01 , - 4.61011581139473403113e+00 , - 5.78472216562783643212e+01 , - 2.28244540737631695038e+02 , - 2.19210128478909325622e+02 ,] ;}
mkitem!{const QS3 : [f64 ; 6] = [4.76651550323729509273e+01 , 6.73865112676699709482e+02 , 3.38015286679526343505e+03 , 5.54772909720722782367e+03 , 1.90311919338810798763e+03 , - 1.35201191444307340817e+02 ,] ;}
mkitem!{const QR2 : [f64 ; 6] = [- 1.78381727510958865572e-07 , - 1.02517042607985553460e-01 , - 2.75220568278187460720e+00 , - 1.96636162643703720221e+01 , - 4.23253133372830490089e+01 , - 2.13719211703704061733e+01 ,] ;}
mkitem!{const QS2 : [f64 ; 6] = [2.95333629060523854548e+01 , 2.52981549982190529136e+02 , 7.57502834868645436472e+02 , 7.39393205320467245656e+02 , 1.55949003336666123687e+02 , - 4.95949898822628210127e+00 ,] ;}

macro_rules! qone_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qone in module {}", module_path!());
    };
}

mkfn!{
    qone_introspect!();
    fn qone (x : f64) -> f64 { let p : & [f64 ; 6] ; let q : & [f64 ; 6] ; let s : f64 ; let r : f64 ; let z : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x40200000 { p = & QR8 ; q = & QS8 ; } else if ix >= 0x40122E8B { p = & QR5 ; q = & QS5 ; } else if ix >= 0x4006DB6D { p = & QR3 ; q = & QS3 ; } else { p = & QR2 ; q = & QS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * (q [4] + z * q [5]))))) ; return (0.375 + r / s) / x ; }
}