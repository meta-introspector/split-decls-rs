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
mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * , x86_64 :: * } , mem :: transmute , } ;}
mkuse!{#[cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_cvtsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_i64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_i64&expand=1792)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsd2si))] pub fn _mm_cvtsd_i64 (a : __m128d) -> i64 { _mm_cvtsd_si64 (a) }
}

macro_rules! _mm_cvtss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_i64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_i64&expand=1894)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtss2si))] pub fn _mm_cvtss_i64 (a : __m128) -> i64 { _mm_cvtss_si64 (a) }
}

macro_rules! _mm_cvtss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtss_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtss_u64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to an unsigned 64-bit integer, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtss_u64&expand=1902)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtss2usi))] pub fn _mm_cvtss_u64 (a : __m128) -> u64 { unsafe { vcvtss2usi64 (a . as_f32x4 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvtsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtsd_u64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to an unsigned 64-bit integer, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtsd_u64&expand=1800)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsd2usi))] pub fn _mm_cvtsd_u64 (a : __m128d) -> u64 { unsafe { vcvtsd2usi64 (a . as_f64x2 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvti64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvti64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvti64_ss_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a single-precision (32-bit) floating-point element, store the result in the lower element of dst, and copy the upper 3 packed elements from a to the upper elements of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=mm_cvti64_ss&expand=1643)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2ss))] pub fn _mm_cvti64_ss (a : __m128 , b : i64) -> __m128 { unsafe { let b = b as f32 ; simd_insert ! (a , 0 , b) } }
}

macro_rules! _mm_cvti64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvti64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvti64_sd_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a double-precision (64-bit) floating-point element, store the result in the lower element of dst, and copy the upper element from a to the upper element of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvti64_sd&expand=1644)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2sd))] pub fn _mm_cvti64_sd (a : __m128d , b : i64) -> __m128d { unsafe { let b = b as f64 ; simd_insert ! (a , 0 , b) } }
}

macro_rules! _mm_cvtu64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtu64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtu64_ss_introspect!();
    #[doc = " Convert the unsigned 64-bit integer b to a single-precision (32-bit) floating-point element, store the result in the lower element of dst, and copy the upper 3 packed elements from a to the upper elements of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtu64_ss&expand=2035)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtusi2ss))] pub fn _mm_cvtu64_ss (a : __m128 , b : u64) -> __m128 { unsafe { let b = b as f32 ; simd_insert ! (a , 0 , b) } }
}

macro_rules! _mm_cvtu64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtu64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtu64_sd_introspect!();
    #[doc = " Convert the unsigned 64-bit integer b to a double-precision (64-bit) floating-point element, store the result in the lower element of dst, and copy the upper element from a to the upper element of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtu64_sd&expand=2034)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtusi2sd))] pub fn _mm_cvtu64_sd (a : __m128d , b : u64) -> __m128d { unsafe { let b = b as f64 ; simd_insert ! (a , 0 , b) } }
}

macro_rules! _mm_cvttsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttsd_i64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttsd_i64&expand=2016)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttsd2si))] pub fn _mm_cvttsd_i64 (a : __m128d) -> i64 { unsafe { vcvttsd2si64 (a . as_f64x2 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvttsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttsd_u64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to an unsigned 64-bit integer with truncation, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttsd_u64&expand=2021)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttsd2usi))] pub fn _mm_cvttsd_u64 (a : __m128d) -> u64 { unsafe { vcvttsd2usi64 (a . as_f64x2 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvttss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttss_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttss_i64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=#text=_mm_cvttss_i64&expand=2023)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttss2si))] pub fn _mm_cvttss_i64 (a : __m128) -> i64 { unsafe { vcvttss2si64 (a . as_f32x4 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvttss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttss_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttss_u64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to an unsigned 64-bit integer with truncation, and store the result in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttss_u64&expand=2027)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttss2usi))] pub fn _mm_cvttss_u64 (a : __m128) -> u64 { unsafe { vcvttss2usi64 (a . as_f32x4 () , _MM_FROUND_CUR_DIRECTION) } }
}

macro_rules! _mm_cvt_roundi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundi64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundi64_sd_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a double-precision (64-bit) floating-point element, store the result in the lower element of dst, and copy the upper element from a to the upper element of dst."] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundi64_sd&expand=1313)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2sd , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundi64_sd < const ROUNDING : i32 > (a : __m128d , b : i64) -> __m128d { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; let r = vcvtsi2sd64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundsi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundsi64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundsi64_sd_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a double-precision (64-bit) floating-point element, store the result in the lower element of dst, and copy the upper element from a to the upper element of dst."] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundsi64_sd&expand=1367)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2sd , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundsi64_sd < const ROUNDING : i32 > (a : __m128d , b : i64) -> __m128d { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; let r = vcvtsi2sd64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundi64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundi64_ss_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a single-precision (32-bit) floating-point element, store the result in the lower element of dst, and copy the upper 3 packed elements from a to the upper elements of dst."] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundi64_ss&expand=1314)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2ss , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundi64_ss < const ROUNDING : i32 > (a : __m128 , b : i64) -> __m128 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; let r = vcvtsi2ss64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundu64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundu64_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundu64_sd_introspect!();
    #[doc = " Convert the unsigned 64-bit integer b to a double-precision (64-bit) floating-point element, store the result in the lower element of dst, and copy the upper element from a to the upper element of dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundu64_sd&expand=1379)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtusi2sd , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundu64_sd < const ROUNDING : i32 > (a : __m128d , b : u64) -> __m128d { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; let r = vcvtusi2sd64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundsi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundsi64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundsi64_ss_introspect!();
    #[doc = " Convert the signed 64-bit integer b to a single-precision (32-bit) floating-point element, store the result in the lower element of dst, and copy the upper 3 packed elements from a to the upper elements of dst."] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundsi64_ss&expand=1368)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsi2ss , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundsi64_ss < const ROUNDING : i32 > (a : __m128 , b : i64) -> __m128 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; let r = vcvtsi2ss64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundu64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundu64_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundu64_ss_introspect!();
    #[doc = " Convert the unsigned 64-bit integer b to a single-precision (32-bit) floating-point element, store the result in the lower element of dst, and copy the upper 3 packed elements from a to the upper elements of dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundu64_ss&expand=1380)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtusi2ss , ROUNDING = 8))] #[rustc_legacy_const_generics (2)] pub fn _mm_cvt_roundu64_ss < const ROUNDING : i32 > (a : __m128 , b : u64) -> __m128 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; let r = vcvtusi2ss64 (a , b , ROUNDING) ; transmute (r) } }
}

macro_rules! _mm_cvt_roundsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundsd_si64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundsd_si64&expand=1360)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsd2si , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundsd_si64 < const ROUNDING : i32 > (a : __m128d) -> i64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; vcvtsd2si64 (a , ROUNDING) } }
}

macro_rules! _mm_cvt_roundsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundsd_i64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundsd_i64&expand=1358)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsd2si , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundsd_i64 < const ROUNDING : i32 > (a : __m128d) -> i64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; vcvtsd2si64 (a , ROUNDING) } }
}

macro_rules! _mm_cvt_roundsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundsd_u64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to an unsigned 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundsd_u64&expand=1365)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtsd2usi , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundsd_u64 < const ROUNDING : i32 > (a : __m128d) -> u64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f64x2 () ; vcvtsd2usi64 (a , ROUNDING) } }
}

macro_rules! _mm_cvt_roundss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundss_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundss_si64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundss_si64&expand=1375)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtss2si , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundss_si64 < const ROUNDING : i32 > (a : __m128) -> i64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; vcvtss2si64 (a , ROUNDING) } }
}

macro_rules! _mm_cvt_roundss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundss_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundss_i64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundss_i64&expand=1370)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtss2si , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundss_i64 < const ROUNDING : i32 > (a : __m128) -> i64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; vcvtss2si64 (a , ROUNDING) } }
}

macro_rules! _mm_cvt_roundss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvt_roundss_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvt_roundss_u64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to an unsigned 64-bit integer, and store the result in dst.\\"] #[doc = " Rounding is done according to the rounding\\[3:0\\] parameter, which can be one of:\\"] #[doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] #[doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] #[doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvt_roundss_u64&expand=1377)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvtss2usi , ROUNDING = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvt_roundss_u64 < const ROUNDING : i32 > (a : __m128) -> u64 { unsafe { static_assert_rounding ! (ROUNDING) ; let a = a . as_f32x4 () ; vcvtss2usi64 (a , ROUNDING) } }
}

macro_rules! _mm_cvtt_roundsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundsd_si64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundsd_si64&expand=1931)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttsd2si , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundsd_si64 < const SAE : i32 > (a : __m128d) -> i64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f64x2 () ; vcvttsd2si64 (a , SAE) } }
}

macro_rules! _mm_cvtt_roundsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundsd_i64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundsd_i64&expand=1929)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttsd2si , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundsd_i64 < const SAE : i32 > (a : __m128d) -> i64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f64x2 () ; vcvttsd2si64 (a , SAE) } }
}

macro_rules! _mm_cvtt_roundsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundsd_u64_introspect!();
    #[doc = " Convert the lower double-precision (64-bit) floating-point element in a to an unsigned 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundsd_u64&expand=1933)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttsd2usi , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundsd_u64 < const SAE : i32 > (a : __m128d) -> u64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f64x2 () ; vcvttsd2usi64 (a , SAE) } }
}

macro_rules! _mm_cvtt_roundss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundss_i64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundss_i64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundss_i64&expand=1935)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttss2si , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundss_i64 < const SAE : i32 > (a : __m128) -> i64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f32x4 () ; vcvttss2si64 (a , SAE) } }
}

macro_rules! _mm_cvtt_roundss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundss_si64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundss_si64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to a 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundss_si64&expand=1937)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttss2si , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundss_si64 < const SAE : i32 > (a : __m128) -> i64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f32x4 () ; vcvttss2si64 (a , SAE) } }
}

macro_rules! _mm_cvtt_roundss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtt_roundss_u64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtt_roundss_u64_introspect!();
    #[doc = " Convert the lower single-precision (32-bit) floating-point element in a to an unsigned 64-bit integer with truncation, and store the result in dst.\\"] #[doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtt_roundss_u64&expand=1939)"] #[inline] #[target_feature (enable = "avx512f")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vcvttss2usi , SAE = 8))] #[rustc_legacy_const_generics (1)] pub fn _mm_cvtt_roundss_u64 < const SAE : i32 > (a : __m128) -> u64 { unsafe { static_assert_sae ! (SAE) ; let a = a . as_f32x4 () ; vcvttss2usi64 (a , SAE) } }
}
mkitem!{#[allow (improper_ctypes)] unsafe extern "C" { #[link_name = "llvm.x86.avx512.vcvtss2si64"] fn vcvtss2si64 (a : f32x4 , rounding : i32) -> i64 ; #[link_name = "llvm.x86.avx512.vcvtss2usi64"] fn vcvtss2usi64 (a : f32x4 , rounding : i32) -> u64 ; #[link_name = "llvm.x86.avx512.vcvtsd2si64"] fn vcvtsd2si64 (a : f64x2 , rounding : i32) -> i64 ; #[link_name = "llvm.x86.avx512.vcvtsd2usi64"] fn vcvtsd2usi64 (a : f64x2 , rounding : i32) -> u64 ; #[link_name = "llvm.x86.avx512.cvtsi2ss64"] fn vcvtsi2ss64 (a : f32x4 , b : i64 , rounding : i32) -> f32x4 ; #[link_name = "llvm.x86.avx512.cvtsi2sd64"] fn vcvtsi2sd64 (a : f64x2 , b : i64 , rounding : i32) -> f64x2 ; #[link_name = "llvm.x86.avx512.cvtusi642ss"] fn vcvtusi2ss64 (a : f32x4 , b : u64 , rounding : i32) -> f32x4 ; #[link_name = "llvm.x86.avx512.cvtusi642sd"] fn vcvtusi2sd64 (a : f64x2 , b : u64 , rounding : i32) -> f64x2 ; #[link_name = "llvm.x86.avx512.cvttss2si64"] fn vcvttss2si64 (a : f32x4 , rounding : i32) -> i64 ; #[link_name = "llvm.x86.avx512.cvttss2usi64"] fn vcvttss2usi64 (a : f32x4 , rounding : i32) -> u64 ; #[link_name = "llvm.x86.avx512.cvttsd2si64"] fn vcvttsd2si64 (a : f64x2 , rounding : i32) -> i64 ; #[link_name = "llvm.x86.avx512.cvttsd2usi64"] fn vcvttsd2usi64 (a : f64x2 , rounding : i32) -> u64 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use crate :: core_arch :: x86_64 :: * ;}
mkuse!{use crate :: hint :: black_box ;}

macro_rules! test_mm512_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_abs_epi64 () { let a = _mm512_set_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let r = _mm512_abs_epi64 (a) ; let e = _mm512_set_epi64 (0 , 1 , 1 , i64 :: MAX , i64 :: MAX . wrapping_add (1) , 100 , 100 , 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_abs_epi64 () { let a = _mm512_set_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let r = _mm512_mask_abs_epi64 (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_abs_epi64 (a , 0b11111111 , a) ; let e = _mm512_set_epi64 (0 , 1 , 1 , i64 :: MAX , i64 :: MIN , 100 , 100 , 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_abs_epi64 () { let a = _mm512_set_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let r = _mm512_maskz_abs_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_abs_epi64 (0b11111111 , a) ; let e = _mm512_set_epi64 (0 , 1 , 1 , i64 :: MAX , i64 :: MIN , 100 , 100 , 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_abs_epi64 () { let a = _mm256_set_epi64x (i64 :: MAX , i64 :: MIN , 100 , - 100) ; let r = _mm256_abs_epi64 (a) ; let e = _mm256_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1) , 100 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_abs_epi64 () { let a = _mm256_set_epi64x (i64 :: MAX , i64 :: MIN , 100 , - 100) ; let r = _mm256_mask_abs_epi64 (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_abs_epi64 (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1) , 100 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_abs_epi64 () { let a = _mm256_set_epi64x (i64 :: MAX , i64 :: MIN , 100 , - 100) ; let r = _mm256_maskz_abs_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_abs_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1) , 100 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_abs_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let r = _mm_abs_epi64 (a) ; let e = _mm_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1)) ; assert_eq_m128i (r , e) ; let a = _mm_set_epi64x (100 , - 100) ; let r = _mm_abs_epi64 (a) ; let e = _mm_set_epi64x (100 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_abs_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let r = _mm_mask_abs_epi64 (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_abs_epi64 (a , 0b00000011 , a) ; let e = _mm_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1)) ; assert_eq_m128i (r , e) ; let a = _mm_set_epi64x (100 , - 100) ; let r = _mm_mask_abs_epi64 (a , 0b00000011 , a) ; let e = _mm_set_epi64x (100 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_abs_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_abs_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_abs_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_abs_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let r = _mm_maskz_abs_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_abs_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (i64 :: MAX , i64 :: MAX . wrapping_add (1)) ; assert_eq_m128i (r , e) ; let a = _mm_set_epi64x (100 , - 100) ; let r = _mm_maskz_abs_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (100 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_abs_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_abs_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_abs_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_abs_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let r = _mm512_abs_pd (a) ; let e = _mm512_setr_pd (0. , 1. , 1. , f64 :: MAX , f64 :: MAX , 100. , 100. , 32.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_abs_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_abs_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_abs_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_abs_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let r = _mm512_mask_abs_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_abs_pd (a , 0b00001111 , a) ; let e = _mm512_setr_pd (0. , 1. , 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mov_epi64 () { let src = _mm512_set1_epi64 (1) ; let a = _mm512_set1_epi64 (2) ; let r = _mm512_mask_mov_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_mov_epi64 (src , 0b11111111 , a) ; assert_eq_m512i (r , a) ; }
}

macro_rules! test_mm512_maskz_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mov_epi64 () { let a = _mm512_set1_epi64 (2) ; let r = _mm512_maskz_mov_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_mov_epi64 (0b11111111 , a) ; assert_eq_m512i (r , a) ; }
}

macro_rules! test_mm256_mask_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_mov_epi64 () { let src = _mm256_set1_epi64x (1) ; let a = _mm256_set1_epi64x (2) ; let r = _mm256_mask_mov_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_mov_epi64 (src , 0b00001111 , a) ; assert_eq_m256i (r , a) ; }
}

macro_rules! test_mm256_maskz_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_mov_epi64 () { let a = _mm256_set1_epi64x (2) ; let r = _mm256_maskz_mov_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_mov_epi64 (0b00001111 , a) ; assert_eq_m256i (r , a) ; }
}

macro_rules! test_mm_mask_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_mov_epi64 () { let src = _mm_set1_epi64x (1) ; let a = _mm_set1_epi64x (2) ; let r = _mm_mask_mov_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_mov_epi64 (src , 0b00000011 , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm_maskz_mov_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mov_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mov_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_mov_epi64 () { let a = _mm_set1_epi64x (2) ; let r = _mm_maskz_mov_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_mov_epi64 (0b00000011 , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm512_mask_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mov_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mov_pd () { let src = _mm512_set1_pd (1.) ; let a = _mm512_set1_pd (2.) ; let r = _mm512_mask_mov_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_mov_pd (src , 0b11111111 , a) ; assert_eq_m512d (r , a) ; }
}

macro_rules! test_mm512_maskz_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mov_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mov_pd () { let a = _mm512_set1_pd (2.) ; let r = _mm512_maskz_mov_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_mov_pd (0b11111111 , a) ; assert_eq_m512d (r , a) ; }
}

macro_rules! test_mm256_mask_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mov_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_mov_pd () { let src = _mm256_set1_pd (1.) ; let a = _mm256_set1_pd (2.) ; let r = _mm256_mask_mov_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_mov_pd (src , 0b00001111 , a) ; assert_eq_m256d (r , a) ; }
}

macro_rules! test_mm256_maskz_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mov_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_mov_pd () { let a = _mm256_set1_pd (2.) ; let r = _mm256_maskz_mov_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_mov_pd (0b00001111 , a) ; assert_eq_m256d (r , a) ; }
}

macro_rules! test_mm_mask_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mov_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_mov_pd () { let src = _mm_set1_pd (1.) ; let a = _mm_set1_pd (2.) ; let r = _mm_mask_mov_pd (src , 0 , a) ; assert_eq_m128d (r , src) ; let r = _mm_mask_mov_pd (src , 0b00000011 , a) ; assert_eq_m128d (r , a) ; }
}

macro_rules! test_mm_maskz_mov_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mov_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mov_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_mov_pd () { let a = _mm_set1_pd (2.) ; let r = _mm_maskz_mov_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_mov_pd (0b00000011 , a) ; assert_eq_m128d (r , a) ; }
}

macro_rules! test_mm512_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_add_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_add_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_add_epi64 (a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 0 , i64 :: MIN , i64 :: MIN + 1 , 101 , - 99 , - 31) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_add_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_add_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_mask_add_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_add_epi64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 0 , i64 :: MIN , i64 :: MIN , 100 , - 100 , - 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_add_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_add_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_maskz_add_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_add_epi64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 0 , i64 :: MIN , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_add_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_add_epi64 () { let a = _mm256_set_epi64x (1 , - 1 , i64 :: MAX , i64 :: MIN) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_mask_add_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_add_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 0 , i64 :: MIN , i64 :: MIN + 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_add_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_add_epi64 () { let a = _mm256_set_epi64x (1 , - 1 , i64 :: MAX , i64 :: MIN) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_add_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_add_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 0 , i64 :: MIN , i64 :: MIN + 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_add_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_add_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let b = _mm_set1_epi64x (1) ; let r = _mm_mask_add_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_add_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (i64 :: MIN , i64 :: MIN + 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_add_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_add_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let b = _mm_set1_epi64x (1) ; let r = _mm_maskz_add_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_add_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (i64 :: MIN , i64 :: MIN + 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_add_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_add_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_add_pd (a , b) ; let e = _mm512_setr_pd (1. , 2. , 0. , f64 :: MAX , f64 :: MIN + 1. , 101. , - 99. , - 31.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_add_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_add_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_mask_add_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_add_pd (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (1. , 2. , 0. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_add_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_add_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_maskz_add_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_add_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (1. , 2. , 0. , f64 :: MAX , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_add_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_add_pd () { let a = _mm256_set_pd (1. , - 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (1.) ; let r = _mm256_mask_add_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_add_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (2. , 0. , f64 :: MAX , f64 :: MIN + 1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_add_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_add_pd () { let a = _mm256_set_pd (1. , - 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (1.) ; let r = _mm256_maskz_add_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_add_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (2. , 0. , f64 :: MAX , f64 :: MIN + 1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_add_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_add_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (1.) ; let r = _mm_mask_add_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_add_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: MAX , f64 :: MIN + 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_add_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_add_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (1.) ; let r = _mm_maskz_add_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_add_pd (0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: MAX , f64 :: MIN + 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sub_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_sub_epi64 (a , b) ; let e = _mm512_setr_epi64 (- 1 , 0 , - 2 , i64 :: MAX - 1 , i64 :: MAX , 99 , - 101 , - 33) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sub_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_mask_sub_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_sub_epi64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (- 1 , 0 , - 2 , i64 :: MAX - 1 , i64 :: MIN , 100 , - 100 , - 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sub_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , - 1 , i64 :: MAX , i64 :: MIN , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (1) ; let r = _mm512_maskz_sub_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_sub_epi64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (- 1 , 0 , - 2 , i64 :: MAX - 1 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sub_epi64 () { let a = _mm256_set_epi64x (1 , - 1 , i64 :: MAX , i64 :: MIN) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_mask_sub_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_sub_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , - 2 , i64 :: MAX - 1 , i64 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sub_epi64 () { let a = _mm256_set_epi64x (1 , - 1 , i64 :: MAX , i64 :: MIN) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_sub_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_sub_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , - 2 , i64 :: MAX - 1 , i64 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sub_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let b = _mm_set1_epi64x (1) ; let r = _mm_mask_sub_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_sub_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (i64 :: MAX - 1 , i64 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_sub_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sub_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sub_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sub_epi64 () { let a = _mm_set_epi64x (i64 :: MAX , i64 :: MIN) ; let b = _mm_set1_epi64x (1) ; let r = _mm_maskz_sub_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_sub_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (i64 :: MAX - 1 , i64 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sub_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_sub_pd (a , b) ; let e = _mm512_setr_pd (- 1. , 0. , - 2. , f64 :: MAX - 1. , f64 :: MIN , 99. , - 101. , - 33.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sub_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_mask_sub_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_sub_pd (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (- 1. , 0. , - 2. , f64 :: MAX - 1. , f64 :: MIN , 100. , - 100. , - 32.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sub_pd () { let a = _mm512_setr_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: MIN , 100. , - 100. , - 32.) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_maskz_sub_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_sub_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (- 1. , 0. , - 2. , f64 :: MAX - 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sub_pd () { let a = _mm256_set_pd (1. , - 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (1.) ; let r = _mm256_mask_sub_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_sub_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (0. , - 2. , f64 :: MAX - 1. , f64 :: MIN) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sub_pd () { let a = _mm256_set_pd (1. , - 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (1.) ; let r = _mm256_maskz_sub_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_sub_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (0. , - 2. , f64 :: MAX - 1. , f64 :: MIN) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sub_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (1.) ; let r = _mm_mask_sub_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_sub_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: MAX - 1. , f64 :: MIN) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_sub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sub_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (1.) ; let r = _mm_maskz_sub_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_sub_pd (0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: MAX - 1. , f64 :: MIN) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mul_epi32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mul_epi32 (a , b) ; let e = _mm512_set_epi64 (15 , 13 , 11 , 9 , 7 , 5 , 3 , 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mul_epi32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_mul_epi32 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_mul_epi32 (a , 0b00001111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 | 1 << 32 , 1 | 1 << 32 , 1 | 1 << 32 , 1 | 1 << 32 , 7 , 5 , 3 , 1 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mul_epi32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_maskz_mul_epi32 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_mul_epi32 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 7 , 5 , 3 , 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_mul_epi32 () { let a = _mm256_set1_epi32 (1) ; let b = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm256_mask_mul_epi32 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_mul_epi32 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 4 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_mul_epi32 () { let a = _mm256_set1_epi32 (1) ; let b = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm256_maskz_mul_epi32 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_mul_epi32 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 4 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_mul_epi32 () { let a = _mm_set1_epi32 (1) ; let b = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_mask_mul_epi32 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_mul_epi32 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (2 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_mul_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mul_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mul_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_mul_epi32 () { let a = _mm_set1_epi32 (1) ; let b = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_maskz_mul_epi32 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_mul_epi32 (0b00000011 , a , b) ; let e = _mm_set_epi64x (2 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mul_epu32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mul_epu32 (a , b) ; let e = _mm512_set_epi64 (15 , 13 , 11 , 9 , 7 , 5 , 3 , 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mul_epu32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_mul_epu32 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_mul_epu32 (a , 0b00001111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 | 1 << 32 , 1 | 1 << 32 , 1 | 1 << 32 , 1 | 1 << 32 , 7 , 5 , 3 , 1 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mul_epu32 () { let a = _mm512_set1_epi32 (1) ; let b = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_maskz_mul_epu32 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_mul_epu32 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 7 , 5 , 3 , 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_mul_epu32 () { let a = _mm256_set1_epi32 (1) ; let b = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm256_mask_mul_epu32 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_mul_epu32 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 4 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_mul_epu32 () { let a = _mm256_set1_epi32 (1) ; let b = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm256_maskz_mul_epu32 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_mul_epu32 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (2 , 4 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_mul_epu32 () { let a = _mm_set1_epi32 (1) ; let b = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_mask_mul_epu32 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_mul_epu32 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (2 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_mul_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mul_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mul_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_mul_epu32 () { let a = _mm_set1_epi32 (1) ; let b = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_maskz_mul_epu32 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_mul_epu32 (0b00000011 , a , b) ; let e = _mm_set_epi64x (2 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mullox_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mullox_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mullox_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mullox_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , i64 :: MAX , i64 :: MIN , i64 :: MAX , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (2) ; let r = _mm512_mullox_epi64 (a , b) ; let e = _mm512_setr_epi64 (0 , 2 , - 2 , 0 , - 2 , 200 , - 200 , - 64) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_mullox_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mullox_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mullox_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mullox_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , i64 :: MAX , i64 :: MIN , i64 :: MAX , 100 , - 100 , - 32) ; let b = _mm512_set1_epi64 (2) ; let r = _mm512_mask_mullox_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_mullox_epi64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (0 , 2 , - 2 , 0 , i64 :: MAX , 100 , - 100 , - 32) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mul_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mul_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_set1_pd (2.) ; let r = _mm512_mul_pd (a , b) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 2. , f64 :: INFINITY , f64 :: NEG_INFINITY , f64 :: INFINITY , f64 :: NEG_INFINITY , - 200. , - 64. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mul_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mul_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_set1_pd (2.) ; let r = _mm512_mask_mul_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_mul_pd (a , 0b00001111 , a , b) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 2. , f64 :: INFINITY , f64 :: NEG_INFINITY , f64 :: MAX , f64 :: MIN , - 100. , - 32. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mul_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mul_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_set1_pd (2.) ; let r = _mm512_maskz_mul_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_mul_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 2. , f64 :: INFINITY , f64 :: NEG_INFINITY , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mul_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_mul_pd () { let a = _mm256_set_pd (0. , 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (2.) ; let r = _mm256_mask_mul_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_mul_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 2. , f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mul_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_mul_pd () { let a = _mm256_set_pd (0. , 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set1_pd (2.) ; let r = _mm256_maskz_mul_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_mul_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 2. , f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mul_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_mul_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (2.) ; let r = _mm_mask_mul_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_mul_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mul_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_mul_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set1_pd (2.) ; let r = _mm_maskz_mul_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_mul_pd (0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_div_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_div_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_setr_pd (2. , 2. , 0. , 0. , 0. , 0. , 2. , 2.) ; let r = _mm512_div_pd (a , b) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 0.5 , f64 :: INFINITY , f64 :: NEG_INFINITY , f64 :: INFINITY , f64 :: NEG_INFINITY , - 50. , - 16. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_div_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_div_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_setr_pd (2. , 2. , 0. , 0. , 0. , 0. , 2. , 2.) ; let r = _mm512_mask_div_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_div_pd (a , 0b00001111 , a , b) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 0.5 , f64 :: INFINITY , f64 :: NEG_INFINITY , f64 :: MAX , f64 :: MIN , - 100. , - 32. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_div_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_div_pd () { let a = _mm512_setr_pd (0. , 1. , f64 :: MAX , f64 :: MIN , f64 :: MAX , f64 :: MIN , - 100. , - 32.) ; let b = _mm512_setr_pd (2. , 2. , 0. , 0. , 0. , 0. , 2. , 2.) ; let r = _mm512_maskz_div_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_div_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 0.5 , f64 :: INFINITY , f64 :: NEG_INFINITY , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_div_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_div_pd () { let a = _mm256_set_pd (0. , 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set_pd (2. , 2. , 0. , 0.) ; let r = _mm256_mask_div_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_div_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 0.5 , f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_div_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_div_pd () { let a = _mm256_set_pd (0. , 1. , f64 :: MAX , f64 :: MIN) ; let b = _mm256_set_pd (2. , 2. , 0. , 0.) ; let r = _mm256_maskz_div_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_div_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 0.5 , f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_div_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_div_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set_pd (0. , 0.) ; let r = _mm_mask_div_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_div_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_div_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_div_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_div_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_div_pd () { let a = _mm_set_pd (f64 :: MAX , f64 :: MIN) ; let b = _mm_set_pd (0. , 0.) ; let r = _mm_maskz_div_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_div_pd (0b00000011 , a , b) ; let e = _mm_set_pd (f64 :: INFINITY , f64 :: NEG_INFINITY) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_max_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_max_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_max_epi64 (a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_max_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_max_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_mask_max_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_max_epi64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_max_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_max_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_maskz_max_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_max_epi64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_max_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_max_epi64 (a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_max_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_mask_max_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_max_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_max_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_maskz_max_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_max_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_max_epi64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_max_epi64 (a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_max_epi64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_mask_max_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_max_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_max_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_max_epi64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_maskz_max_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_max_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_max_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_max_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_max_pd (a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_max_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_max_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_mask_max_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_max_pd (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_max_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_max_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_maskz_max_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_max_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_max_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_max_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_mask_max_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_max_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (3. , 2. , 2. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_max_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_max_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_maskz_max_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_max_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (3. , 2. , 2. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_max_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_max_pd () { let a = _mm_set_pd (2. , 3.) ; let b = _mm_set_pd (3. , 2.) ; let r = _mm_mask_max_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_max_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (3. , 3.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_max_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_max_pd () { let a = _mm_set_pd (2. , 3.) ; let b = _mm_set_pd (3. , 2.) ; let r = _mm_maskz_max_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_max_pd (0b00000011 , a , b) ; let e = _mm_set_pd (3. , 3.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_max_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_max_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_max_epu64 (a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_max_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_max_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_mask_max_epu64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_max_epu64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_max_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_max_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_maskz_max_epu64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_max_epu64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_max_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_max_epu64 (a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_max_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_mask_max_epu64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_max_epu64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_max_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_maskz_max_epu64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_max_epu64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (3 , 2 , 2 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_max_epu64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_max_epu64 (a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_max_epu64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_mask_max_epu64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_max_epu64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_max_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_max_epu64 () { let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_maskz_max_epu64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_max_epu64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (3 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_min_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_min_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_min_epi64 (a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 3 , 2 , 1 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_min_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_min_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_mask_min_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_min_epi64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_min_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_min_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_maskz_min_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_min_epi64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_min_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_min_epi64 (a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_min_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_mask_min_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_min_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_min_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_maskz_min_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_min_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_min_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_min_epi64 (a , b) ; let e = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , e) ; let a = _mm_set_epi64x (2 , 3) ; let b = _mm_set_epi64x (1 , 0) ; let r = _mm_min_epi64 (a , b) ; let e = _mm_set_epi64x (1 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_min_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_mask_min_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_min_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_min_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_min_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (3 , 2) ; let r = _mm_maskz_min_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_min_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_min_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_min_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_min_pd (a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 3. , 2. , 1. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_min_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_min_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_mask_min_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_min_pd (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_min_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_min_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_maskz_min_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_min_pd (0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_min_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_min_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_mask_min_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_min_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 1. , 1. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_min_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_min_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_maskz_min_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_min_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (0. , 1. , 1. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_min_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_min_pd () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set_pd (1. , 0.) ; let r = _mm_mask_min_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_min_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (0. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_min_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_min_pd () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set_pd (1. , 0.) ; let r = _mm_maskz_min_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_min_pd (0b00000011 , a , b) ; let e = _mm_set_pd (0. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_min_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_min_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_min_epu64 (a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 3 , 2 , 1 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_min_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_min_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_mask_min_epu64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_min_epu64 (a , 0b00001111 , a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_min_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_min_epu64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let b = _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0) ; let r = _mm512_maskz_min_epu64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_min_epu64 (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_min_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_min_epu64 (a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_min_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_mask_min_epu64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_min_epu64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_min_epu64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_maskz_min_epu64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_min_epu64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (0 , 1 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_min_epu64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (1 , 0) ; let r = _mm_min_epu64 (a , b) ; let e = _mm_set_epi64x (0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_min_epu64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (1 , 0) ; let r = _mm_mask_min_epu64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_min_epu64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_min_epu64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_min_epu64 () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (1 , 0) ; let r = _mm_maskz_min_epu64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_min_epu64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sqrt_pd () { let a = _mm512_setr_pd (0. , 1. , 4. , 9. , 16. , 25. , 36. , 49.) ; let r = _mm512_sqrt_pd (a) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sqrt_pd () { let a = _mm512_setr_pd (0. , 1. , 4. , 9. , 16. , 25. , 36. , 49.) ; let r = _mm512_mask_sqrt_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_sqrt_pd (a , 0b00001111 , a) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 16. , 25. , 36. , 49.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sqrt_pd () { let a = _mm512_setr_pd (0. , 1. , 4. , 9. , 16. , 25. , 36. , 49.) ; let r = _mm512_maskz_sqrt_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_sqrt_pd (0b00001111 , a) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sqrt_pd () { let a = _mm256_set_pd (0. , 1. , 4. , 9.) ; let r = _mm256_mask_sqrt_pd (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_sqrt_pd (a , 0b00001111 , a) ; let e = _mm256_set_pd (0. , 1. , 2. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sqrt_pd () { let a = _mm256_set_pd (0. , 1. , 4. , 9.) ; let r = _mm256_maskz_sqrt_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_sqrt_pd (0b00001111 , a) ; let e = _mm256_set_pd (0. , 1. , 2. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sqrt_pd () { let a = _mm_set_pd (0. , 1.) ; let r = _mm_mask_sqrt_pd (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_sqrt_pd (a , 0b00000011 , a) ; let e = _mm_set_pd (0. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_sqrt_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sqrt_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sqrt_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sqrt_pd () { let a = _mm_set_pd (0. , 1.) ; let r = _mm_maskz_sqrt_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_sqrt_pd (0b00000011 , a) ; let e = _mm_set_pd (0. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmadd_pd () { let a = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let r = _mm512_fmadd_pd (a , b , c) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmadd_pd () { let a = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let r = _mm512_mask_fmadd_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmadd_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmadd_pd () { let a = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let r = _mm512_maskz_fmadd_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmadd_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmadd_pd () { let a = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fmadd_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmadd_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fmadd_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fmadd_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (1. , 2. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fmadd_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fmadd_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (1. , 2. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fmadd_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fmadd_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (1. , 2. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fmadd_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fmadd_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fmadd_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fmadd_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fmadd_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fmadd_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fmsub_pd (a , b , c) ; let e = _mm512_setr_pd (- 1. , 0. , 1. , 2. , 3. , 4. , 5. , 6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fmsub_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmsub_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (- 1. , 0. , 1. , 2. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fmsub_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmsub_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (- 1. , 0. , 1. , 2. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fmsub_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmsub_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (- 1. , 0. , 1. , 2. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fmsub_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fmsub_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (- 1. , 0. , 1. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fmsub_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fmsub_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (- 1. , 0. , 1. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fmsub_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fmsub_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (- 1. , 0. , 1. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fmsub_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fmsub_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (- 1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fmsub_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fmsub_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (- 1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fmsub_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fmsub_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (- 1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmaddsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fmaddsub_pd (a , b , c) ; let e = _mm512_setr_pd (- 1. , 2. , 1. , 4. , 3. , 6. , 5. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmaddsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fmaddsub_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmaddsub_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (- 1. , 2. , 1. , 4. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmaddsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fmaddsub_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmaddsub_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (- 1. , 2. , 1. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmaddsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fmaddsub_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmaddsub_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (- 1. , 2. , 1. , 4. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fmaddsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fmaddsub_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fmaddsub_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (1. , 0. , 3. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fmaddsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fmaddsub_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fmaddsub_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (1. , 0. , 3. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fmaddsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fmaddsub_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fmaddsub_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (1. , 0. , 3. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fmaddsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fmaddsub_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fmaddsub_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fmaddsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fmaddsub_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fmaddsub_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fmaddsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fmaddsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fmaddsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fmaddsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fmaddsub_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fmaddsub_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmsubadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fmsubadd_pd (a , b , c) ; let e = _mm512_setr_pd (1. , 0. , 3. , 2. , 5. , 4. , 7. , 6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmsubadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fmsubadd_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmsubadd_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (1. , 0. , 3. , 2. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmsubadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fmsubadd_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmsubadd_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (1. , 0. , 3. , 2. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmsubadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fmsubadd_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmsubadd_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (1. , 0. , 3. , 2. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fmsubadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fmsubadd_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fmsubadd_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (- 1. , 2. , 1. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fmsubadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fmsubadd_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fmsubadd_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (- 1. , 2. , 1. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fmsubadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fmsubadd_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fmsubadd_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (- 1. , 2. , 1. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fmsubadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fmsubadd_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fmsubadd_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (- 1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fmsubadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fmsubadd_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fmsubadd_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (- 1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fmsubadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fmsubadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fmsubadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fmsubadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fmsubadd_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fmsubadd_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (- 1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fnmadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fnmadd_pd (a , b , c) ; let e = _mm512_setr_pd (1. , 0. , - 1. , - 2. , - 3. , - 4. , - 5. , - 6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fnmadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fnmadd_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fnmadd_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (1. , 0. , - 1. , - 2. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fnmadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fnmadd_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fnmadd_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (1. , 0. , - 1. , - 2. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fnmadd_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fnmadd_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fnmadd_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (1. , 0. , - 1. , - 2. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fnmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fnmadd_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fnmadd_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (1. , 0. , - 1. , - 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fnmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fnmadd_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fnmadd_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (1. , 0. , - 1. , - 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fnmadd_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fnmadd_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fnmadd_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (1. , 0. , - 1. , - 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fnmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fnmadd_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fnmadd_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fnmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fnmadd_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fnmadd_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fnmadd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fnmadd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fnmadd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fnmadd_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fnmadd_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fnmadd_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fnmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fnmsub_pd (a , b , c) ; let e = _mm512_setr_pd (- 1. , - 2. , - 3. , - 4. , - 5. , - 6. , - 7. , - 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fnmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fnmsub_pd (a , 0 , b , c) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fnmsub_pd (a , 0b00001111 , b , c) ; let e = _mm512_setr_pd (- 1. , - 2. , - 3. , - 4. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fnmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fnmsub_pd (0 , a , b , c) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fnmsub_pd (0b00001111 , a , b , c) ; let e = _mm512_setr_pd (- 1. , - 2. , - 3. , - 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fnmsub_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let c = _mm512_setr_pd (1. , 1. , 1. , 1. , 2. , 2. , 2. , 2.) ; let r = _mm512_mask3_fnmsub_pd (a , b , c , 0) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fnmsub_pd (a , b , c , 0b00001111) ; let e = _mm512_setr_pd (- 1. , - 2. , - 3. , - 4. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fnmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask_fnmsub_pd (a , 0 , b , c) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_fnmsub_pd (a , 0b00001111 , b , c) ; let e = _mm256_set_pd (- 1. , - 2. , - 3. , - 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fnmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_maskz_fnmsub_pd (0 , a , b , c) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_fnmsub_pd (0b00001111 , a , b , c) ; let e = _mm256_set_pd (- 1. , - 2. , - 3. , - 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask3_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask3_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask3_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask3_fnmsub_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set_pd (0. , 1. , 2. , 3.) ; let c = _mm256_set1_pd (1.) ; let r = _mm256_mask3_fnmsub_pd (a , b , c , 0) ; assert_eq_m256d (r , c) ; let r = _mm256_mask3_fnmsub_pd (a , b , c , 0b00001111) ; let e = _mm256_set_pd (- 1. , - 2. , - 3. , - 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fnmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask_fnmsub_pd (a , 0 , b , c) ; assert_eq_m128d (r , a) ; let r = _mm_mask_fnmsub_pd (a , 0b00000011 , b , c) ; let e = _mm_set_pd (- 1. , - 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fnmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_maskz_fnmsub_pd (0 , a , b , c) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_fnmsub_pd (0b00000011 , a , b , c) ; let e = _mm_set_pd (- 1. , - 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask3_fnmsub_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask3_fnmsub_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask3_fnmsub_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask3_fnmsub_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set_pd (0. , 1.) ; let c = _mm_set1_pd (1.) ; let r = _mm_mask3_fnmsub_pd (a , b , c , 0) ; assert_eq_m128d (r , c) ; let r = _mm_mask3_fnmsub_pd (a , b , c , 0b00000011) ; let e = _mm_set_pd (- 1. , - 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_rcp14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_rcp14_pd (a) ; let e = _mm512_set1_pd (0.3333320617675781) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_rcp14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_mask_rcp14_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_rcp14_pd (a , 0b11110000 , a) ; #[rustfmt :: skip] let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 0.3333320617675781 , 0.3333320617675781 , 0.3333320617675781 , 0.3333320617675781 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_rcp14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_maskz_rcp14_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_rcp14_pd (0b11110000 , a) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 0.3333320617675781 , 0.3333320617675781 , 0.3333320617675781 , 0.3333320617675781 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_rcp14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_rcp14_pd (a) ; let e = _mm256_set1_pd (0.3333320617675781) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_rcp14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_mask_rcp14_pd (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_rcp14_pd (a , 0b00001111 , a) ; let e = _mm256_set1_pd (0.3333320617675781) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_rcp14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_maskz_rcp14_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_rcp14_pd (0b00001111 , a) ; let e = _mm256_set1_pd (0.3333320617675781) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_rcp14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_rcp14_pd (a) ; let e = _mm_set1_pd (0.3333320617675781) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_rcp14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_mask_rcp14_pd (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_rcp14_pd (a , 0b00000011 , a) ; let e = _mm_set1_pd (0.3333320617675781) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_rcp14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_rcp14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_rcp14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_rcp14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_maskz_rcp14_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_rcp14_pd (0b00000011 , a) ; let e = _mm_set1_pd (0.3333320617675781) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_rsqrt14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_rsqrt14_pd (a) ; let e = _mm512_set1_pd (0.5773391723632813) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_rsqrt14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_mask_rsqrt14_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_rsqrt14_pd (a , 0b11110000 , a) ; #[rustfmt :: skip] let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 0.5773391723632813 , 0.5773391723632813 , 0.5773391723632813 , 0.5773391723632813 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_rsqrt14_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_maskz_rsqrt14_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_rsqrt14_pd (0b11110000 , a) ; #[rustfmt :: skip] let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 0.5773391723632813 , 0.5773391723632813 , 0.5773391723632813 , 0.5773391723632813 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_rsqrt14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_rsqrt14_pd (a) ; let e = _mm256_set1_pd (0.5773391723632813) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_rsqrt14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_mask_rsqrt14_pd (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_rsqrt14_pd (a , 0b00001111 , a) ; let e = _mm256_set1_pd (0.5773391723632813) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_rsqrt14_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_maskz_rsqrt14_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_rsqrt14_pd (0b00001111 , a) ; let e = _mm256_set1_pd (0.5773391723632813) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_rsqrt14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_rsqrt14_pd (a) ; let e = _mm_set1_pd (0.5773391723632813) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_rsqrt14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_mask_rsqrt14_pd (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_rsqrt14_pd (a , 0b00000011 , a) ; let e = _mm_set1_pd (0.5773391723632813) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_rsqrt14_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_rsqrt14_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_rsqrt14_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_rsqrt14_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_maskz_rsqrt14_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_rsqrt14_pd (0b00000011 , a) ; let e = _mm_set1_pd (0.5773391723632813) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_getexp_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_getexp_pd (a) ; let e = _mm512_set1_pd (1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_getexp_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_mask_getexp_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_getexp_pd (a , 0b11110000 , a) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_getexp_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_maskz_getexp_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_getexp_pd (0b11110000 , a) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_getexp_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_getexp_pd (a) ; let e = _mm256_set1_pd (1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_getexp_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_mask_getexp_pd (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_getexp_pd (a , 0b00001111 , a) ; let e = _mm256_set1_pd (1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_getexp_pd () { let a = _mm256_set1_pd (3.) ; let r = _mm256_maskz_getexp_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_getexp_pd (0b00001111 , a) ; let e = _mm256_set1_pd (1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_getexp_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_getexp_pd (a) ; let e = _mm_set1_pd (1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_getexp_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_mask_getexp_pd (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_getexp_pd (a , 0b00000011 , a) ; let e = _mm_set1_pd (1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_getexp_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_getexp_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_getexp_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_getexp_pd () { let a = _mm_set1_pd (3.) ; let r = _mm_maskz_getexp_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_getexp_pd (0b00000011 , a) ; let e = _mm_set1_pd (1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_roundscale_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_roundscale_pd :: < 0b00_00_00_00 > (a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_roundscale_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0 , a) ; let e = _mm512_set1_pd (1.1) ; assert_eq_m512d (r , e) ; let r = _mm512_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0b11111111 , a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_roundscale_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_maskz_roundscale_pd :: < 0b00_00_00_00 > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_roundscale_pd :: < 0b00_00_00_00 > (0b11111111 , a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_roundscale_pd () { let a = _mm256_set1_pd (1.1) ; let r = _mm256_roundscale_pd :: < 0b00_00_00_00 > (a) ; let e = _mm256_set1_pd (1.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_roundscale_pd () { let a = _mm256_set1_pd (1.1) ; let r = _mm256_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0b00001111 , a) ; let e = _mm256_set1_pd (1.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_roundscale_pd () { let a = _mm256_set1_pd (1.1) ; let r = _mm256_maskz_roundscale_pd :: < 0b00_00_00_00 > (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_roundscale_pd :: < 0b00_00_00_00 > (0b00001111 , a) ; let e = _mm256_set1_pd (1.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_roundscale_pd () { let a = _mm_set1_pd (1.1) ; let r = _mm_roundscale_pd :: < 0b00_00_00_00 > (a) ; let e = _mm_set1_pd (1.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_roundscale_pd () { let a = _mm_set1_pd (1.1) ; let r = _mm_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0 , a) ; let e = _mm_set1_pd (1.1) ; assert_eq_m128d (r , e) ; let r = _mm_mask_roundscale_pd :: < 0b00_00_00_00 > (a , 0b00000011 , a) ; let e = _mm_set1_pd (1.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_roundscale_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_roundscale_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_roundscale_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_roundscale_pd () { let a = _mm_set1_pd (1.1) ; let r = _mm_maskz_roundscale_pd :: < 0b00_00_00_00 > (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_roundscale_pd :: < 0b00_00_00_00 > (0b00000011 , a) ; let e = _mm_set1_pd (1.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_scalef_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_scalef_pd (a , b) ; let e = _mm512_set1_pd (8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_scalef_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_mask_scalef_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_scalef_pd (a , 0b11110000 , a , b) ; let e = _mm512_set_pd (8. , 8. , 8. , 8. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_scalef_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_maskz_scalef_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_scalef_pd (0b11110000 , a , b) ; let e = _mm512_set_pd (8. , 8. , 8. , 8. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_scalef_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set1_pd (3.) ; let r = _mm256_scalef_pd (a , b) ; let e = _mm256_set1_pd (8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_scalef_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set1_pd (3.) ; let r = _mm256_mask_scalef_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_scalef_pd (a , 0b00001111 , a , b) ; let e = _mm256_set1_pd (8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_scalef_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set1_pd (3.) ; let r = _mm256_maskz_scalef_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_scalef_pd (0b00001111 , a , b) ; let e = _mm256_set1_pd (8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_scalef_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set1_pd (3.) ; let r = _mm_scalef_pd (a , b) ; let e = _mm_set1_pd (8.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_scalef_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set1_pd (3.) ; let r = _mm_mask_scalef_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_scalef_pd (a , 0b00000011 , a , b) ; let e = _mm_set1_pd (8.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_scalef_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_scalef_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_scalef_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_scalef_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set1_pd (3.) ; let r = _mm_maskz_scalef_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_scalef_pd (0b00000011 , a , b) ; let e = _mm_set1_pd (8.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fixupimm_pd () { let a = _mm512_set1_pd (f64 :: NAN) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_fixupimm_pd :: < 5 > (a , b , c) ; let e = _mm512_set1_pd (0.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fixupimm_pd () { let a = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , 1. , 1. , 1. , 1.) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_mask_fixupimm_pd :: < 5 > (a , 0b11110000 , b , c) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fixupimm_pd () { let a = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , 1. , 1. , 1. , 1.) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_maskz_fixupimm_pd :: < 5 > (0b11110000 , a , b , c) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_fixupimm_pd () { let a = _mm256_set1_pd (f64 :: NAN) ; let b = _mm256_set1_pd (f64 :: MAX) ; let c = _mm256_set1_epi64x (i32 :: MAX as i64) ; let r = _mm256_fixupimm_pd :: < 5 > (a , b , c) ; let e = _mm256_set1_pd (0.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_fixupimm_pd () { let a = _mm256_set1_pd (f64 :: NAN) ; let b = _mm256_set1_pd (f64 :: MAX) ; let c = _mm256_set1_epi64x (i32 :: MAX as i64) ; let r = _mm256_mask_fixupimm_pd :: < 5 > (a , 0b00001111 , b , c) ; let e = _mm256_set1_pd (0.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_fixupimm_pd () { let a = _mm256_set1_pd (f64 :: NAN) ; let b = _mm256_set1_pd (f64 :: MAX) ; let c = _mm256_set1_epi64x (i32 :: MAX as i64) ; let r = _mm256_maskz_fixupimm_pd :: < 5 > (0b00001111 , a , b , c) ; let e = _mm256_set1_pd (0.0) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_fixupimm_pd () { let a = _mm_set1_pd (f64 :: NAN) ; let b = _mm_set1_pd (f64 :: MAX) ; let c = _mm_set1_epi64x (i32 :: MAX as i64) ; let r = _mm_fixupimm_pd :: < 5 > (a , b , c) ; let e = _mm_set1_pd (0.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_fixupimm_pd () { let a = _mm_set1_pd (f64 :: NAN) ; let b = _mm_set1_pd (f64 :: MAX) ; let c = _mm_set1_epi64x (i32 :: MAX as i64) ; let r = _mm_mask_fixupimm_pd :: < 5 > (a , 0b00000011 , b , c) ; let e = _mm_set1_pd (0.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_fixupimm_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_fixupimm_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_fixupimm_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_fixupimm_pd () { let a = _mm_set1_pd (f64 :: NAN) ; let b = _mm_set1_pd (f64 :: MAX) ; let c = _mm_set1_epi64x (i32 :: MAX as i64) ; let r = _mm_maskz_fixupimm_pd :: < 5 > (0b00000011 , a , b , c) ; let e = _mm_set1_pd (0.0) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_ternarylogic_epi64 () { let a = _mm512_set1_epi64 (1 << 2) ; let b = _mm512_set1_epi64 (1 << 1) ; let c = _mm512_set1_epi64 (1 << 0) ; let r = _mm512_ternarylogic_epi64 :: < 8 > (a , b , c) ; let e = _mm512_set1_epi64 (0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_ternarylogic_epi64 () { let src = _mm512_set1_epi64 (1 << 2) ; let a = _mm512_set1_epi64 (1 << 1) ; let b = _mm512_set1_epi64 (1 << 0) ; let r = _mm512_mask_ternarylogic_epi64 :: < 8 > (src , 0 , a , b) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_ternarylogic_epi64 :: < 8 > (src , 0b11111111 , a , b) ; let e = _mm512_set1_epi64 (0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_ternarylogic_epi64 () { let a = _mm512_set1_epi64 (1 << 2) ; let b = _mm512_set1_epi64 (1 << 1) ; let c = _mm512_set1_epi64 (1 << 0) ; let r = _mm512_maskz_ternarylogic_epi64 :: < 8 > (0 , a , b , c) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_ternarylogic_epi64 :: < 8 > (0b11111111 , a , b , c) ; let e = _mm512_set1_epi64 (0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_ternarylogic_epi64 () { let a = _mm256_set1_epi64x (1 << 2) ; let b = _mm256_set1_epi64x (1 << 1) ; let c = _mm256_set1_epi64x (1 << 0) ; let r = _mm256_ternarylogic_epi64 :: < 8 > (a , b , c) ; let e = _mm256_set1_epi64x (0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_ternarylogic_epi64 () { let src = _mm256_set1_epi64x (1 << 2) ; let a = _mm256_set1_epi64x (1 << 1) ; let b = _mm256_set1_epi64x (1 << 0) ; let r = _mm256_mask_ternarylogic_epi64 :: < 8 > (src , 0 , a , b) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_ternarylogic_epi64 :: < 8 > (src , 0b00001111 , a , b) ; let e = _mm256_set1_epi64x (0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_ternarylogic_epi64 () { let a = _mm256_set1_epi64x (1 << 2) ; let b = _mm256_set1_epi64x (1 << 1) ; let c = _mm256_set1_epi64x (1 << 0) ; let r = _mm256_maskz_ternarylogic_epi64 :: < 9 > (0 , a , b , c) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_ternarylogic_epi64 :: < 8 > (0b00001111 , a , b , c) ; let e = _mm256_set1_epi64x (0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_ternarylogic_epi64 () { let a = _mm_set1_epi64x (1 << 2) ; let b = _mm_set1_epi64x (1 << 1) ; let c = _mm_set1_epi64x (1 << 0) ; let r = _mm_ternarylogic_epi64 :: < 8 > (a , b , c) ; let e = _mm_set1_epi64x (0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_ternarylogic_epi64 () { let src = _mm_set1_epi64x (1 << 2) ; let a = _mm_set1_epi64x (1 << 1) ; let b = _mm_set1_epi64x (1 << 0) ; let r = _mm_mask_ternarylogic_epi64 :: < 8 > (src , 0 , a , b) ; assert_eq_m128i (r , src) ; let r = _mm_mask_ternarylogic_epi64 :: < 8 > (src , 0b00000011 , a , b) ; let e = _mm_set1_epi64x (0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_ternarylogic_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_ternarylogic_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_ternarylogic_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_ternarylogic_epi64 () { let a = _mm_set1_epi64x (1 << 2) ; let b = _mm_set1_epi64x (1 << 1) ; let c = _mm_set1_epi64x (1 << 0) ; let r = _mm_maskz_ternarylogic_epi64 :: < 9 > (0 , a , b , c) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_ternarylogic_epi64 :: < 8 > (0b00000011 , a , b , c) ; let e = _mm_set1_epi64x (0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_getmant_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a) ; let e = _mm512_set1_pd (1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_getmant_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0b11110000 , a) ; let e = _mm512_setr_pd (10. , 10. , 10. , 10. , 1.25 , 1.25 , 1.25 , 1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_getmant_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0b11110000 , a) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1.25 , 1.25 , 1.25 , 1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_getmant_pd () { let a = _mm256_set1_pd (10.) ; let r = _mm256_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a) ; let e = _mm256_set1_pd (1.25) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_getmant_pd () { let a = _mm256_set1_pd (10.) ; let r = _mm256_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0b00001111 , a) ; let e = _mm256_set1_pd (1.25) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_getmant_pd () { let a = _mm256_set1_pd (10.) ; let r = _mm256_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0b00001111 , a) ; let e = _mm256_set1_pd (1.25) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_getmant_pd () { let a = _mm_set1_pd (10.) ; let r = _mm_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a) ; let e = _mm_set1_pd (1.25) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_getmant_pd () { let a = _mm_set1_pd (10.) ; let r = _mm_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (a , 0b00000011 , a) ; let e = _mm_set1_pd (1.25) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_getmant_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_getmant_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_getmant_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_getmant_pd () { let a = _mm_set1_pd (10.) ; let r = _mm_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_getmant_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC > (0b00000011 , a) ; let e = _mm_set1_pd (1.25) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_cvtps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtps_pd (a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm512_set1_pd (0.) ; let r = _mm512_mask_cvtps_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtps_pd (src , 0b00001111 , a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvtps_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_cvtps_pd (0b00001111 , a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvtpslo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpslo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpslo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtpslo_pd () { let v2 = _mm512_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5 , 100. , 100. , 100. , 100. , 100. , 100. , 100. , 100. ,) ; let r = _mm512_cvtpslo_pd (v2) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpslo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpslo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpslo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtpslo_pd () { let v2 = _mm512_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5 , 100. , 100. , 100. , 100. , 100. , 100. , 100. , 100. ,) ; let src = _mm512_set1_pd (0.) ; let r = _mm512_mask_cvtpslo_pd (src , 0 , v2) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtpslo_pd (src , 0b00001111 , v2) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtpd_ps (a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_ps (0.) ; let r = _mm512_mask_cvtpd_ps (src , 0 , a) ; assert_eq_m256 (r , src) ; let r = _mm512_mask_cvtpd_ps (src , 0b00001111 , a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvtpd_ps (0 , a) ; assert_eq_m256 (r , _mm256_setzero_ps ()) ; let r = _mm512_maskz_cvtpd_ps (0b00001111 , a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_mask_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtpd_ps () { let a = _mm256_set_pd (4. , - 5.5 , 6. , - 7.5) ; let src = _mm_set1_ps (0.) ; let r = _mm256_mask_cvtpd_ps (src , 0 , a) ; assert_eq_m128 (r , src) ; let r = _mm256_mask_cvtpd_ps (src , 0b00001111 , a) ; let e = _mm_set_ps (4. , - 5.5 , 6. , - 7.5) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtpd_ps () { let a = _mm256_set_pd (4. , - 5.5 , 6. , - 7.5) ; let r = _mm256_maskz_cvtpd_ps (0 , a) ; assert_eq_m128 (r , _mm_setzero_ps ()) ; let r = _mm256_maskz_cvtpd_ps (0b00001111 , a) ; let e = _mm_set_ps (4. , - 5.5 , 6. , - 7.5) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtpd_ps () { let a = _mm_set_pd (6. , - 7.5) ; let src = _mm_set1_ps (0.) ; let r = _mm_mask_cvtpd_ps (src , 0 , a) ; assert_eq_m128 (r , src) ; let r = _mm_mask_cvtpd_ps (src , 0b00000011 , a) ; let e = _mm_set_ps (0. , 0. , 6. , - 7.5) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_cvtpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtpd_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtpd_ps () { let a = _mm_set_pd (6. , - 7.5) ; let r = _mm_maskz_cvtpd_ps (0 , a) ; assert_eq_m128 (r , _mm_setzero_ps ()) ; let r = _mm_maskz_cvtpd_ps (0b00000011 , a) ; let e = _mm_set_ps (0. , 0. , 6. , - 7.5) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm512_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtpd_epi32 (a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 4 , - 6 , 6 , - 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvtpd_epi32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtpd_epi32 (src , 0b11111111 , a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 4 , - 6 , 6 , - 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvtpd_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtpd_epi32 (0b11111111 , a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 4 , - 6 , 6 , - 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtpd_epi32 () { let a = _mm256_set_pd (4. , - 5.5 , 6. , - 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvtpd_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtpd_epi32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (4 , - 6 , 6 , - 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtpd_epi32 () { let a = _mm256_set_pd (4. , - 5.5 , 6. , - 7.5) ; let r = _mm256_maskz_cvtpd_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtpd_epi32 (0b00001111 , a) ; let e = _mm_set_epi32 (4 , - 6 , 6 , - 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtpd_epi32 () { let a = _mm_set_pd (6. , - 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvtpd_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtpd_epi32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , - 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtpd_epi32 () { let a = _mm_set_pd (6. , - 7.5) ; let r = _mm_maskz_cvtpd_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtpd_epi32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , - 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtpd_epu32 () { let a = _mm512_setr_pd (0. , 1.5 , 2. , 3.5 , 4. , 5.5 , 6. , 7.5) ; let r = _mm512_cvtpd_epu32 (a) ; let e = _mm256_setr_epi32 (0 , 2 , 2 , 4 , 4 , 6 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtpd_epu32 () { let a = _mm512_setr_pd (0. , 1.5 , 2. , 3.5 , 4. , 5.5 , 6. , 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvtpd_epu32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtpd_epu32 (src , 0b11111111 , a) ; let e = _mm256_setr_epi32 (0 , 2 , 2 , 4 , 4 , 6 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtpd_epu32 () { let a = _mm512_setr_pd (0. , 1.5 , 2. , 3.5 , 4. , 5.5 , 6. , 7.5) ; let r = _mm512_maskz_cvtpd_epu32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtpd_epu32 (0b11111111 , a) ; let e = _mm256_setr_epi32 (0 , 2 , 2 , 4 , 4 , 6 , 6 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let r = _mm256_cvtpd_epu32 (a) ; let e = _mm_set_epi32 (4 , 6 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvtpd_epu32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtpd_epu32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (4 , 6 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let r = _mm256_maskz_cvtpd_epu32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtpd_epu32 (0b00001111 , a) ; let e = _mm_set_epi32 (4 , 6 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let r = _mm_cvtpd_epu32 (a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvtpd_epu32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtpd_epu32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let r = _mm_maskz_cvtpd_epu32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtpd_epu32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtpd_pslo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_pslo in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_pslo_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtpd_pslo () { let v2 = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtpd_pslo (v2) ; let e = _mm512_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5 , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_pslo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_pslo in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_pslo_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtpd_pslo () { let v2 = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm512_set1_ps (0.) ; let r = _mm512_mask_cvtpd_pslo (src , 0 , v2) ; assert_eq_m512 (r , src) ; let r = _mm512_mask_cvtpd_pslo (src , 0b00001111 , v2) ; let e = _mm512_setr_ps (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi8_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepi8_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepi8_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi8_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepi8_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepi8_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepi8_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepi8_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepi8_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set1_epi64x (- 1) ; let r = _mm_mask_cvtepi8_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi8_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepi8_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi8_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepu8_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepu8_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepu8_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepu8_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepu8_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepu8_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepu8_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepu8_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepu8_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set1_epi64x (- 1) ; let r = _mm_mask_cvtepu8_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepu8_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu8_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu8_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu8_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepu8_epi64 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepu8_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepu8_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi16_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepi16_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepi16_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi16_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepi16_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepi16_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepi16_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepi16_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepi16_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set1_epi64x (- 1) ; let r = _mm_mask_cvtepi16_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi16_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepi16_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi16_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepu16_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepu16_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepu16_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepu16_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepu16_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepu16_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepu16_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepu16_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepu16_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set1_epi64x (- 1) ; let r = _mm_mask_cvtepu16_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepu16_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu16_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu16_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu16_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepu16_epi64 () { let a = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepu16_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepu16_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi32_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepi32_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepi32_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi32_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepi32_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi32_epi64 () { let a = _mm_set_epi32 (8 , 9 , 10 , 11) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepi32_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepi32_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (8 , 9 , 10 , 11) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi32_epi64 () { let a = _mm_set_epi32 (8 , 9 , 10 , 11) ; let r = _mm256_maskz_cvtepi32_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepi32_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (8 , 9 , 10 , 11) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi32_epi64 () { let a = _mm_set_epi32 (8 , 9 , 10 , 11) ; let src = _mm_set1_epi64x (0) ; let r = _mm_mask_cvtepi32_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi32_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (10 , 11) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi32_epi64 () { let a = _mm_set_epi32 (8 , 9 , 10 , 11) ; let r = _mm_maskz_cvtepi32_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi32_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (10 , 11) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepu32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepu32_epi64 (a) ; let e = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepu32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_epi64 (- 1) ; let r = _mm512_mask_cvtepu32_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_cvtepu32_epi64 (src , 0b00001111 , a) ; let e = _mm512_set_epi64 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepu32_epi64 () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepu32_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_cvtepu32_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepu32_epi64 () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm256_set1_epi64x (- 1) ; let r = _mm256_mask_cvtepu32_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_cvtepu32_epi64 (src , 0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepu32_epi64 () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepu32_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_cvtepu32_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepu32_epi64 () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm_set1_epi64x (- 1) ; let r = _mm_mask_cvtepu32_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepu32_epi64 (src , 0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu32_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu32_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu32_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepu32_epi64 () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepu32_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepu32_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi32_pd (a) ; let e = _mm512_set_pd (8. , 9. , 10. , 11. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_cvtepi32_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtepi32_pd (src , 0b00001111 , a) ; let e = _mm512_set_pd (- 1. , - 1. , - 1. , - 1. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi32_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_cvtepi32_pd (0b00001111 , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm256_set1_pd (- 1.) ; let r = _mm256_mask_cvtepi32_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_cvtepi32_pd (src , 0b00001111 , a) ; let e = _mm256_set_pd (12. , 13. , 14. , 15.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepi32_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_cvtepi32_pd (0b00001111 , a) ; let e = _mm256_set_pd (12. , 13. , 14. , 15.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm_set1_pd (- 1.) ; let r = _mm_mask_cvtepi32_pd (src , 0 , a) ; assert_eq_m128d (r , src) ; let r = _mm_mask_cvtepi32_pd (src , 0b00000011 , a) ; let e = _mm_set_pd (14. , 15.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepi32_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_cvtepi32_pd (0b00000011 , a) ; let e = _mm_set_pd (14. , 15.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepu32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepu32_pd (a) ; let e = _mm512_set_pd (8. , 9. , 10. , 11. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepu32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_cvtepu32_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtepu32_pd (src , 0b00001111 , a) ; let e = _mm512_set_pd (- 1. , - 1. , - 1. , - 1. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepu32_pd () { let a = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepu32_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_cvtepu32_pd (0b00001111 , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm256_cvtepu32_pd (a) ; let e = _mm256_set_pd (12. , 13. , 14. , 15.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm256_set1_pd (- 1.) ; let r = _mm256_mask_cvtepu32_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_cvtepu32_pd (src , 0b00001111 , a) ; let e = _mm256_set_pd (12. , 13. , 14. , 15.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepu32_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_cvtepu32_pd (0b00001111 , a) ; let e = _mm256_set_pd (12. , 13. , 14. , 15.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm_cvtepu32_pd (a) ; let e = _mm_set_pd (14. , 15.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let src = _mm_set1_pd (- 1.) ; let r = _mm_mask_cvtepu32_pd (src , 0 , a) ; assert_eq_m128d (r , src) ; let r = _mm_mask_cvtepu32_pd (src , 0b00000011 , a) ; let e = _mm_set_pd (14. , 15.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu32_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu32_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu32_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepu32_pd () { let a = _mm_set_epi32 (12 , 13 , 14 , 15) ; let r = _mm_maskz_cvtepu32_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_cvtepu32_pd (0b00000011 , a) ; let e = _mm_set_pd (14. , 15.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_cvtepi32lo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi32lo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi32lo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi32lo_pd () { let a = _mm512_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi32lo_pd (a) ; let e = _mm512_set_pd (8. , 9. , 10. , 11. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi32lo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi32lo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi32lo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi32lo_pd () { let a = _mm512_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_cvtepi32lo_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtepi32lo_pd (src , 0b00001111 , a) ; let e = _mm512_set_pd (- 1. , - 1. , - 1. , - 1. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvtepu32lo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu32lo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu32lo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepu32lo_pd () { let a = _mm512_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepu32lo_pd (a) ; let e = _mm512_set_pd (8. , 9. , 10. , 11. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu32lo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu32lo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu32lo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepu32lo_pd () { let a = _mm512_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_cvtepu32lo_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvtepu32lo_pd (src , 0b00001111 , a) ; let e = _mm512_set_pd (- 1. , - 1. , - 1. , - 1. , 12. , 13. , 14. , 15.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi64_epi32 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi64_epi32 (a) ; let e = _mm256_set_epi32 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_epi32 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm256_set1_epi32 (- 1) ; let r = _mm512_mask_cvtepi64_epi32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtepi64_epi32 (src , 0b00001111 , a) ; let e = _mm256_set_epi32 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi64_epi32 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi64_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtepi64_epi32 (0b00001111 , a) ; let e = _mm256_set_epi32 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtepi64_epi32 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_cvtepi64_epi32 (a) ; let e = _mm_set_epi32 (1 , 2 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_epi32 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvtepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtepi64_epi32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (1 , 2 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi64_epi32 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_cvtepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtepi64_epi32 (0b00001111 , a) ; let e = _mm_set_epi32 (1 , 2 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtepi64_epi32 () { let a = _mm_set_epi64x (3 , 4) ; let r = _mm_cvtepi64_epi32 (a) ; let e = _mm_set_epi32 (0 , 0 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_epi32 () { let a = _mm_set_epi64x (3 , 4) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvtepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi64_epi32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi64_epi32 () { let a = _mm_set_epi64x (3 , 4) ; let r = _mm_maskz_cvtepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi64_epi32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi64_epi16 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi64_epi16 (a) ; let e = _mm_set_epi16 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_epi16 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set1_epi16 (- 1) ; let r = _mm512_mask_cvtepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtepi64_epi16 (src , 0b00001111 , a) ; let e = _mm_set_epi16 (- 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi64_epi16 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtepi64_epi16 (0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtepi64_epi16 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let r = _mm256_cvtepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_epi16 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let src = _mm_set1_epi16 (0) ; let r = _mm256_mask_cvtepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtepi64_epi16 (src , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi64_epi16 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtepi64_epi16 (0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtepi64_epi16 () { let a = _mm_set_epi64x (14 , 15) ; let r = _mm_cvtepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_epi16 () { let a = _mm_set_epi64x (14 , 15) ; let src = _mm_set1_epi16 (0) ; let r = _mm_mask_cvtepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi64_epi16 (src , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi64_epi16 () { let a = _mm_set_epi64x (14 , 15) ; let r = _mm_maskz_cvtepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi64_epi16 (0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtepi64_epi8 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_cvtepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_epi8 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let src = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_mask_cvtepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtepi64_epi8 (src , 0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtepi64_epi8 () { let a = _mm512_set_epi64 (8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm512_maskz_cvtepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtepi64_epi8 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let r = _mm256_cvtepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_epi8 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let src = _mm_set1_epi8 (0) ; let r = _mm256_mask_cvtepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtepi64_epi8 (src , 0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtepi64_epi8 () { let a = _mm256_set_epi64x (12 , 13 , 14 , 15) ; let r = _mm256_maskz_cvtepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 12 , 13 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtepi64_epi8 () { let a = _mm_set_epi64x (14 , 15) ; let r = _mm_cvtepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_epi8 () { let a = _mm_set_epi64x (14 , 15) ; let src = _mm_set1_epi8 (0) ; let r = _mm_mask_cvtepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtepi64_epi8 (src , 0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtepi64_epi8 () { let a = _mm_set_epi64x (14 , 15) ; let r = _mm_maskz_cvtepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtepi64_epi8 (0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 14 , 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtsepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_cvtsepi64_epi32 (a) ; let e = _mm256_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm256_set1_epi32 (- 1) ; let r = _mm512_mask_cvtsepi64_epi32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtsepi64_epi32 (src , 0b00001111 , a) ; let e = _mm256_set_epi32 (- 1 , - 1 , - 1 , - 1 , 4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtsepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_maskz_cvtsepi64_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtsepi64_epi32 (0b00001111 , a) ; let e = _mm256_set_epi32 (0 , 0 , 0 , 0 , 4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtsepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_cvtsepi64_epi32 (a) ; let e = _mm_set_epi32 (4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi32 (- 1) ; let r = _mm256_mask_cvtsepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtsepi64_epi32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtsepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_maskz_cvtsepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtsepi64_epi32 (0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtsepi64_epi32 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_cvtsepi64_epi32 (a) ; let e = _mm_set_epi32 (0 , 0 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_epi32 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvtsepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtsepi64_epi32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtsepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtsepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtsepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtsepi64_epi32 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_maskz_cvtsepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtsepi64_epi32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , i32 :: MIN , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtsepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_cvtsepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 1 , 2 , 3 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi16 (- 1) ; let r = _mm512_mask_cvtsepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtsepi64_epi16 (src , 0b00001111 , a) ; let e = _mm_set_epi16 (- 1 , - 1 , - 1 , - 1 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtsepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_maskz_cvtsepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtsepi64_epi16 (0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtsepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_cvtsepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi16 (0) ; let r = _mm256_mask_cvtsepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtsepi64_epi16 (src , 0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtsepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_maskz_cvtsepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtsepi64_epi16 (0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtsepi64_epi16 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_cvtsepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_epi16 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi16 (0) ; let r = _mm_mask_cvtsepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtsepi64_epi16 (src , 0b00000011 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtsepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtsepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtsepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtsepi64_epi16 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_maskz_cvtsepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtsepi64_epi16 (0b00000011 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , i16 :: MIN , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtsepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_cvtsepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_mask_cvtsepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtsepi64_epi8 (src , 0b00001111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , 4 , 5 , i8 :: MIN , i8 :: MAX ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtsepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm512_maskz_cvtsepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtsepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtsepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_cvtsepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi8 (0) ; let r = _mm256_mask_cvtsepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtsepi64_epi8 (src , 0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtsepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , i64 :: MIN , i64 :: MAX) ; let r = _mm256_maskz_cvtsepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtsepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtsepi64_epi8 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_cvtsepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_epi8 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let src = _mm_set1_epi8 (0) ; let r = _mm_mask_cvtsepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtsepi64_epi8 (src , 0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtsepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtsepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtsepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtsepi64_epi8 () { let a = _mm_set_epi64x (i64 :: MIN , i64 :: MAX) ; let r = _mm_maskz_cvtsepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtsepi64_epi8 (0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MIN , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtusepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_cvtusepi64_epi32 (a) ; let e = _mm256_set_epi32 (0 , 1 , 2 , 3 , 4 , 5 , - 1 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let src = _mm256_set1_epi32 (- 1) ; let r = _mm512_mask_cvtusepi64_epi32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtusepi64_epi32 (src , 0b00001111 , a) ; let e = _mm256_set_epi32 (- 1 , - 1 , - 1 , - 1 , 4 , 5 , - 1 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtusepi64_epi32 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_maskz_cvtusepi64_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtusepi64_epi32 (0b00001111 , a) ; let e = _mm256_set_epi32 (0 , 0 , 0 , 0 , 4 , 5 , - 1 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtusepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_cvtusepi64_epi32 (a) ; let e = _mm_set_epi32 (4 , 5 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvtusepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtusepi64_epi32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtusepi64_epi32 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_maskz_cvtusepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtusepi64_epi32 (0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtusepi64_epi32 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_cvtusepi64_epi32 (a) ; let e = _mm_set_epi32 (0 , 0 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_epi32 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvtusepi64_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtusepi64_epi32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtusepi64_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtusepi64_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtusepi64_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtusepi64_epi32 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_maskz_cvtusepi64_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtusepi64_epi32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtusepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_cvtusepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 1 , 2 , 3 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let src = _mm_set1_epi16 (- 1) ; let r = _mm512_mask_cvtusepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtusepi64_epi16 (src , 0b00001111 , a) ; let e = _mm_set_epi16 (- 1 , - 1 , - 1 , - 1 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtusepi64_epi16 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_maskz_cvtusepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtusepi64_epi16 (0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtusepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_cvtusepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let src = _mm_set1_epi16 (0) ; let r = _mm256_mask_cvtusepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtusepi64_epi16 (src , 0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtusepi64_epi16 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_maskz_cvtusepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtusepi64_epi16 (0b00001111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 4 , 5 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtusepi64_epi16 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_cvtusepi64_epi16 (a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_epi16 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let src = _mm_set1_epi16 (0) ; let r = _mm_mask_cvtusepi64_epi16 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtusepi64_epi16 (src , 0b00000011 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtusepi64_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtusepi64_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtusepi64_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtusepi64_epi16 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_maskz_cvtusepi64_epi16 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtusepi64_epi16 (0b00000011 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 6 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtusepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_cvtusepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let src = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_mask_cvtusepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm512_mask_cvtusepi64_epi8 (src , 0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtusepi64_epi8 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , i64 :: MIN , i64 :: MIN) ; let r = _mm512_maskz_cvtusepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm512_maskz_cvtusepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , - 1 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvtusepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_cvtusepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let src = _mm_set1_epi8 (0) ; let r = _mm256_mask_cvtusepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvtusepi64_epi8 (src , 0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvtusepi64_epi8 () { let a = _mm256_set_epi64x (4 , 5 , 6 , i64 :: MAX) ; let r = _mm256_maskz_cvtusepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvtusepi64_epi8 (0b00001111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 5 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvtusepi64_epi8 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_cvtusepi64_epi8 (a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_epi8 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let src = _mm_set1_epi8 (0) ; let r = _mm_mask_cvtusepi64_epi8 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvtusepi64_epi8 (src , 0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtusepi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtusepi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtusepi64_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvtusepi64_epi8 () { let a = _mm_set_epi64x (6 , i64 :: MAX) ; let r = _mm_maskz_cvtusepi64_epi8 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvtusepi64_epi8 (0b00000011 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , u8 :: MAX as i8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtt_roundpd_epi32 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 4 , - 5 , 6 , - 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvtt_roundpd_epi32 :: < _MM_FROUND_NO_EXC > (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtt_roundpd_epi32 :: < _MM_FROUND_NO_EXC > (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvtt_roundpd_epi32 :: < _MM_FROUND_NO_EXC > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtt_roundpd_epi32 :: < _MM_FROUND_NO_EXC > (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvtt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvtt_roundpd_epu32 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 4 , - 1 , 6 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvtt_roundpd_epu32 :: < _MM_FROUND_NO_EXC > (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvtt_roundpd_epu32 :: < _MM_FROUND_NO_EXC > (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvtt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvtt_roundpd_epu32 :: < _MM_FROUND_NO_EXC > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvtt_roundpd_epu32 :: < _MM_FROUND_NO_EXC > (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvttpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvttpd_epi32 (a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 4 , - 5 , 6 , - 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvttpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvttpd_epi32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvttpd_epi32 (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvttpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvttpd_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvttpd_epi32 (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 3 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvttpd_epi32 () { let a = _mm256_setr_pd (4. , - 5.5 , 6. , - 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvttpd_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvttpd_epi32 (src , 0b00001111 , a) ; let e = _mm_setr_epi32 (4 , - 5 , 6 , - 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvttpd_epi32 () { let a = _mm256_setr_pd (4. , - 5.5 , 6. , - 7.5) ; let r = _mm256_maskz_cvttpd_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvttpd_epi32 (0b00001111 , a) ; let e = _mm_setr_epi32 (4 , - 5 , 6 , - 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvttpd_epi32 () { let a = _mm_set_pd (6. , - 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvttpd_epi32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvttpd_epi32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , - 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttpd_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvttpd_epi32 () { let a = _mm_set_pd (6. , - 7.5) ; let r = _mm_maskz_cvttpd_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvttpd_epi32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , - 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvttpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvttpd_epu32 (a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 4 , - 1 , 6 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvttpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvttpd_epu32 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvttpd_epu32 (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvttpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvttpd_epu32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvttpd_epu32 (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cvttpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let r = _mm256_cvttpd_epu32 (a) ; let e = _mm_set_epi32 (4 , 5 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvttpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm256_mask_cvttpd_epu32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm256_mask_cvttpd_epu32 (src , 0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_cvttpd_epu32 () { let a = _mm256_set_pd (4. , 5.5 , 6. , 7.5) ; let r = _mm256_maskz_cvttpd_epu32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm256_maskz_cvttpd_epu32 (0b00001111 , a) ; let e = _mm_set_epi32 (4 , 5 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cvttpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let r = _mm_cvttpd_epu32 (a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvttpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let src = _mm_set1_epi32 (0) ; let r = _mm_mask_cvttpd_epu32 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_cvttpd_epu32 (src , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttpd_epu32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_cvttpd_epu32 () { let a = _mm_set_pd (6. , 7.5) ; let r = _mm_maskz_cvttpd_epu32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_cvttpd_epu32 (0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , 6 , 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_add_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_add_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_add_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_add_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (- 1.) ; let r = _mm512_add_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (7. , 8.5 , 9. , 10.5 , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; let r = _mm512_add_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (7. , 8.5 , 9. , 10.5 , 11. , 12.5 , 13. , - 0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_add_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_add_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_add_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_add_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_add_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a , b ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_add_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a , b ,) ; let e = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_add_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_add_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_add_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_add_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (- 1.) ; let r = _mm512_maskz_add_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_add_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a , b ,) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_sub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sub_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_sub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (7. , 8.5 , 9. , 10.5 , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; let r = _mm512_sub_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (7. , 8.5 , 9. , 10.5 , 11. , 12.5 , 13. , - 0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_sub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sub_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_mask_sub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a , b ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_sub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a , b ,) ; let e = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_sub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sub_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let r = _mm512_maskz_sub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_sub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a , b ,) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 11. , 12.5 , 13. , - 1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mul_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mul_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mul_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mul_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.) ; let b = _mm512_set1_pd (0.1) ; let r = _mm512_mul_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (0.8 , 0.9500000000000001 , 1. , 1.1500000000000001 , 1.2000000000000002 , 1.35 , 1.4000000000000001 , 0. ,) ; assert_eq_m512d (r , e) ; let r = _mm512_mul_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_setr_pd (0.8 , 0.95 , 1.0 , 1.15 , 1.2 , 1.3499999999999999 , 1.4 , 0.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_mul_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mul_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mul_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_mul_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.) ; let b = _mm512_set1_pd (0.1) ; let r = _mm512_mask_mul_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a , b ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_mul_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a , b ,) ; let e = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 1.2000000000000002 , 1.35 , 1.4000000000000001 , 0. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_mul_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mul_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mul_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_mul_round_pd () { let a = _mm512_setr_pd (8. , 9.5 , 10. , 11.5 , 12. , 13.5 , 14. , 0.) ; let b = _mm512_set1_pd (0.1) ; let r = _mm512_maskz_mul_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_mul_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a , b ,) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1.2000000000000002 , 1.35 , 1.4000000000000001 , 0. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_div_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_div_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_div_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_div_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_div_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_set1_pd (0.3333333333333333) ; assert_eq_m512d (r , e) ; let r = _mm512_div_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_set1_pd (0.3333333333333333) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_div_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_div_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_div_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_div_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_mask_div_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a , b ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_div_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a , b ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 0.3333333333333333 , 0.3333333333333333 , 0.3333333333333333 , 0.3333333333333333 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_div_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_div_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_div_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_div_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_maskz_div_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_div_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a , b ,) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 0.3333333333333333 , 0.3333333333333333 , 0.3333333333333333 , 0.3333333333333333 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_sqrt_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sqrt_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sqrt_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sqrt_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_sqrt_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set1_pd (1.7320508075688772) ; assert_eq_m512d (r , e) ; let r = _mm512_sqrt_round_pd :: < { _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set1_pd (1.7320508075688774) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_sqrt_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sqrt_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sqrt_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sqrt_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_mask_sqrt_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_sqrt_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a ,) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 1.7320508075688772 , 1.7320508075688772 , 1.7320508075688772 , 1.7320508075688772 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_sqrt_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sqrt_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sqrt_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sqrt_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_maskz_sqrt_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_sqrt_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a ,) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1.7320508075688772 , 1.7320508075688772 , 1.7320508075688772 , 1.7320508075688772 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (- 1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fmadd_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (- 0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_maskz_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask3_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , - 1. , - 1. , - 1. , - 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (- 1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fmsub_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (- 0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask3_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (- 1. , - 1. , - 1. , - 1. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fmaddsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmaddsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmaddsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmaddsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_setr_pd (1. , - 1. , 1. , - 1. , 1. , - 1. , 1. , - 1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fmaddsub_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_setr_pd (1. , - 0.9999999999999999 , 1. , - 0.9999999999999999 , 1. , - 0.9999999999999999 , 1. , - 0.9999999999999999 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmaddsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmaddsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmaddsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmaddsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (1. , - 1. , 1. , - 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmaddsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmaddsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmaddsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmaddsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_maskz_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (1. , - 1. , 1. , - 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmaddsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmaddsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmaddsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmaddsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask3_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmaddsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (1. , - 1. , 1. , - 1. , - 1. , - 1. , - 1. , - 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fmsubadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fmsubadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fmsubadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fmsubadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_setr_pd (- 1. , 1. , - 1. , 1. , - 1. , 1. , - 1. , 1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fmsubadd_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_setr_pd (- 0.9999999999999999 , 1. , - 0.9999999999999999 , 1. , - 0.9999999999999999 , 1. , - 0.9999999999999999 , 1. ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fmsubadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fmsubadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fmsubadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fmsubadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (- 1. , 1. , - 1. , 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fmsubadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fmsubadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fmsubadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fmsubadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_maskz_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (- 1. , 1. , - 1. , 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fmsubadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fmsubadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fmsubadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fmsubadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask3_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fmsubadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (- 1. , 1. , - 1. , 1. , - 1. , - 1. , - 1. , - 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fnmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fnmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fnmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fnmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fnmadd_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fnmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fnmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fnmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fnmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fnmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fnmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fnmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fnmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_maskz_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fnmadd_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fnmadd_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fnmadd_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fnmadd_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (1.) ; let r = _mm512_mask3_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fnmadd_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fnmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fnmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fnmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fnmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (1.) ; assert_eq_m512d (r , e) ; let r = _mm512_fnmsub_round_pd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b , c) ; let e = _mm512_set1_pd (0.9999999999999999) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fnmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fnmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fnmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fnmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , b , c ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b00001111 , b , c ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 , 0.000000000000000007 ,) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fnmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fnmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fnmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fnmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_maskz_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b , c ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b00001111 , a , b , c ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask3_fnmsub_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask3_fnmsub_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask3_fnmsub_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask3_fnmsub_round_pd () { let a = _mm512_set1_pd (0.000000000000000007) ; let b = _mm512_set1_pd (1.) ; let c = _mm512_set1_pd (- 1.) ; let r = _mm512_mask3_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0 ,) ; assert_eq_m512d (r , c) ; let r = _mm512_mask3_fnmsub_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b , c , 0b00001111 ,) ; let e = _mm512_setr_pd (1. , 1. , 1. , 1. , - 1. , - 1. , - 1. , - 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_max_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_max_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_max_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_max_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_max_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_max_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_max_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_max_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_max_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_mask_max_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_max_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_max_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_max_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_max_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_max_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_maskz_max_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_max_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a , b) ; let e = _mm512_setr_pd (7. , 6. , 5. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_min_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_min_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_min_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_min_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_min_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 3. , 2. , 1. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_min_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_min_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_min_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_min_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_mask_min_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_min_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_min_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_min_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_min_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_min_round_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_setr_pd (7. , 6. , 5. , 4. , 3. , 2. , 1. , 0.) ; let r = _mm512_maskz_min_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_min_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a , b) ; let e = _mm512_setr_pd (0. , 1. , 2. , 3. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_getexp_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_getexp_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_getexp_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_getexp_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_getexp_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm512_set1_pd (1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_getexp_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_getexp_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_getexp_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_getexp_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_mask_getexp_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_getexp_round_pd :: < _MM_FROUND_CUR_DIRECTION > (a , 0b11110000 , a) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_getexp_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_getexp_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_getexp_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_getexp_round_pd () { let a = _mm512_set1_pd (3.) ; let r = _mm512_maskz_getexp_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_getexp_round_pd :: < _MM_FROUND_CUR_DIRECTION > (0b11110000 , a) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_roundscale_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_roundscale_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_roundscale_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_roundscale_round_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_roundscale_round_pd :: < 0 , _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_roundscale_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_roundscale_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_roundscale_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_roundscale_round_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_mask_roundscale_round_pd :: < 0 , _MM_FROUND_CUR_DIRECTION > (a , 0 , a) ; let e = _mm512_set1_pd (1.1) ; assert_eq_m512d (r , e) ; let r = _mm512_mask_roundscale_round_pd :: < 0 , _MM_FROUND_CUR_DIRECTION > (a , 0b11111111 , a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_roundscale_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_roundscale_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_roundscale_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_roundscale_round_pd () { let a = _mm512_set1_pd (1.1) ; let r = _mm512_maskz_roundscale_round_pd :: < 0 , _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_roundscale_round_pd :: < 0 , _MM_FROUND_CUR_DIRECTION > (0b11111111 , a) ; let e = _mm512_set1_pd (1.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_scalef_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_scalef_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_scalef_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_scalef_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_scalef_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm512_set1_pd (8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_scalef_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_scalef_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_scalef_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_scalef_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_mask_scalef_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0 , a , b ,) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_scalef_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a , 0b11110000 , a , b ,) ; let e = _mm512_set_pd (8. , 8. , 8. , 8. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_scalef_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_scalef_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_scalef_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_scalef_round_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (3.) ; let r = _mm512_maskz_scalef_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0 , a , b ,) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_scalef_round_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b11110000 , a , b ,) ; let e = _mm512_set_pd (8. , 8. , 8. , 8. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_fixupimm_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fixupimm_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fixupimm_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_fixupimm_round_pd () { let a = _mm512_set1_pd (f64 :: NAN) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_fixupimm_round_pd :: < 5 , _MM_FROUND_CUR_DIRECTION > (a , b , c) ; let e = _mm512_set1_pd (0.0) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_fixupimm_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fixupimm_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fixupimm_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_fixupimm_round_pd () { let a = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , 1. , 1. , 1. , 1.) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_mask_fixupimm_round_pd :: < 5 , _MM_FROUND_CUR_DIRECTION > (a , 0b11110000 , b , c) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_fixupimm_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_fixupimm_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_fixupimm_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_fixupimm_round_pd () { let a = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , 1. , 1. , 1. , 1.) ; let b = _mm512_set1_pd (f64 :: MAX) ; let c = _mm512_set1_epi64 (i32 :: MAX as i64) ; let r = _mm512_maskz_fixupimm_round_pd :: < 5 , _MM_FROUND_CUR_DIRECTION > (0b11110000 , a , b , c) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_getmant_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_getmant_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_getmant_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_getmant_round_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_getmant_round_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC , _MM_FROUND_CUR_DIRECTION , > (a) ; let e = _mm512_set1_pd (1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_getmant_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_getmant_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_getmant_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_getmant_round_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_mask_getmant_round_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC , _MM_FROUND_CUR_DIRECTION , > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_getmant_round_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC , _MM_FROUND_CUR_DIRECTION , > (a , 0b11110000 , a) ; let e = _mm512_setr_pd (10. , 10. , 10. , 10. , 1.25 , 1.25 , 1.25 , 1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_getmant_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_getmant_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_getmant_round_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_getmant_round_pd () { let a = _mm512_set1_pd (10.) ; let r = _mm512_maskz_getmant_round_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC , _MM_FROUND_CUR_DIRECTION , > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_getmant_round_pd :: < _MM_MANT_NORM_1_2 , _MM_MANT_SIGN_SRC , _MM_FROUND_CUR_DIRECTION , > (0b11110000 , a) ; let e = _mm512_setr_pd (0. , 0. , 0. , 0. , 1.25 , 1.25 , 1.25 , 1.25) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvt_roundps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvt_roundps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvt_roundps_pd :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvt_roundps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm512_set1_pd (0.) ; let r = _mm512_mask_cvt_roundps_pd :: < _MM_FROUND_CUR_DIRECTION > (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_cvt_roundps_pd :: < _MM_FROUND_CUR_DIRECTION > (src , 0b00001111 , a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundps_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundps_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundps_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvt_roundps_pd () { let a = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvt_roundps_pd :: < _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_cvt_roundps_pd :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a) ; let e = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvt_roundpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvt_roundpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvt_roundpd_ps :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvt_roundpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_ps (0.) ; let r = _mm512_mask_cvt_roundpd_ps :: < _MM_FROUND_CUR_DIRECTION > (src , 0 , a) ; assert_eq_m256 (r , src) ; let r = _mm512_mask_cvt_roundpd_ps :: < _MM_FROUND_CUR_DIRECTION > (src , 0b00001111 , a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvt_roundpd_ps () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvt_roundpd_ps :: < _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m256 (r , _mm256_setzero_ps ()) ; let r = _mm512_maskz_cvt_roundpd_ps :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a) ; let e = _mm256_setr_ps (0. , - 1.5 , 2. , - 3.5 , 0. , 0. , 0. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_cvt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvt_roundpd_epi32 :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 4 , - 6 , 6 , - 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvt_roundpd_epi32 :: < _MM_FROUND_CUR_DIRECTION > (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvt_roundpd_epi32 :: < _MM_FROUND_CUR_DIRECTION > (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundpd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundpd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundpd_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvt_roundpd_epi32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvt_roundpd_epi32 :: < _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvt_roundpd_epi32 :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 2 , 2 , - 4 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cvt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_cvt_roundpd_epu32 :: < _MM_FROUND_CUR_DIRECTION > (a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 4 , - 1 , 6 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let src = _mm256_set1_epi32 (0) ; let r = _mm512_mask_cvt_roundpd_epu32 :: < _MM_FROUND_CUR_DIRECTION > (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_cvt_roundpd_epu32 :: < _MM_FROUND_CUR_DIRECTION > (src , 0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundpd_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundpd_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundpd_epu32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_cvt_roundpd_epu32 () { let a = _mm512_setr_pd (0. , - 1.5 , 2. , - 3.5 , 4. , - 5.5 , 6. , - 7.5) ; let r = _mm512_maskz_cvt_roundpd_epu32 :: < _MM_FROUND_CUR_DIRECTION > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_cvt_roundpd_epu32 :: < _MM_FROUND_CUR_DIRECTION > (0b00001111 , a) ; let e = _mm256_setr_epi32 (0 , - 1 , 2 , - 1 , 0 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_setzero_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_setzero_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_setzero_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_setzero_pd () { assert_eq_m512d (_mm512_setzero_pd () , _mm512_set1_pd (0.)) ; }
}

macro_rules! test_mm512_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_set1_epi64_introspect!();
    unsafe fn test_mm512_set1_epi64 () { let r = _mm512_set_epi64 (2 , 2 , 2 , 2 , 2 , 2 , 2 , 2) ; assert_eq_m512i (r , _mm512_set1_epi64 (2)) ; }
}

macro_rules! test_mm512_set1_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_set1_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_set1_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_set1_pd () { let expected = _mm512_set_pd (2. , 2. , 2. , 2. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (expected , _mm512_set1_pd (2.)) ; }
}

macro_rules! test_mm512_set4_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_set4_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_set4_epi64_introspect!();
    unsafe fn test_mm512_set4_epi64 () { let r = _mm512_set_epi64 (4 , 3 , 2 , 1 , 4 , 3 , 2 , 1) ; assert_eq_m512i (r , _mm512_set4_epi64 (4 , 3 , 2 , 1)) ; }
}

macro_rules! test_mm512_set4_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_set4_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_set4_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_set4_pd () { let r = _mm512_set_pd (4. , 3. , 2. , 1. , 4. , 3. , 2. , 1.) ; assert_eq_m512d (r , _mm512_set4_pd (4. , 3. , 2. , 1.)) ; }
}

macro_rules! test_mm512_setr4_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_setr4_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_setr4_epi64_introspect!();
    unsafe fn test_mm512_setr4_epi64 () { let r = _mm512_set_epi64 (4 , 3 , 2 , 1 , 4 , 3 , 2 , 1) ; assert_eq_m512i (r , _mm512_setr4_epi64 (1 , 2 , 3 , 4)) ; }
}

macro_rules! test_mm512_setr4_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_setr4_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_setr4_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_setr4_pd () { let r = _mm512_set_pd (4. , 3. , 2. , 1. , 4. , 3. , 2. , 1.) ; assert_eq_m512d (r , _mm512_setr4_pd (1. , 2. , 3. , 4.)) ; }
}

macro_rules! test_mm512_cmplt_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmplt_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmplt_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmplt_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let m = _mm512_cmplt_pd_mask (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmplt_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmplt_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmplt_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmplt_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01100110 ; let r = _mm512_mask_cmplt_pd_mask (mask , a , b) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm512_cmpnlt_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpnlt_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpnlt_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpnlt_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; assert_eq ! (_mm512_cmpnlt_pd_mask (a , b) , ! _mm512_cmplt_pd_mask (a , b)) ; }
}

macro_rules! test_mm512_mask_cmpnlt_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpnlt_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpnlt_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpnlt_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01111010 ; assert_eq ! (_mm512_mask_cmpnlt_pd_mask (mask , a , b) , 0b01111010) ; }
}

macro_rules! test_mm512_cmple_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmple_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmple_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmple_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; assert_eq ! (_mm512_cmple_pd_mask (a , b) , 0b00100101) ; }
}

macro_rules! test_mm512_mask_cmple_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmple_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmple_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmple_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01111010 ; assert_eq ! (_mm512_mask_cmple_pd_mask (mask , a , b) , 0b00100000) ; }
}

macro_rules! test_mm512_cmpnle_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpnle_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpnle_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpnle_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let m = _mm512_cmpnle_pd_mask (b , a) ; assert_eq ! (m , 0b00001101) ; }
}

macro_rules! test_mm512_mask_cmpnle_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpnle_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpnle_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpnle_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , f64 :: MAX , f64 :: NAN , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01100110 ; let r = _mm512_mask_cmpnle_pd_mask (mask , b , a) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm512_cmpeq_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpeq_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpeq_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpeq_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let b = _mm512_set_pd (0. , 1. , 13. , 42. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let m = _mm512_cmpeq_pd_mask (b , a) ; assert_eq ! (m , 0b11001101) ; }
}

macro_rules! test_mm512_mask_cmpeq_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpeq_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpeq_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpeq_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let b = _mm512_set_pd (0. , 1. , 13. , 42. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpeq_pd_mask (mask , b , a) ; assert_eq ! (r , 0b01001000) ; }
}

macro_rules! test_mm512_cmpneq_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpneq_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpneq_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpneq_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let b = _mm512_set_pd (0. , 1. , 13. , 42. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let m = _mm512_cmpneq_pd_mask (b , a) ; assert_eq ! (m , 0b00110010) ; }
}

macro_rules! test_mm512_mask_cmpneq_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpneq_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpneq_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpneq_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let b = _mm512_set_pd (0. , 1. , 13. , 42. , f64 :: MAX , f64 :: MIN , f64 :: NAN , - 100.) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpneq_pd_mask (mask , b , a) ; assert_eq ! (r , 0b00110010) }
}

macro_rules! test_mm512_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmp_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let m = _mm512_cmp_pd_mask :: < _CMP_LT_OQ > (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmp_pd_mask () { let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01100110 ; let r = _mm512_mask_cmp_pd_mask :: < _CMP_LT_OQ > (mask , a , b) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm256_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmp_pd_mask () { let a = _mm256_set_pd (0. , 1. , - 1. , 13.) ; let b = _mm256_set1_pd (1.) ; let m = _mm256_cmp_pd_mask :: < _CMP_LT_OQ > (a , b) ; assert_eq ! (m , 0b00001010) ; }
}

macro_rules! test_mm256_mask_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmp_pd_mask () { let a = _mm256_set_pd (0. , 1. , - 1. , 13.) ; let b = _mm256_set1_pd (1.) ; let mask = 0b11111111 ; let r = _mm256_mask_cmp_pd_mask :: < _CMP_LT_OQ > (mask , a , b) ; assert_eq ! (r , 0b00001010) ; }
}

macro_rules! test_mm_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmp_pd_mask () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set1_pd (1.) ; let m = _mm_cmp_pd_mask :: < _CMP_LT_OQ > (a , b) ; assert_eq ! (m , 0b00000010) ; }
}

macro_rules! test_mm_mask_cmp_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmp_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmp_pd_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmp_pd_mask () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set1_pd (1.) ; let mask = 0b11111111 ; let r = _mm_mask_cmp_pd_mask :: < _CMP_LT_OQ > (mask , a , b) ; assert_eq ! (r , 0b00000010) ; }
}

macro_rules! test_mm512_cmp_round_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmp_round_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmp_round_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmp_round_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let m = _mm512_cmp_round_pd_mask :: < _CMP_LT_OQ , _MM_FROUND_CUR_DIRECTION > (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmp_round_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmp_round_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmp_round_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmp_round_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (0. , 1. , - 1. , 13. , f64 :: MAX , f64 :: MIN , 100. , - 100.) ; let b = _mm512_set1_pd (- 1.) ; let mask = 0b01100110 ; let r = _mm512_mask_cmp_round_pd_mask :: < _CMP_LT_OQ , _MM_FROUND_CUR_DIRECTION > (mask , a , b) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm512_cmpord_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpord_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpord_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpord_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (f64 :: NAN , f64 :: MAX , f64 :: NAN , f64 :: MIN , f64 :: NAN , - 1. , f64 :: NAN , 0.) ; #[rustfmt :: skip] let b = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: MIN , f64 :: MAX , - 1. , 0.) ; let m = _mm512_cmpord_pd_mask (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmpord_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpord_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpord_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpord_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (f64 :: NAN , f64 :: MAX , f64 :: NAN , f64 :: MIN , f64 :: NAN , - 1. , f64 :: NAN , 0.) ; #[rustfmt :: skip] let b = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: MIN , f64 :: MAX , - 1. , 0.) ; let mask = 0b11000011 ; let m = _mm512_mask_cmpord_pd_mask (mask , a , b) ; assert_eq ! (m , 0b00000001) ; }
}

macro_rules! test_mm512_cmpunord_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpunord_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpunord_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpunord_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (f64 :: NAN , f64 :: MAX , f64 :: NAN , f64 :: MIN , f64 :: NAN , - 1. , f64 :: NAN , 0.) ; #[rustfmt :: skip] let b = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: MIN , f64 :: MAX , - 1. , 0.) ; let m = _mm512_cmpunord_pd_mask (a , b) ; assert_eq ! (m , 0b11111010) ; }
}

macro_rules! test_mm512_mask_cmpunord_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpunord_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpunord_pd_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpunord_pd_mask () { #[rustfmt :: skip] let a = _mm512_set_pd (f64 :: NAN , f64 :: MAX , f64 :: NAN , f64 :: MIN , f64 :: NAN , - 1. , f64 :: NAN , 0.) ; #[rustfmt :: skip] let b = _mm512_set_pd (f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: NAN , f64 :: MIN , f64 :: MAX , - 1. , 0.) ; let mask = 0b00001111 ; let m = _mm512_mask_cmpunord_pd_mask (mask , a , b) ; assert_eq ! (m , 0b000001010) ; }
}

macro_rules! test_mm512_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmplt_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmplt_epu64_mask (a , b) ; assert_eq ! (m , 0b11001111) ; }
}

macro_rules! test_mm512_mask_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmplt_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01111010 ; let r = _mm512_mask_cmplt_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b01001010) ; }
}

macro_rules! test_mm256_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmplt_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 100) ; let b = _mm256_set1_epi64x (2) ; let r = _mm256_cmplt_epu64_mask (a , b) ; assert_eq ! (r , 0b00001100) ; }
}

macro_rules! test_mm256_mask_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmplt_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 100) ; let b = _mm256_set1_epi64x (2) ; let mask = 0b11111111 ; let r = _mm256_mask_cmplt_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00001100) ; }
}

macro_rules! test_mm_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmplt_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (2) ; let r = _mm_cmplt_epu64_mask (a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm_mask_cmplt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmplt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmplt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmplt_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (2) ; let mask = 0b11111111 ; let r = _mm_mask_cmplt_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm512_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpgt_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmpgt_epu64_mask (b , a) ; assert_eq ! (m , 0b11001111) ; }
}

macro_rules! test_mm512_mask_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpgt_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpgt_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b01001010) ; }
}

macro_rules! test_mm256_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpgt_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_cmpgt_epu64_mask (a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm256_mask_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpgt_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let b = _mm256_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpgt_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpgt_epu64_mask () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set1_epi64x (1) ; let r = _mm_cmpgt_epu64_mask (a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm_mask_cmpgt_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpgt_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpgt_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpgt_epu64_mask () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpgt_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm512_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmple_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; assert_eq ! (_mm512_cmple_epu64_mask (a , b) , ! _mm512_cmpgt_epu64_mask (a , b)) }
}

macro_rules! test_mm512_mask_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmple_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01111010 ; assert_eq ! (_mm512_mask_cmple_epu64_mask (mask , a , b) , 0b01111010) ; }
}

macro_rules! test_mm256_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmple_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 1) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_cmple_epu64_mask (a , b) ; assert_eq ! (r , 0b00001101) }
}

macro_rules! test_mm256_mask_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmple_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , 1) ; let b = _mm256_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmple_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00001101) }
}

macro_rules! test_mm_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmple_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let r = _mm_cmple_epu64_mask (a , b) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm_mask_cmple_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmple_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmple_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmple_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmple_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm512_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpge_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; assert_eq ! (_mm512_cmpge_epu64_mask (a , b) , ! _mm512_cmplt_epu64_mask (a , b)) ; }
}

macro_rules! test_mm512_mask_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpge_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b11111111 ; let r = _mm512_mask_cmpge_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00110000) ; }
}

macro_rules! test_mm256_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpge_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , u64 :: MAX as i64) ; let b = _mm256_set1_epi64x (1) ; let r = _mm256_cmpge_epu64_mask (a , b) ; assert_eq ! (r , 0b00000111) ; }
}

macro_rules! test_mm256_mask_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpge_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , 2 , u64 :: MAX as i64) ; let b = _mm256_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpge_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000111) ; }
}

macro_rules! test_mm_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpge_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let r = _mm_cmpge_epu64_mask (a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm_mask_cmpge_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpge_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpge_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpge_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpge_epu64_mask (mask , a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm512_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpeq_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let m = _mm512_cmpeq_epu64_mask (b , a) ; assert_eq ! (m , 0b11001111) ; }
}

macro_rules! test_mm512_mask_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpeq_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpeq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b01001010) ; }
}

macro_rules! test_mm256_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpeq_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , u64 :: MAX as i64) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let m = _mm256_cmpeq_epu64_mask (b , a) ; assert_eq ! (m , 0b00001100) ; }
}

macro_rules! test_mm256_mask_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpeq_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , u64 :: MAX as i64) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpeq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b00001100) ; }
}

macro_rules! test_mm_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpeq_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (0 , 1) ; let m = _mm_cmpeq_epu64_mask (b , a) ; assert_eq ! (m , 0b00000011) ; }
}

macro_rules! test_mm_mask_cmpeq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpeq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpeq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpeq_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (0 , 1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpeq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm512_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpneq_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let m = _mm512_cmpneq_epu64_mask (b , a) ; assert_eq ! (m , ! _mm512_cmpeq_epu64_mask (b , a)) ; }
}

macro_rules! test_mm512_mask_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpneq_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , - 100 , 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpneq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b00110010) ; }
}

macro_rules! test_mm256_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpneq_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , u64 :: MAX as i64) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let r = _mm256_cmpneq_epu64_mask (b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm256_mask_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpneq_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , u64 :: MAX as i64) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpneq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpneq_epu64_mask () { let a = _mm_set_epi64x (- 1 , u64 :: MAX as i64) ; let b = _mm_set_epi64x (13 , 42) ; let r = _mm_cmpneq_epu64_mask (b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm_mask_cmpneq_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpneq_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpneq_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpneq_epu64_mask () { let a = _mm_set_epi64x (- 1 , u64 :: MAX as i64) ; let b = _mm_set_epi64x (13 , 42) ; let mask = 0b11111111 ; let r = _mm_mask_cmpneq_epu64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm512_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmp_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmp_epu64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b11001111) ; }
}

macro_rules! test_mm512_mask_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmp_epu64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01111010 ; let r = _mm512_mask_cmp_epu64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b01001010) ; }
}

macro_rules! test_mm256_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmp_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 100) ; let b = _mm256_set1_epi64x (1) ; let m = _mm256_cmp_epu64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b00001000) ; }
}

macro_rules! test_mm256_mask_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmp_epu64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 100) ; let b = _mm256_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmp_epu64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b00001000) ; }
}

macro_rules! test_mm_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmp_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let m = _mm_cmp_epu64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b00000010) ; }
}

macro_rules! test_mm_mask_cmp_epu64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmp_epu64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmp_epu64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmp_epu64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmp_epu64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b00000010) ; }
}

macro_rules! test_mm512_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmplt_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmplt_epi64_mask (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmplt_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01100110 ; let r = _mm512_mask_cmplt_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm256_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmplt_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , - 13) ; let b = _mm256_set1_epi64x (- 1) ; let r = _mm256_cmplt_epi64_mask (a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm256_mask_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmplt_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , - 13) ; let b = _mm256_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmplt_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmplt_epi64_mask () { let a = _mm_set_epi64x (- 1 , - 13) ; let b = _mm_set1_epi64x (- 1) ; let r = _mm_cmplt_epi64_mask (a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm_mask_cmplt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmplt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmplt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmplt_epi64_mask () { let a = _mm_set_epi64x (- 1 , - 13) ; let b = _mm_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm_mask_cmplt_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000001) ; }
}

macro_rules! test_mm512_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpgt_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmpgt_epi64_mask (b , a) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpgt_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01100110 ; let r = _mm512_mask_cmpgt_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm256_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpgt_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set1_epi64x (- 1) ; let r = _mm256_cmpgt_epi64_mask (a , b) ; assert_eq ! (r , 0b00001101) ; }
}

macro_rules! test_mm256_mask_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpgt_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpgt_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00001101) ; }
}

macro_rules! test_mm_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpgt_epi64_mask () { let a = _mm_set_epi64x (0 , - 1) ; let b = _mm_set1_epi64x (- 1) ; let r = _mm_cmpgt_epi64_mask (a , b) ; assert_eq ! (r , 0b00000010) ; }
}

macro_rules! test_mm_mask_cmpgt_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpgt_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpgt_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpgt_epi64_mask () { let a = _mm_set_epi64x (0 , - 1) ; let b = _mm_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpgt_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000010) ; }
}

macro_rules! test_mm512_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmple_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; assert_eq ! (_mm512_cmple_epi64_mask (a , b) , ! _mm512_cmpgt_epi64_mask (a , b)) }
}

macro_rules! test_mm512_mask_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmple_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01111010 ; assert_eq ! (_mm512_mask_cmple_epi64_mask (mask , a , b) , 0b00110000) ; }
}

macro_rules! test_mm256_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmple_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , i64 :: MAX) ; let b = _mm256_set1_epi64x (- 1) ; let r = _mm256_cmple_epi64_mask (a , b) ; assert_eq ! (r , 0b00000010) }
}

macro_rules! test_mm256_mask_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmple_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , i64 :: MAX) ; let b = _mm256_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmple_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000010) }
}

macro_rules! test_mm_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmple_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let r = _mm_cmple_epi64_mask (a , b) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm_mask_cmple_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmple_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmple_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmple_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmple_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm512_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpge_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; assert_eq ! (_mm512_cmpge_epi64_mask (a , b) , ! _mm512_cmplt_epi64_mask (a , b)) }
}

macro_rules! test_mm512_mask_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpge_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , u64 :: MAX as i64 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b11111111 ; let r = _mm512_mask_cmpge_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b11111010) ; }
}

macro_rules! test_mm256_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpge_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , i64 :: MAX) ; let b = _mm256_set1_epi64x (- 1) ; let r = _mm256_cmpge_epi64_mask (a , b) ; assert_eq ! (r , 0b00001111) ; }
}

macro_rules! test_mm256_mask_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpge_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , i64 :: MAX) ; let b = _mm256_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpge_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00001111) ; }
}

macro_rules! test_mm_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpge_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (- 1) ; let r = _mm_cmpge_epi64_mask (a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm_mask_cmpge_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpge_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpge_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpge_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (- 1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpge_epi64_mask (mask , a , b) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm512_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmpeq_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let m = _mm512_cmpeq_epi64_mask (b , a) ; assert_eq ! (m , 0b11001111) ; }
}

macro_rules! test_mm512_mask_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpeq_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpeq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b01001010) ; }
}

macro_rules! test_mm256_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpeq_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let m = _mm256_cmpeq_epi64_mask (b , a) ; assert_eq ! (m , 0b00001100) ; }
}

macro_rules! test_mm256_mask_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpeq_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpeq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00001100) ; }
}

macro_rules! test_mm_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpeq_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (0 , 1) ; let m = _mm_cmpeq_epi64_mask (b , a) ; assert_eq ! (m , 0b00000011) ; }
}

macro_rules! test_mm_mask_cmpeq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpeq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpeq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpeq_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set_epi64x (0 , 1) ; let mask = 0b11111111 ; let r = _mm_mask_cmpeq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) ; }
}

macro_rules! test_mm512_set_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_set_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_set_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_set_epi64 () { let r = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , _mm512_set_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0)) }
}

macro_rules! test_mm512_setr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_setr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_setr_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_setr_epi64 () { let r = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; assert_eq_m512i (r , _mm512_setr_epi64 (7 , 6 , 5 , 4 , 3 , 2 , 1 , 0)) }
}

macro_rules! test_mm512_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmpneq_epi64_mask_introspect!();
    unsafe fn test_mm512_cmpneq_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let m = _mm512_cmpneq_epi64_mask (b , a) ; assert_eq ! (m , ! _mm512_cmpeq_epi64_mask (b , a)) ; }
}

macro_rules! test_mm512_mask_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmpneq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmpneq_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , - 100 , 100) ; let b = _mm512_set_epi64 (0 , 1 , 13 , 42 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let mask = 0b01111010 ; let r = _mm512_mask_cmpneq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00110010) }
}

macro_rules! test_mm256_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmpneq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmpneq_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let r = _mm256_cmpneq_epi64_mask (b , a) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm256_mask_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmpneq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmpneq_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set_epi64x (0 , 1 , 13 , 42) ; let mask = 0b11111111 ; let r = _mm256_mask_cmpneq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmpneq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmpneq_epi64_mask () { let a = _mm_set_epi64x (- 1 , 13) ; let b = _mm_set_epi64x (13 , 42) ; let r = _mm_cmpneq_epi64_mask (b , a) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm_mask_cmpneq_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmpneq_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmpneq_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmpneq_epi64_mask () { let a = _mm_set_epi64x (- 1 , 13) ; let b = _mm_set_epi64x (13 , 42) ; let mask = 0b11111111 ; let r = _mm_mask_cmpneq_epi64_mask (mask , b , a) ; assert_eq ! (r , 0b00000011) }
}

macro_rules! test_mm512_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_cmp_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let m = _mm512_cmp_epi64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b00000101) ; }
}

macro_rules! test_mm512_mask_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cmp_epi64_mask () { let a = _mm512_set_epi64 (0 , 1 , - 1 , 13 , i64 :: MAX , i64 :: MIN , 100 , - 100) ; let b = _mm512_set1_epi64 (- 1) ; let mask = 0b01100110 ; let r = _mm512_mask_cmp_epi64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b00000100) ; }
}

macro_rules! test_mm256_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_cmp_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set1_epi64x (1) ; let m = _mm256_cmp_epi64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b00001010) ; }
}

macro_rules! test_mm256_mask_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cmp_epi64_mask () { let a = _mm256_set_epi64x (0 , 1 , - 1 , 13) ; let b = _mm256_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm256_mask_cmp_epi64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b00001010) ; }
}

macro_rules! test_mm_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_cmp_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let m = _mm_cmp_epi64_mask :: < _MM_CMPINT_LT > (a , b) ; assert_eq ! (m , 0b00000010) ; }
}

macro_rules! test_mm_mask_cmp_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cmp_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cmp_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cmp_epi64_mask () { let a = _mm_set_epi64x (0 , 1) ; let b = _mm_set1_epi64x (1) ; let mask = 0b11111111 ; let r = _mm_mask_cmp_epi64_mask :: < _MM_CMPINT_LT > (mask , a , b) ; assert_eq ! (r , 0b00000010) ; }
}

macro_rules! test_mm512_i32gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32gather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32gather_pd () { let arr : [f64 ; 128] = core :: array :: from_fn (| i | i as f64) ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i32gather_pd :: < 8 > (index , arr . as_ptr ()) ; assert_eq_m512d (r , _mm512_setr_pd (0. , 16. , 32. , 48. , 64. , 80. , 96. , 112.)) ; }
}

macro_rules! test_mm512_mask_i32gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32gather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32gather_pd () { let arr : [f64 ; 128] = core :: array :: from_fn (| i | i as f64) ; let src = _mm512_set1_pd (2.) ; let mask = 0b10101010 ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i32gather_pd :: < 8 > (src , mask , index , arr . as_ptr ()) ; assert_eq_m512d (r , _mm512_setr_pd (2. , 16. , 2. , 48. , 2. , 80. , 2. , 112.)) ; }
}

macro_rules! test_mm512_i64gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64gather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64gather_pd () { let arr : [f64 ; 128] = core :: array :: from_fn (| i | i as f64) ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i64gather_pd :: < 8 > (index , arr . as_ptr ()) ; assert_eq_m512d (r , _mm512_setr_pd (0. , 16. , 32. , 48. , 64. , 80. , 96. , 112.)) ; }
}

macro_rules! test_mm512_mask_i64gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64gather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64gather_pd () { let arr : [f64 ; 128] = core :: array :: from_fn (| i | i as f64) ; let src = _mm512_set1_pd (2.) ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i64gather_pd :: < 8 > (src , mask , index , arr . as_ptr ()) ; assert_eq_m512d (r , _mm512_setr_pd (2. , 16. , 2. , 48. , 2. , 80. , 2. , 112.)) ; }
}

macro_rules! test_mm512_i64gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64gather_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64gather_ps () { let arr : [f32 ; 128] = core :: array :: from_fn (| i | i as f32) ; #[rustfmt :: skip] let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i64gather_ps :: < 4 > (index , arr . as_ptr ()) ; assert_eq_m256 (r , _mm256_setr_ps (0. , 16. , 32. , 48. , 64. , 80. , 96. , 112.)) ; }
}

macro_rules! test_mm512_mask_i64gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64gather_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64gather_ps () { let arr : [f32 ; 128] = core :: array :: from_fn (| i | i as f32) ; let src = _mm256_set1_ps (2.) ; let mask = 0b10101010 ; #[rustfmt :: skip] let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i64gather_ps :: < 4 > (src , mask , index , arr . as_ptr ()) ; assert_eq_m256 (r , _mm256_setr_ps (2. , 16. , 2. , 48. , 2. , 80. , 2. , 112.)) ; }
}

macro_rules! test_mm512_i32gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32gather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32gather_epi64 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i32gather_epi64 :: < 8 > (index , arr . as_ptr ()) ; assert_eq_m512i (r , _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112)) ; }
}

macro_rules! test_mm512_mask_i32gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32gather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32gather_epi64 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let src = _mm512_set1_epi64 (2) ; let mask = 0b10101010 ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i32gather_epi64 :: < 8 > (src , mask , index , arr . as_ptr ()) ; assert_eq_m512i (r , _mm512_setr_epi64 (2 , 16 , 2 , 48 , 2 , 80 , 2 , 112)) ; }
}

macro_rules! test_mm512_i64gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64gather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64gather_epi64 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i64gather_epi64 :: < 8 > (index , arr . as_ptr ()) ; assert_eq_m512i (r , _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112)) ; }
}

macro_rules! test_mm512_mask_i64gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64gather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64gather_epi64 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let src = _mm512_set1_epi64 (2) ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i64gather_epi64 :: < 8 > (src , mask , index , arr . as_ptr ()) ; assert_eq_m512i (r , _mm512_setr_epi64 (2 , 16 , 2 , 48 , 2 , 80 , 2 , 112)) ; }
}

macro_rules! test_mm512_i64gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64gather_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64gather_epi32 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_i64gather_epi32 :: < 8 > (index , arr . as_ptr () as * const i32) ; assert_eq_m256i (r , _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112)) ; }
}

macro_rules! test_mm512_mask_i64gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64gather_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64gather_epi32 () { let mut arr = [0i64 ; 128] ; for i in 0 .. 128i64 { arr [i as usize] = i ; } let src = _mm256_set1_epi32 (2) ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let r = _mm512_mask_i64gather_epi32 :: < 8 > (src , mask , index , arr . as_ptr () as * const i32) ; assert_eq_m256i (r , _mm256_setr_epi32 (2 , 16 , 2 , 48 , 2 , 80 , 2 , 112)) ; }
}

macro_rules! test_mm512_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32scatter_pd () { let mut arr = [0f64 ; 128] ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_i32scatter_pd :: < 8 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0f64 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as f64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32scatter_pd () { let mut arr = [0f64 ; 128] ; let mask = 0b10101010 ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_mask_i32scatter_pd :: < 8 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0f64 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2. * (i + 1) as f64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64scatter_pd () { let mut arr = [0f64 ; 128] ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_i64scatter_pd :: < 8 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0f64 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as f64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64scatter_pd () { let mut arr = [0f64 ; 128] ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_mask_i64scatter_pd :: < 8 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0f64 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2. * (i + 1) as f64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64scatter_ps () { let mut arr = [0f32 ; 128] ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm256_setr_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_i64scatter_ps :: < 4 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0f32 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as f32 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64scatter_ps () { let mut arr = [0f32 ; 128] ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm256_setr_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; _mm512_mask_i64scatter_ps :: < 4 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0f32 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2. * (i + 1) as f32 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32scatter_epi64 () { let mut arr = [0i64 ; 128] ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_i32scatter_epi64 :: < 8 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0i64 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as i64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32scatter_epi64 () { let mut arr = [0i64 ; 128] ; let mask = 0b10101010 ; let index = _mm256_setr_epi32 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_mask_i32scatter_epi64 :: < 8 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0i64 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2 * (i + 1) as i64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64scatter_epi64 () { let mut arr = [0i64 ; 128] ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_i64scatter_epi64 :: < 8 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0i64 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as i64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64scatter_epi64 () { let mut arr = [0i64 ; 128] ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_mask_i64scatter_epi64 :: < 8 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0i64 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2 * (i + 1) as i64 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i64scatter_epi32 () { let mut arr = [0i32 ; 128] ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_i64scatter_epi32 :: < 4 > (arr . as_mut_ptr () , index , src) ; let mut expected = [0i32 ; 128] ; for i in 0 .. 8 { expected [i * 16] = (i + 1) as i32 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_mask_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i64scatter_epi32 () { let mut arr = [0i32 ; 128] ; let mask = 0b10101010 ; let index = _mm512_setr_epi64 (0 , 16 , 32 , 48 , 64 , 80 , 96 , 112) ; let src = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; _mm512_mask_i64scatter_epi32 :: < 4 > (arr . as_mut_ptr () , mask , index , src) ; let mut expected = [0i32 ; 128] ; for i in 0 .. 4 { expected [i * 32 + 16] = 2 * (i + 1) as i32 ; } assert_eq ! (& arr [..] , & expected [..] ,) ; }
}

macro_rules! test_mm512_i32logather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32logather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32logather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32logather_epi64 () { let base_addr : [i64 ; 8] = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_i32logather_epi64 :: < 8 > (vindex , base_addr . as_ptr ()) ; let expected = _mm512_setr_epi64 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 1) ; assert_eq_m512i (expected , r) ; }
}

macro_rules! test_mm512_mask_i32logather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32logather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32logather_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32logather_epi64 () { let base_addr : [i64 ; 8] = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8] ; let src = _mm512_setr_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_mask_i32logather_epi64 :: < 8 > (src , 0b01010101 , vindex , base_addr . as_ptr ()) ; let expected = _mm512_setr_epi64 (2 , 10 , 4 , 12 , 6 , 14 , 8 , 16) ; assert_eq_m512i (expected , r) ; }
}

macro_rules! test_mm512_i32logather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32logather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32logather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32logather_pd () { let base_addr : [f64 ; 8] = [1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_i32logather_pd :: < 8 > (vindex , base_addr . as_ptr ()) ; let expected = _mm512_setr_pd (2. , 3. , 4. , 5. , 6. , 7. , 8. , 1.) ; assert_eq_m512d (expected , r) ; }
}

macro_rules! test_mm512_mask_i32logather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32logather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32logather_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32logather_pd () { let base_addr : [f64 ; 8] = [1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.] ; let src = _mm512_setr_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_mask_i32logather_pd :: < 8 > (src , 0b01010101 , vindex , base_addr . as_ptr ()) ; let expected = _mm512_setr_pd (2. , 10. , 4. , 12. , 6. , 14. , 8. , 16.) ; assert_eq_m512d (expected , r) ; }
}

macro_rules! test_mm512_i32loscatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32loscatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32loscatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32loscatter_epi64 () { let mut base_addr : [i64 ; 8] = [0 ; 8] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let src = _mm512_setr_epi64 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 1) ; _mm512_i32loscatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm512_mask_i32loscatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32loscatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32loscatter_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32loscatter_epi64 () { let mut base_addr : [i64 ; 8] = [0 ; 8] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let src = _mm512_setr_epi64 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 1) ; _mm512_mask_i32loscatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , 0b01010101 , vindex , src) ; let expected = [0 , 2 , 0 , 4 , 0 , 6 , 0 , 8] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm512_i32loscatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_i32loscatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_i32loscatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_i32loscatter_pd () { let mut base_addr : [f64 ; 8] = [0. ; 8] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let src = _mm512_setr_pd (2. , 3. , 4. , 5. , 6. , 7. , 8. , 1.) ; _mm512_i32loscatter_pd :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm512_mask_i32loscatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_i32loscatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_i32loscatter_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_i32loscatter_pd () { let mut base_addr : [f64 ; 8] = [0. ; 8] ; let vindex = _mm512_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let src = _mm512_setr_pd (2. , 3. , 4. , 5. , 6. , 7. , 8. , 1.) ; _mm512_mask_i32loscatter_pd :: < 8 > (base_addr . as_mut_ptr () , 0b01010101 , vindex , src) ; let expected = [0. , 2. , 0. , 4. , 0. , 6. , 0. , 8.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mmask_i32gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i32gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i32gather_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i32gather_epi32 () { let base_addr : [i32 ; 4] = [1 , 2 , 3 , 4] ; let src = _mm_setr_epi32 (5 , 6 , 7 , 8) ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let r = _mm_mmask_i32gather_epi32 :: < 4 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_epi32 (2 , 6 , 4 , 8) ; assert_eq_m128i (expected , r) ; }
}

macro_rules! test_mm_mmask_i32gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i32gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i32gather_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i32gather_epi64 () { let base_addr : [i64 ; 2] = [1 , 2] ; let src = _mm_setr_epi64x (5 , 6) ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let r = _mm_mmask_i32gather_epi64 :: < 8 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_epi64x (2 , 6) ; assert_eq_m128i (expected , r) ; }
}

macro_rules! test_mm_mmask_i32gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i32gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i32gather_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i32gather_pd () { let base_addr : [f64 ; 2] = [1. , 2.] ; let src = _mm_setr_pd (5. , 6.) ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let r = _mm_mmask_i32gather_pd :: < 8 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_pd (2. , 6.) ; assert_eq_m128d (expected , r) ; }
}

macro_rules! test_mm_mmask_i32gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i32gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i32gather_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i32gather_ps () { let base_addr : [f32 ; 4] = [1. , 2. , 3. , 4.] ; let src = _mm_setr_ps (5. , 6. , 7. , 8.) ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let r = _mm_mmask_i32gather_ps :: < 4 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_ps (2. , 6. , 4. , 8.) ; assert_eq_m128 (expected , r) ; }
}

macro_rules! test_mm_mmask_i64gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i64gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i64gather_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i64gather_epi32 () { let base_addr : [i32 ; 2] = [1 , 2] ; let src = _mm_setr_epi32 (5 , 6 , 7 , 8) ; let vindex = _mm_setr_epi64x (1 , 0) ; let r = _mm_mmask_i64gather_epi32 :: < 4 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_epi32 (2 , 6 , 0 , 0) ; assert_eq_m128i (expected , r) ; }
}

macro_rules! test_mm_mmask_i64gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i64gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i64gather_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i64gather_epi64 () { let base_addr : [i64 ; 2] = [1 , 2] ; let src = _mm_setr_epi64x (5 , 6) ; let vindex = _mm_setr_epi64x (1 , 0) ; let r = _mm_mmask_i64gather_epi64 :: < 8 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_epi64x (2 , 6) ; assert_eq_m128i (expected , r) ; }
}

macro_rules! test_mm_mmask_i64gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i64gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i64gather_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i64gather_pd () { let base_addr : [f64 ; 2] = [1. , 2.] ; let src = _mm_setr_pd (5. , 6.) ; let vindex = _mm_setr_epi64x (1 , 0) ; let r = _mm_mmask_i64gather_pd :: < 8 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_pd (2. , 6.) ; assert_eq_m128d (expected , r) ; }
}

macro_rules! test_mm_mmask_i64gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mmask_i64gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mmask_i64gather_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mmask_i64gather_ps () { let base_addr : [f32 ; 2] = [1. , 2.] ; let src = _mm_setr_ps (5. , 6. , 7. , 8.) ; let vindex = _mm_setr_epi64x (1 , 0) ; let r = _mm_mmask_i64gather_ps :: < 4 > (src , 0b01 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_ps (2. , 6. , 0. , 0.) ; assert_eq_m128 (expected , r) ; }
}

macro_rules! test_mm256_mmask_i32gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i32gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i32gather_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i32gather_epi32 () { let base_addr : [i32 ; 8] = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8] ; let src = _mm256_setr_epi32 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let r = _mm256_mmask_i32gather_epi32 :: < 4 > (src , 0b01010101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_epi32 (2 , 10 , 4 , 12 , 6 , 14 , 8 , 16) ; assert_eq_m256i (expected , r) ; }
}

macro_rules! test_mm256_mmask_i32gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i32gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i32gather_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i32gather_epi64 () { let base_addr : [i64 ; 4] = [1 , 2 , 3 , 4] ; let src = _mm256_setr_epi64x (9 , 10 , 11 , 12) ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let r = _mm256_mmask_i32gather_epi64 :: < 8 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_epi64x (2 , 10 , 4 , 12) ; assert_eq_m256i (expected , r) ; }
}

macro_rules! test_mm256_mmask_i32gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i32gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i32gather_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i32gather_pd () { let base_addr : [f64 ; 4] = [1. , 2. , 3. , 4.] ; let src = _mm256_setr_pd (9. , 10. , 11. , 12.) ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 4) ; let r = _mm256_mmask_i32gather_pd :: < 8 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_pd (2. , 10. , 4. , 12.) ; assert_eq_m256d (expected , r) ; }
}

macro_rules! test_mm256_mmask_i32gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i32gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i32gather_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i32gather_ps () { let base_addr : [f32 ; 8] = [1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.] ; let src = _mm256_setr_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let r = _mm256_mmask_i32gather_ps :: < 4 > (src , 0b01010101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_ps (2. , 10. , 4. , 12. , 6. , 14. , 8. , 16.) ; assert_eq_m256 (expected , r) ; }
}

macro_rules! test_mm256_mmask_i64gather_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i64gather_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i64gather_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i64gather_epi32 () { let base_addr : [i32 ; 4] = [1 , 2 , 3 , 4] ; let src = _mm_setr_epi32 (9 , 10 , 11 , 12) ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let r = _mm256_mmask_i64gather_epi32 :: < 4 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_epi32 (2 , 10 , 4 , 12) ; assert_eq_m128i (expected , r) ; }
}

macro_rules! test_mm256_mmask_i64gather_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i64gather_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i64gather_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i64gather_epi64 () { let base_addr : [i64 ; 4] = [1 , 2 , 3 , 4] ; let src = _mm256_setr_epi64x (9 , 10 , 11 , 12) ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let r = _mm256_mmask_i64gather_epi64 :: < 8 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_epi64x (2 , 10 , 4 , 12) ; assert_eq_m256i (expected , r) ; }
}

macro_rules! test_mm256_mmask_i64gather_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i64gather_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i64gather_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i64gather_pd () { let base_addr : [f64 ; 4] = [1. , 2. , 3. , 4.] ; let src = _mm256_setr_pd (9. , 10. , 11. , 12.) ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let r = _mm256_mmask_i64gather_pd :: < 8 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm256_setr_pd (2. , 10. , 4. , 12.) ; assert_eq_m256d (expected , r) ; }
}

macro_rules! test_mm256_mmask_i64gather_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mmask_i64gather_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mmask_i64gather_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mmask_i64gather_ps () { let base_addr : [f32 ; 4] = [1. , 2. , 3. , 4.] ; let src = _mm_setr_ps (9. , 10. , 11. , 12.) ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let r = _mm256_mmask_i64gather_ps :: < 4 > (src , 0b0101 , vindex , base_addr . as_ptr ()) ; let expected = _mm_setr_ps (2. , 10. , 4. , 12.) ; assert_eq_m128 (expected , r) ; }
}

macro_rules! test_mm_i32scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i32scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i32scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i32scatter_epi32 () { let mut base_addr : [i32 ; 4] = [0 ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm_setr_epi32 (2 , 3 , 4 , 1) ; _mm_i32scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i32scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i32scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i32scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i32scatter_epi32 () { let mut base_addr : [i32 ; 4] = [0 ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm_setr_epi32 (2 , 3 , 4 , 1) ; _mm_mask_i32scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0 , 2 , 0 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i32scatter_epi64 () { let mut base_addr : [i64 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let src = _mm_setr_epi64x (2 , 1) ; _mm_i32scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i32scatter_epi64 () { let mut base_addr : [i64 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let src = _mm_setr_epi64x (2 , 1) ; _mm_mask_i32scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i32scatter_pd () { let mut base_addr : [f64 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let src = _mm_setr_pd (2. , 1.) ; _mm_i32scatter_pd :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i32scatter_pd () { let mut base_addr : [f64 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi32 (1 , 0 , - 1 , - 1) ; let src = _mm_setr_pd (2. , 1.) ; _mm_mask_i32scatter_pd :: < 8 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i32scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i32scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i32scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i32scatter_ps () { let mut base_addr : [f32 ; 4] = [0. ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm_setr_ps (2. , 3. , 4. , 1.) ; _mm_i32scatter_ps :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i32scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i32scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i32scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i32scatter_ps () { let mut base_addr : [f32 ; 4] = [0. ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm_setr_ps (2. , 3. , 4. , 1.) ; _mm_mask_i32scatter_ps :: < 4 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0. , 2. , 0. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i64scatter_epi32 () { let mut base_addr : [i32 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_epi32 (2 , 1 , - 1 , - 1) ; _mm_i64scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i64scatter_epi32 () { let mut base_addr : [i32 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_epi32 (2 , 1 , - 1 , - 1) ; _mm_mask_i64scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i64scatter_epi64 () { let mut base_addr : [i64 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_epi64x (2 , 1) ; _mm_i64scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i64scatter_epi64 () { let mut base_addr : [i64 ; 2] = [0 ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_epi64x (2 , 1) ; _mm_mask_i64scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0 , 2] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i64scatter_pd () { let mut base_addr : [f64 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_pd (2. , 1.) ; _mm_i64scatter_pd :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i64scatter_pd () { let mut base_addr : [f64 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_pd (2. , 1.) ; _mm_mask_i64scatter_pd :: < 8 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_i64scatter_ps () { let mut base_addr : [f32 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_ps (2. , 1. , - 1. , - 1.) ; _mm_i64scatter_ps :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm_mask_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_i64scatter_ps () { let mut base_addr : [f32 ; 2] = [0. ; 2] ; let vindex = _mm_setr_epi64x (1 , 0) ; let src = _mm_setr_ps (2. , 1. , - 1. , - 1.) ; _mm_mask_i64scatter_ps :: < 4 > (base_addr . as_mut_ptr () , 0b01 , vindex , src) ; let expected = [0. , 2.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i32scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i32scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i32scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i32scatter_epi32 () { let mut base_addr : [i32 ; 8] = [0 ; 8] ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let src = _mm256_setr_epi32 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 1) ; _mm256_i32scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i32scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i32scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i32scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i32scatter_epi32 () { let mut base_addr : [i32 ; 8] = [0 ; 8] ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let src = _mm256_setr_epi32 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 1) ; _mm256_mask_i32scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , 0b01010101 , vindex , src) ; let expected = [0 , 2 , 0 , 4 , 0 , 6 , 0 , 8] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i32scatter_epi64 () { let mut base_addr : [i64 ; 4] = [0 ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm256_setr_epi64x (2 , 3 , 4 , 1) ; _mm256_i32scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i32scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i32scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i32scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i32scatter_epi64 () { let mut base_addr : [i64 ; 4] = [0 ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm256_setr_epi64x (2 , 3 , 4 , 1) ; _mm256_mask_i32scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0 , 2 , 0 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i32scatter_pd () { let mut base_addr : [f64 ; 4] = [0. ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm256_setr_pd (2. , 3. , 4. , 1.) ; _mm256_i32scatter_pd :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i32scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i32scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i32scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i32scatter_pd () { let mut base_addr : [f64 ; 4] = [0. ; 4] ; let vindex = _mm_setr_epi32 (1 , 2 , 3 , 0) ; let src = _mm256_setr_pd (2. , 3. , 4. , 1.) ; _mm256_mask_i32scatter_pd :: < 8 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0. , 2. , 0. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i32scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i32scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i32scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i32scatter_ps () { let mut base_addr : [f32 ; 8] = [0. ; 8] ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let src = _mm256_setr_ps (2. , 3. , 4. , 5. , 6. , 7. , 8. , 1.) ; _mm256_i32scatter_ps :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i32scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i32scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i32scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i32scatter_ps () { let mut base_addr : [f32 ; 8] = [0. ; 8] ; let vindex = _mm256_setr_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 0) ; let src = _mm256_setr_ps (2. , 3. , 4. , 5. , 6. , 7. , 8. , 1.) ; _mm256_mask_i32scatter_ps :: < 4 > (base_addr . as_mut_ptr () , 0b01010101 , vindex , src) ; let expected = [0. , 2. , 0. , 4. , 0. , 6. , 0. , 8.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i64scatter_epi32 () { let mut base_addr : [i32 ; 4] = [0 ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm_setr_epi32 (2 , 3 , 4 , 1) ; _mm256_i64scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i64scatter_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i64scatter_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i64scatter_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i64scatter_epi32 () { let mut base_addr : [i32 ; 4] = [0 ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm_setr_epi32 (2 , 3 , 4 , 1) ; _mm256_mask_i64scatter_epi32 :: < 4 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0 , 2 , 0 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i64scatter_epi64 () { let mut base_addr : [i64 ; 4] = [0 ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm256_setr_epi64x (2 , 3 , 4 , 1) ; _mm256_i64scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1 , 2 , 3 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i64scatter_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i64scatter_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i64scatter_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i64scatter_epi64 () { let mut base_addr : [i64 ; 4] = [0 ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm256_setr_epi64x (2 , 3 , 4 , 1) ; _mm256_mask_i64scatter_epi64 :: < 8 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0 , 2 , 0 , 4] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i64scatter_pd () { let mut base_addr : [f64 ; 4] = [0. ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm256_setr_pd (2. , 3. , 4. , 1.) ; _mm256_i64scatter_pd :: < 8 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i64scatter_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i64scatter_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i64scatter_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i64scatter_pd () { let mut base_addr : [f64 ; 4] = [0. ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm256_setr_pd (2. , 3. , 4. , 1.) ; _mm256_mask_i64scatter_pd :: < 8 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0. , 2. , 0. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_i64scatter_ps () { let mut base_addr : [f32 ; 4] = [0. ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm_setr_ps (2. , 3. , 4. , 1.) ; _mm256_i64scatter_ps :: < 4 > (base_addr . as_mut_ptr () , vindex , src) ; let expected = [1. , 2. , 3. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm256_mask_i64scatter_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_i64scatter_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_i64scatter_ps_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_i64scatter_ps () { let mut base_addr : [f32 ; 4] = [0. ; 4] ; let vindex = _mm256_setr_epi64x (1 , 2 , 3 , 0) ; let src = _mm_setr_ps (2. , 3. , 4. , 1.) ; _mm256_mask_i64scatter_ps :: < 4 > (base_addr . as_mut_ptr () , 0b0101 , vindex , src) ; let expected = [0. , 2. , 0. , 4.] ; assert_eq ! (expected , base_addr) ; }
}

macro_rules! test_mm512_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_rol_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_rol_epi64 :: < 1 > (a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_rol_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_mask_rol_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_rol_epi64 :: < 1 > (a , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_rol_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 63 ,) ; let r = _mm512_maskz_rol_epi64 :: < 1 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_rol_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_rol_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_rol_epi64 :: < 1 > (a) ; let e = _mm256_set_epi64x (1 << 0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_rol_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_mask_rol_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_rol_epi64 :: < 1 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_rol_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_maskz_rol_epi64 :: < 1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_rol_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_rol_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let r = _mm_rol_epi64 :: < 1 > (a) ; let e = _mm_set_epi64x (1 << 0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_rol_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let r = _mm_mask_rol_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_rol_epi64 :: < 1 > (a , 0b00000011 , a) ; let e = _mm_set_epi64x (1 << 0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_rol_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_rol_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_rol_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_rol_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let r = _mm_maskz_rol_epi64 :: < 1 > (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_rol_epi64 :: < 1 > (0b00000011 , a) ; let e = _mm_set_epi64x (1 << 0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_ror_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_ror_epi64 :: < 1 > (a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 63 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_ror_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_mask_ror_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_ror_epi64 :: < 1 > (a , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 63 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_ror_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 0 ,) ; let r = _mm512_maskz_ror_epi64 :: < 1 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_ror_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 63) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_ror_epi64 () { let a = _mm256_set_epi64x (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_ror_epi64 :: < 1 > (a) ; let e = _mm256_set_epi64x (1 << 63 , 1 << 31 , 1 << 31 , 1 << 31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_ror_epi64 () { let a = _mm256_set_epi64x (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_mask_ror_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_ror_epi64 :: < 1 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 63 , 1 << 31 , 1 << 31 , 1 << 31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_ror_epi64 () { let a = _mm256_set_epi64x (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_maskz_ror_epi64 :: < 1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_ror_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 63 , 1 << 31 , 1 << 31 , 1 << 31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_ror_epi64 () { let a = _mm_set_epi64x (1 << 0 , 1 << 32) ; let r = _mm_ror_epi64 :: < 1 > (a) ; let e = _mm_set_epi64x (1 << 63 , 1 << 31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_ror_epi64 () { let a = _mm_set_epi64x (1 << 0 , 1 << 32) ; let r = _mm_mask_ror_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_ror_epi64 :: < 1 > (a , 0b00000011 , a) ; let e = _mm_set_epi64x (1 << 63 , 1 << 31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_ror_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_ror_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_ror_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_ror_epi64 () { let a = _mm_set_epi64x (1 << 0 , 1 << 32) ; let r = _mm_maskz_ror_epi64 :: < 1 > (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_ror_epi64 :: < 1 > (0b00000011 , a) ; let e = _mm_set_epi64x (1 << 63 , 1 << 31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_slli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_slli_epi64 :: < 1 > (a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_slli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_mask_slli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_slli_epi64 :: < 1 > (a , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_slli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 63 ,) ; let r = _mm512_maskz_slli_epi64 :: < 1 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_slli_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 33 , 1 << 33 , 1 << 33 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_slli_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_mask_slli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_slli_epi64 :: < 1 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_slli_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let r = _mm256_maskz_slli_epi64 :: < 1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_slli_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm256_set_epi64x (0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_slli_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let r = _mm_mask_slli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_slli_epi64 :: < 1 > (a , 0b00000011 , a) ; let e = _mm_set_epi64x (0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_slli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_slli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_slli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_slli_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let r = _mm_maskz_slli_epi64 :: < 1 > (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_slli_epi64 :: < 1 > (0b00000011 , a) ; let e = _mm_set_epi64x (0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_srli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_srli_epi64 :: < 1 > (a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_srli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let r = _mm512_mask_srli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_srli_epi64 :: < 1 > (a , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_srli_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 0 ,) ; let r = _mm512_maskz_srli_epi64 :: < 1 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_srli_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 31 , 1 << 31 , 1 << 31 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_srli_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let r = _mm256_mask_srli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_srli_epi64 :: < 1 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_srli_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let r = _mm256_maskz_srli_epi64 :: < 1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_srli_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_srli_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let r = _mm_mask_srli_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_srli_epi64 :: < 1 > (a , 0b00000011 , a) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_srli_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_srli_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_srli_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_srli_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let r = _mm_maskz_srli_epi64 :: < 1 > (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_srli_epi64 :: < 1 > (0b00000011 , a) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_rolv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_rolv_epi64 (a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 34 , 1 << 35 , 1 << 36 , 1 << 37 , 1 << 38 , 1 << 39 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_rolv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_rolv_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_rolv_epi64 (a , 0b11111111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 34 , 1 << 35 , 1 << 36 , 1 << 37 , 1 << 38 , 1 << 39 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_rolv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 62 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 2) ; let r = _mm512_maskz_rolv_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_rolv_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 36 , 1 << 37 , 1 << 38 , 1 << 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_rolv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_rolv_epi64 (a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 34 , 1 << 35) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_rolv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_rolv_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_rolv_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 34 , 1 << 35) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_rolv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_rolv_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_rolv_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 34 , 1 << 35) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_rolv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 63) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_rolv_epi64 (a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_rolv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 63) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_rolv_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_rolv_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_rolv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_rolv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_rolv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_rolv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 63) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_rolv_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_rolv_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_rorv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_rorv_epi64 (a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 1 << 63 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_rorv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_rorv_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_rorv_epi64 (a , 0b11111111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 1 << 63 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_rorv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 0 ,) ; let b = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 2) ; let r = _mm512_maskz_rorv_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_rorv_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 62) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_rorv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_rorv_epi64 (a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 30 , 1 << 29) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_rorv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_rorv_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_rorv_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 30 , 1 << 29) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_rorv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32) ; let b = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_rorv_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_rorv_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 63 , 1 << 30 , 1 << 29) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_rorv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 0) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_rorv_epi64 (a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 63) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_rorv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 0) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_rorv_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_rorv_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 63) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_rorv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_rorv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_rorv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_rorv_epi64 () { let a = _mm_set_epi64x (1 << 32 , 1 << 0) ; let b = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_rorv_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_rorv_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (1 << 32 , 1 << 63) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sllv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm512_set_epi64 (0 , 2 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_sllv_epi64 (a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 0 , 1 << 34 , 1 << 35 , 1 << 36 , 1 << 37 , 1 << 38 , 1 << 39 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sllv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_sllv_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_sllv_epi64 (a , 0b11111111 , a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 1 << 33 , 0 , 1 << 35 , 1 << 36 , 1 << 37 , 1 << 38 , 1 << 39 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sllv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 63 ,) ; let count = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 1) ; let r = _mm512_maskz_sllv_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_sllv_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 36 , 1 << 37 , 1 << 38 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sllv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 32 , 1 << 63 , 1 << 32) ; let count = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_sllv_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_sllv_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 33 , 0 , 1 << 35) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sllv_epi64 () { let a = _mm256_set_epi64x (1 << 32 , 1 << 32 , 1 << 63 , 1 << 32) ; let count = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_sllv_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_sllv_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 32 , 1 << 33 , 0 , 1 << 35) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sllv_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let count = _mm_set_epi64x (2 , 3) ; let r = _mm_mask_sllv_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_sllv_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (0 , 1 << 35) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_sllv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sllv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sllv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sllv_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let count = _mm_set_epi64x (2 , 3) ; let r = _mm_maskz_sllv_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_sllv_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (0 , 1 << 35) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_srlv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_srlv_epi64 (a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 0 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_srlv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_srlv_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_srlv_epi64 (a , 0b11111111 , a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 32 , 0 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_srlv_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 0 ,) ; let count = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_maskz_srlv_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_srlv_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 28 , 1 << 27 , 1 << 26 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_srlv_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm256_set1_epi64x (1) ; let r = _mm256_mask_srlv_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_srlv_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_srlv_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_srlv_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_srlv_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_srlv_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set1_epi64x (1) ; let r = _mm_mask_srlv_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_srlv_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_srlv_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_srlv_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_srlv_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_srlv_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set1_epi64x (1) ; let r = _mm_maskz_srlv_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_srlv_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sll_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_sll_epi64 (a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; let count = _mm_set_epi64x (1 , 0) ; let r = _mm512_sll_epi64 (a , count) ; assert_eq_m512i (r , a) ; }
}

macro_rules! test_mm512_mask_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sll_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_mask_sll_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_sll_epi64 (a , 0b11111111 , a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 , 1 << 33 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sll_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 63 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_maskz_sll_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_sll_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 33 , 1 << 33 , 1 << 33 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sll_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_mask_sll_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_sll_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sll_epi64 () { let a = _mm256_set_epi64x (1 << 63 , 1 << 32 , 1 << 32 , 1 << 32) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_maskz_sll_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_sll_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (0 , 1 << 33 , 1 << 33 , 1 << 33) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sll_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_sll_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_sll_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_sll_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sll_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sll_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sll_epi64 () { let a = _mm_set_epi64x (1 << 63 , 1 << 32) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_sll_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_sll_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (0 , 1 << 33) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_srl_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_srl_epi64 (a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_srl_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 0 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_mask_srl_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_srl_epi64 (a , 0b11111111 , a , count) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_srl_epi64 () { #[rustfmt :: skip] let a = _mm512_set_epi64 (1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 32 , 1 << 0 ,) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm512_maskz_srl_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_srl_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 31 , 1 << 31 , 1 << 31 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_srl_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_mask_srl_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_srl_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_srl_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_maskz_srl_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_srl_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_srl_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_srl_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_srl_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_srl_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_srl_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_srl_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_srl_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_srl_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_srl_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_sra_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm_set_epi64x (0 , 2) ; let r = _mm512_sra_epi64 (a , count) ; let e = _mm512_set_epi64 (0 , - 2 , 0 , 0 , 0 , 0 , 3 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_sra_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm_set_epi64x (0 , 2) ; let r = _mm512_mask_sra_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_sra_epi64 (a , 0b11111111 , a , count) ; let e = _mm512_set_epi64 (0 , - 2 , 0 , 0 , 0 , 0 , 3 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_sra_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm_set_epi64x (0 , 2) ; let r = _mm512_maskz_sra_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_sra_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 3 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_sra_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_sra_epi64 (a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_sra_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_mask_sra_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_sra_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_sra_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm256_maskz_sra_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_sra_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_sra_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_sra_epi64 (a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_sra_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_sra_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_sra_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_sra_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_sra_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_sra_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_sra_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_sra_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_sra_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_srav_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm512_set_epi64 (2 , 2 , 0 , 0 , 0 , 0 , 2 , 1) ; let r = _mm512_srav_epi64 (a , count) ; let e = _mm512_set_epi64 (0 , - 2 , 0 , 0 , 0 , 0 , 3 , - 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_srav_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm512_set_epi64 (2 , 2 , 0 , 0 , 0 , 0 , 2 , 1) ; let r = _mm512_mask_srav_epi64 (a , 0 , a , count) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_srav_epi64 (a , 0b11111111 , a , count) ; let e = _mm512_set_epi64 (0 , - 2 , 0 , 0 , 0 , 0 , 3 , - 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_srav_epi64 () { let a = _mm512_set_epi64 (1 , - 8 , 0 , 0 , 0 , 0 , 15 , - 16) ; let count = _mm512_set_epi64 (2 , 2 , 0 , 0 , 0 , 0 , 2 , 1) ; let r = _mm512_maskz_srav_epi64 (0 , a , count) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_srav_epi64 (0b00001111 , a , count) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 3 , - 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_srav_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm256_set1_epi64x (1) ; let r = _mm256_srav_epi64 (a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_srav_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm256_set1_epi64x (1) ; let r = _mm256_mask_srav_epi64 (a , 0 , a , count) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_srav_epi64 (a , 0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_srav_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let count = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_srav_epi64 (0 , a , count) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_srav_epi64 (0b00001111 , a , count) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_srav_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set1_epi64x (1) ; let r = _mm_srav_epi64 (a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_srav_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set1_epi64x (1) ; let r = _mm_mask_srav_epi64 (a , 0 , a , count) ; assert_eq_m128i (r , a) ; let r = _mm_mask_srav_epi64 (a , 0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_srav_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_srav_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_srav_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_srav_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let count = _mm_set1_epi64x (1) ; let r = _mm_maskz_srav_epi64 (0 , a , count) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_srav_epi64 (0b00000011 , a , count) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_srai_epi64 () { let a = _mm512_set_epi64 (1 , - 4 , 15 , 0 , 0 , 0 , 0 , - 16) ; let r = _mm512_srai_epi64 :: < 2 > (a) ; let e = _mm512_set_epi64 (0 , - 1 , 3 , 0 , 0 , 0 , 0 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_srai_epi64 () { let a = _mm512_set_epi64 (1 , - 4 , 15 , 0 , 0 , 0 , 0 , - 16) ; let r = _mm512_mask_srai_epi64 :: < 2 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_srai_epi64 :: < 2 > (a , 0b11111111 , a) ; let e = _mm512_set_epi64 (0 , - 1 , 3 , 0 , 0 , 0 , 0 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_srai_epi64 () { let a = _mm512_set_epi64 (1 , - 4 , 15 , 0 , 0 , 0 , 0 , - 16) ; let r = _mm512_maskz_srai_epi64 :: < 2 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_srai_epi64 :: < 2 > (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , - 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_srai_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let r = _mm256_srai_epi64 :: < 1 > (a) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_srai_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let r = _mm256_mask_srai_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_srai_epi64 :: < 1 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_srai_epi64 () { let a = _mm256_set_epi64x (1 << 5 , 0 , 0 , 0) ; let r = _mm256_maskz_srai_epi64 :: < 1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_srai_epi64 :: < 1 > (0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 4 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_srai_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let r = _mm_srai_epi64 :: < 1 > (a) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_srai_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let r = _mm_mask_srai_epi64 :: < 1 > (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_srai_epi64 :: < 1 > (a , 0b00000011 , a) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_srai_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_srai_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_srai_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_srai_epi64 () { let a = _mm_set_epi64x (1 << 5 , 0) ; let r = _mm_maskz_srai_epi64 :: < 1 > (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_srai_epi64 :: < 1 > (0b00000011 , a) ; let e = _mm_set_epi64x (1 << 4 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permute_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permute_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_permute_pd :: < 0b11_11_11_11 > (a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permute_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permute_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_mask_permute_pd :: < 0b11_11_11_11 > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_permute_pd :: < 0b11_11_11_11 > (a , 0b11111111 , a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permute_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permute_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_maskz_permute_pd :: < 0b11_11_11_11 > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_permute_pd :: < 0b11_11_11_11 > (0b11111111 , a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permute_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permute_pd () { let a = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_mask_permute_pd :: < 0b11_11 > (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_permute_pd :: < 0b11_11 > (a , 0b00001111 , a) ; let e = _mm256_set_pd (3. , 3. , 1. , 1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permute_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permute_pd () { let a = _mm256_set_pd (3. , 2. , 1. , 0.) ; let r = _mm256_maskz_permute_pd :: < 0b11_11 > (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_permute_pd :: < 0b11_11 > (0b00001111 , a) ; let e = _mm256_set_pd (3. , 3. , 1. , 1.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permute_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_permute_pd () { let a = _mm_set_pd (1. , 0.) ; let r = _mm_mask_permute_pd :: < 0b11 > (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_permute_pd :: < 0b11 > (a , 0b00000011 , a) ; let e = _mm_set_pd (1. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_permute_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permute_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permute_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_permute_pd () { let a = _mm_set_pd (1. , 0.) ; let r = _mm_maskz_permute_pd :: < 0b11 > (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_permute_pd :: < 0b11 > (0b00000011 , a) ; let e = _mm_set_pd (1. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutex_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_permutex_epi64 :: < 0b11_11_11_11 > (a) ; let e = _mm512_setr_epi64 (3 , 3 , 3 , 3 , 7 , 7 , 7 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutex_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_permutex_epi64 :: < 0b11_11_11_11 > (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_permutex_epi64 :: < 0b11_11_11_11 > (a , 0b11111111 , a) ; let e = _mm512_setr_epi64 (3 , 3 , 3 , 3 , 7 , 7 , 7 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutex_epi64 () { let a = _mm512_setr_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_maskz_permutex_epi64 :: < 0b11_11_11_11 > (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_permutex_epi64 :: < 0b11_11_11_11 > (0b11111111 , a) ; let e = _mm512_setr_epi64 (3 , 3 , 3 , 3 , 7 , 7 , 7 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutex_epi64 () { let a = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_permutex_epi64 :: < 0b11_11_11_11 > (a) ; let e = _mm256_set_epi64x (3 , 3 , 3 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutex_epi64 () { let a = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_mask_permutex_epi64 :: < 0b11_11_11_11 > (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_permutex_epi64 :: < 0b11_11_11_11 > (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (3 , 3 , 3 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_permutex_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutex_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutex_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm256_maskz_permutex_epi64 () { let a = _mm256_set_epi64x (3 , 2 , 1 , 0) ; let r = _mm256_maskz_permutex_epi64 :: < 0b11_11_11_11 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_permutex_epi64 :: < 0b11_11_11_11 > (0b00001111 , a) ; let e = _mm256_set_epi64x (3 , 3 , 3 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutex_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_permutex_pd :: < 0b11_11_11_11 > (a) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 7. , 7. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutex_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_mask_permutex_pd :: < 0b11_11_11_11 > (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_permutex_pd :: < 0b11_11_11_11 > (a , 0b11111111 , a) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 7. , 7. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutex_pd () { let a = _mm512_setr_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_maskz_permutex_pd :: < 0b11_11_11_11 > (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_permutex_pd :: < 0b11_11_11_11 > (0b11111111 , a) ; let e = _mm512_setr_pd (3. , 3. , 3. , 3. , 7. , 7. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutex_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_permutex_pd :: < 0b11_11_11_11 > (a) ; let e = _mm256_set_pd (0. , 0. , 0. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutex_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_mask_permutex_pd :: < 0b11_11_11_11 > (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_permutex_pd :: < 0b11_11_11_11 > (a , 0b00001111 , a) ; let e = _mm256_set_pd (0. , 0. , 0. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_permutex_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutex_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutex_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutex_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_maskz_permutex_pd :: < 0b11_11_11_11 > (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_permutex_pd :: < 0b11_11_11_11 > (0b00001111 , a) ; let e = _mm256_set_pd (0. , 0. , 0. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutevar_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_set1_epi64 (0b1) ; let r = _mm512_permutevar_pd (a , b) ; let e = _mm512_set_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutevar_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_set1_epi64 (0b1) ; let r = _mm512_mask_permutevar_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_permutevar_pd (a , 0b11111111 , a , b) ; let e = _mm512_set_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutevar_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let b = _mm512_set1_epi64 (0b1) ; let r = _mm512_maskz_permutevar_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_permutevar_pd (0b00001111 , a , b) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutevar_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set1_epi64x (0b1) ; let r = _mm256_mask_permutevar_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_permutevar_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (1. , 1. , 3. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutevar_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let b = _mm256_set1_epi64x (0b1) ; let r = _mm256_maskz_permutevar_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_permutevar_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (1. , 1. , 3. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_permutevar_pd () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set1_epi64x (0b1) ; let r = _mm_mask_permutevar_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_permutevar_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (1. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_permutevar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permutevar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permutevar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_permutevar_pd () { let a = _mm_set_pd (0. , 1.) ; let b = _mm_set1_epi64x (0b1) ; let r = _mm_maskz_permutevar_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_permutevar_pd (0b00000011 , a , b) ; let e = _mm_set_pd (1. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutexvar_epi64 () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_permutexvar_epi64 (idx , a) ; let e = _mm512_set1_epi64 (6) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutexvar_epi64 () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_permutexvar_epi64 (a , 0 , idx , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_permutexvar_epi64 (a , 0b11111111 , idx , a) ; let e = _mm512_set1_epi64 (6) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutexvar_epi64 () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_maskz_permutexvar_epi64 (0 , idx , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_permutexvar_epi64 (0b00001111 , idx , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 6 , 6 , 6 , 6) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutexvar_epi64 () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_permutexvar_epi64 (idx , a) ; let e = _mm256_set1_epi64x (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutexvar_epi64 () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_permutexvar_epi64 (a , 0 , idx , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_permutexvar_epi64 (a , 0b00001111 , idx , a) ; let e = _mm256_set1_epi64x (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_permutexvar_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutexvar_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutexvar_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutexvar_epi64 () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_permutexvar_epi64 (0 , idx , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_permutexvar_epi64 (0b00001111 , idx , a) ; let e = _mm256_set1_epi64x (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutexvar_pd () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_permutexvar_pd (idx , a) ; let e = _mm512_set1_pd (6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutexvar_pd () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_mask_permutexvar_pd (a , 0 , idx , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_permutexvar_pd (a , 0b11111111 , idx , a) ; let e = _mm512_set1_pd (6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutexvar_pd () { let idx = _mm512_set1_epi64 (1) ; let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_maskz_permutexvar_pd (0 , idx , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_permutexvar_pd (0b00001111 , idx , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 6. , 6. , 6. , 6.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutexvar_pd () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_permutexvar_pd (idx , a) ; let e = _mm256_set1_pd (2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutexvar_pd () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_mask_permutexvar_pd (a , 0 , idx , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_permutexvar_pd (a , 0b00001111 , idx , a) ; let e = _mm256_set1_pd (2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_permutexvar_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutexvar_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutexvar_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutexvar_pd () { let idx = _mm256_set1_epi64x (1) ; let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_maskz_permutexvar_pd (0 , idx , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_permutexvar_pd (0b00001111 , idx , a) ; let e = _mm256_set1_pd (2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutex2var_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_epi64 (100) ; let r = _mm512_permutex2var_epi64 (a , idx , b) ; let e = _mm512_set_epi64 (6 , 100 , 5 , 100 , 4 , 100 , 3 , 100) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutex2var_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_epi64 (100) ; let r = _mm512_mask_permutex2var_epi64 (a , 0 , idx , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_permutex2var_epi64 (a , 0b11111111 , idx , b) ; let e = _mm512_set_epi64 (6 , 100 , 5 , 100 , 4 , 100 , 3 , 100) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutex2var_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_epi64 (100) ; let r = _mm512_maskz_permutex2var_epi64 (0 , a , idx , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_permutex2var_epi64 (0b00001111 , a , idx , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 4 , 100 , 3 , 100) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask2_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask2_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask2_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask2_permutex2var_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let idx = _mm512_set_epi64 (1000 , 1 << 3 , 2000 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_epi64 (100) ; let r = _mm512_mask2_permutex2var_epi64 (a , idx , 0 , b) ; assert_eq_m512i (r , idx) ; let r = _mm512_mask2_permutex2var_epi64 (a , idx , 0b00001111 , b) ; let e = _mm512_set_epi64 (1000 , 1 << 3 , 2000 , 1 << 3 , 4 , 100 , 3 , 100) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutex2var_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_epi64x (100) ; let r = _mm256_permutex2var_epi64 (a , idx , b) ; let e = _mm256_set_epi64x (2 , 100 , 1 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutex2var_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_epi64x (100) ; let r = _mm256_mask_permutex2var_epi64 (a , 0 , idx , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_permutex2var_epi64 (a , 0b00001111 , idx , b) ; let e = _mm256_set_epi64x (2 , 100 , 1 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutex2var_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_epi64x (100) ; let r = _mm256_maskz_permutex2var_epi64 (0 , a , idx , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_permutex2var_epi64 (0b00001111 , a , idx , b) ; let e = _mm256_set_epi64x (2 , 100 , 1 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask2_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask2_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask2_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask2_permutex2var_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_epi64x (100) ; let r = _mm256_mask2_permutex2var_epi64 (a , idx , 0 , b) ; assert_eq_m256i (r , idx) ; let r = _mm256_mask2_permutex2var_epi64 (a , idx , 0b00001111 , b) ; let e = _mm256_set_epi64x (2 , 100 , 1 , 100) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_permutex2var_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_epi64x (100) ; let r = _mm_permutex2var_epi64 (a , idx , b) ; let e = _mm_set_epi64x (0 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_permutex2var_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_epi64x (100) ; let r = _mm_mask_permutex2var_epi64 (a , 0 , idx , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_permutex2var_epi64 (a , 0b00000011 , idx , b) ; let e = _mm_set_epi64x (0 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_permutex2var_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_epi64x (100) ; let r = _mm_maskz_permutex2var_epi64 (0 , a , idx , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_permutex2var_epi64 (0b00000011 , a , idx , b) ; let e = _mm_set_epi64x (0 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask2_permutex2var_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask2_permutex2var_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask2_permutex2var_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask2_permutex2var_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_epi64x (100) ; let r = _mm_mask2_permutex2var_epi64 (a , idx , 0 , b) ; assert_eq_m128i (r , idx) ; let r = _mm_mask2_permutex2var_epi64 (a , idx , 0b00000011 , b) ; let e = _mm_set_epi64x (0 , 100) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_permutex2var_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_pd (100.) ; let r = _mm512_permutex2var_pd (a , idx , b) ; let e = _mm512_set_pd (6. , 100. , 5. , 100. , 4. , 100. , 3. , 100.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_permutex2var_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_pd (100.) ; let r = _mm512_mask_permutex2var_pd (a , 0 , idx , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_permutex2var_pd (a , 0b11111111 , idx , b) ; let e = _mm512_set_pd (6. , 100. , 5. , 100. , 4. , 100. , 3. , 100.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_permutex2var_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_pd (100.) ; let r = _mm512_maskz_permutex2var_pd (0 , a , idx , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_permutex2var_pd (0b00001111 , a , idx , b) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 4. , 100. , 3. , 100.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask2_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask2_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask2_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask2_permutex2var_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let idx = _mm512_set_epi64 (1 , 1 << 3 , 2 , 1 << 3 , 3 , 1 << 3 , 4 , 1 << 3) ; let b = _mm512_set1_pd (100.) ; let r = _mm512_mask2_permutex2var_pd (a , idx , 0 , b) ; assert_eq_m512d (r , _mm512_castsi512_pd (idx)) ; let r = _mm512_mask2_permutex2var_pd (a , idx , 0b11111111 , b) ; let e = _mm512_set_pd (6. , 100. , 5. , 100. , 4. , 100. , 3. , 100.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_permutex2var_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_pd (100.) ; let r = _mm256_permutex2var_pd (a , idx , b) ; let e = _mm256_set_pd (2. , 100. , 1. , 100.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_permutex2var_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_pd (100.) ; let r = _mm256_mask_permutex2var_pd (a , 0 , idx , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_permutex2var_pd (a , 0b00001111 , idx , b) ; let e = _mm256_set_pd (2. , 100. , 1. , 100.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_permutex2var_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_pd (100.) ; let r = _mm256_maskz_permutex2var_pd (0 , a , idx , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_permutex2var_pd (0b00001111 , a , idx , b) ; let e = _mm256_set_pd (2. , 100. , 1. , 100.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask2_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask2_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask2_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask2_permutex2var_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let idx = _mm256_set_epi64x (1 , 1 << 2 , 2 , 1 << 2) ; let b = _mm256_set1_pd (100.) ; let r = _mm256_mask2_permutex2var_pd (a , idx , 0 , b) ; assert_eq_m256d (r , _mm256_castsi256_pd (idx)) ; let r = _mm256_mask2_permutex2var_pd (a , idx , 0b00001111 , b) ; let e = _mm256_set_pd (2. , 100. , 1. , 100.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_permutex2var_pd () { let a = _mm_set_pd (0. , 1.) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_pd (100.) ; let r = _mm_permutex2var_pd (a , idx , b) ; let e = _mm_set_pd (0. , 100.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_permutex2var_pd () { let a = _mm_set_pd (0. , 1.) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_pd (100.) ; let r = _mm_mask_permutex2var_pd (a , 0 , idx , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_permutex2var_pd (a , 0b00000011 , idx , b) ; let e = _mm_set_pd (0. , 100.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_permutex2var_pd () { let a = _mm_set_pd (0. , 1.) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_pd (100.) ; let r = _mm_maskz_permutex2var_pd (0 , a , idx , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_permutex2var_pd (0b00000011 , a , idx , b) ; let e = _mm_set_pd (0. , 100.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask2_permutex2var_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask2_permutex2var_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask2_permutex2var_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask2_permutex2var_pd () { let a = _mm_set_pd (0. , 1.) ; let idx = _mm_set_epi64x (1 , 1 << 1) ; let b = _mm_set1_pd (100.) ; let r = _mm_mask2_permutex2var_pd (a , idx , 0 , b) ; assert_eq_m128d (r , _mm_castsi128_pd (idx)) ; let r = _mm_mask2_permutex2var_pd (a , idx , 0b00000011 , b) ; let e = _mm_set_pd (0. , 100.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_shuffle_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_shuffle_pd () { let a = _mm256_set_pd (1. , 4. , 5. , 8.) ; let b = _mm256_set_pd (2. , 3. , 6. , 7.) ; let r = _mm256_mask_shuffle_pd :: < 0b11_11_11_11 > (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_shuffle_pd :: < 0b11_11_11_11 > (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (2. , 1. , 6. , 5.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_shuffle_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_shuffle_pd () { let a = _mm256_set_pd (1. , 4. , 5. , 8.) ; let b = _mm256_set_pd (2. , 3. , 6. , 7.) ; let r = _mm256_maskz_shuffle_pd :: < 0b11_11_11_11 > (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_shuffle_pd :: < 0b11_11_11_11 > (0b00001111 , a , b) ; let e = _mm256_set_pd (2. , 1. , 6. , 5.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_shuffle_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_shuffle_pd () { let a = _mm_set_pd (1. , 4.) ; let b = _mm_set_pd (2. , 3.) ; let r = _mm_mask_shuffle_pd :: < 0b11_11_11_11 > (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_shuffle_pd :: < 0b11_11_11_11 > (a , 0b00000011 , a , b) ; let e = _mm_set_pd (2. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_shuffle_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_shuffle_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_shuffle_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_shuffle_pd () { let a = _mm_set_pd (1. , 4.) ; let b = _mm_set_pd (2. , 3.) ; let r = _mm_maskz_shuffle_pd :: < 0b11_11_11_11 > (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_shuffle_pd :: < 0b11_11_11_11 > (0b00000011 , a , b) ; let e = _mm_set_pd (2. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_shuffle_i64x2 () { let a = _mm512_setr_epi64 (1 , 4 , 5 , 8 , 9 , 12 , 13 , 16) ; let b = _mm512_setr_epi64 (2 , 3 , 6 , 7 , 10 , 11 , 14 , 15) ; let r = _mm512_shuffle_i64x2 :: < 0b00_00_00_00 > (a , b) ; let e = _mm512_setr_epi64 (1 , 4 , 1 , 4 , 2 , 3 , 2 , 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_shuffle_i64x2 () { let a = _mm512_setr_epi64 (1 , 4 , 5 , 8 , 9 , 12 , 13 , 16) ; let b = _mm512_setr_epi64 (2 , 3 , 6 , 7 , 10 , 11 , 14 , 15) ; let r = _mm512_mask_shuffle_i64x2 :: < 0b00_00_00_00 > (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_shuffle_i64x2 :: < 0b00_00_00_00 > (a , 0b11111111 , a , b) ; let e = _mm512_setr_epi64 (1 , 4 , 1 , 4 , 2 , 3 , 2 , 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_shuffle_i64x2 () { let a = _mm512_setr_epi64 (1 , 4 , 5 , 8 , 9 , 12 , 13 , 16) ; let b = _mm512_setr_epi64 (2 , 3 , 6 , 7 , 10 , 11 , 14 , 15) ; let r = _mm512_maskz_shuffle_i64x2 :: < 0b00_00_00_00 > (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_shuffle_i64x2 :: < 0b00_00_00_00 > (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (1 , 4 , 1 , 4 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_shuffle_i64x2 () { let a = _mm256_set_epi64x (1 , 4 , 5 , 8) ; let b = _mm256_set_epi64x (2 , 3 , 6 , 7) ; let r = _mm256_shuffle_i64x2 :: < 0b00 > (a , b) ; let e = _mm256_set_epi64x (6 , 7 , 5 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_shuffle_i64x2 () { let a = _mm256_set_epi64x (1 , 4 , 5 , 8) ; let b = _mm256_set_epi64x (2 , 3 , 6 , 7) ; let r = _mm256_mask_shuffle_i64x2 :: < 0b00 > (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_shuffle_i64x2 :: < 0b00 > (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (6 , 7 , 5 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_shuffle_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_shuffle_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_shuffle_i64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_shuffle_i64x2 () { let a = _mm256_set_epi64x (1 , 4 , 5 , 8) ; let b = _mm256_set_epi64x (2 , 3 , 6 , 7) ; let r = _mm256_maskz_shuffle_i64x2 :: < 0b00 > (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_shuffle_i64x2 :: < 0b00 > (0b00001111 , a , b) ; let e = _mm256_set_epi64x (6 , 7 , 5 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_shuffle_f64x2 () { let a = _mm512_setr_pd (1. , 4. , 5. , 8. , 9. , 12. , 13. , 16.) ; let b = _mm512_setr_pd (2. , 3. , 6. , 7. , 10. , 11. , 14. , 15.) ; let r = _mm512_shuffle_f64x2 :: < 0b00_00_00_00 > (a , b) ; let e = _mm512_setr_pd (1. , 4. , 1. , 4. , 2. , 3. , 2. , 3.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_shuffle_f64x2 () { let a = _mm512_setr_pd (1. , 4. , 5. , 8. , 9. , 12. , 13. , 16.) ; let b = _mm512_setr_pd (2. , 3. , 6. , 7. , 10. , 11. , 14. , 15.) ; let r = _mm512_mask_shuffle_f64x2 :: < 0b00_00_00_00 > (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_shuffle_f64x2 :: < 0b00_00_00_00 > (a , 0b11111111 , a , b) ; let e = _mm512_setr_pd (1. , 4. , 1. , 4. , 2. , 3. , 2. , 3.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_shuffle_f64x2 () { let a = _mm512_setr_pd (1. , 4. , 5. , 8. , 9. , 12. , 13. , 16.) ; let b = _mm512_setr_pd (2. , 3. , 6. , 7. , 10. , 11. , 14. , 15.) ; let r = _mm512_maskz_shuffle_f64x2 :: < 0b00_00_00_00 > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_shuffle_f64x2 :: < 0b00_00_00_00 > (0b00001111 , a , b) ; let e = _mm512_setr_pd (1. , 4. , 1. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_shuffle_f64x2 () { let a = _mm256_set_pd (1. , 4. , 5. , 8.) ; let b = _mm256_set_pd (2. , 3. , 6. , 7.) ; let r = _mm256_shuffle_f64x2 :: < 0b00 > (a , b) ; let e = _mm256_set_pd (6. , 7. , 5. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_shuffle_f64x2 () { let a = _mm256_set_pd (1. , 4. , 5. , 8.) ; let b = _mm256_set_pd (2. , 3. , 6. , 7.) ; let r = _mm256_mask_shuffle_f64x2 :: < 0b00 > (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_shuffle_f64x2 :: < 0b00 > (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (6. , 7. , 5. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_shuffle_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_shuffle_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_shuffle_f64x2_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_shuffle_f64x2 () { let a = _mm256_set_pd (1. , 4. , 5. , 8.) ; let b = _mm256_set_pd (2. , 3. , 6. , 7.) ; let r = _mm256_maskz_shuffle_f64x2 :: < 0b00 > (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_shuffle_f64x2 :: < 0b00 > (0b00001111 , a , b) ; let e = _mm256_set_pd (6. , 7. , 5. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_movedup_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_movedup_pd (a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_movedup_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_mask_movedup_pd (a , 0 , a) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_movedup_pd (a , 0b11111111 , a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 5. , 5. , 7. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_movedup_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_movedup_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_movedup_pd (0b00001111 , a) ; let e = _mm512_setr_pd (1. , 1. , 3. , 3. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_movedup_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_mask_movedup_pd (a , 0 , a) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_movedup_pd (a , 0b00001111 , a) ; let e = _mm256_set_pd (2. , 2. , 4. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_movedup_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_movedup_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_movedup_pd (0b00001111 , a) ; let e = _mm256_set_pd (2. , 2. , 4. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_movedup_pd () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_mask_movedup_pd (a , 0 , a) ; assert_eq_m128d (r , a) ; let r = _mm_mask_movedup_pd (a , 0b00000011 , a) ; let e = _mm_set_pd (2. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_movedup_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_movedup_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_movedup_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_movedup_pd () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_maskz_movedup_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_movedup_pd (0b00000011 , a) ; let e = _mm_set_pd (2. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_inserti64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_inserti64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_inserti64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_inserti64x4 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_inserti64x4 :: < 1 > (a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 17 , 18 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_inserti64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_inserti64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_inserti64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_inserti64x4 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_mask_inserti64x4 :: < 1 > (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_inserti64x4 :: < 1 > (a , 0b11111111 , a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 17 , 18 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_inserti64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_inserti64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_inserti64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_inserti64x4 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_maskz_inserti64x4 :: < 1 > (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_inserti64x4 :: < 1 > (0b00001111 , a , b) ; let e = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_insertf64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_insertf64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_insertf64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_insertf64x4 () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_setr_pd (17. , 18. , 19. , 20.) ; let r = _mm512_insertf64x4 :: < 1 > (a , b) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 17. , 18. , 19. , 20.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_insertf64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_insertf64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_insertf64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_insertf64x4 () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_setr_pd (17. , 18. , 19. , 20.) ; let r = _mm512_mask_insertf64x4 :: < 1 > (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_insertf64x4 :: < 1 > (a , 0b11111111 , a , b) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 17. , 18. , 19. , 20.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_insertf64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_insertf64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_insertf64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_insertf64x4 () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_setr_pd (17. , 18. , 19. , 20.) ; let r = _mm512_maskz_insertf64x4 :: < 1 > (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_insertf64x4 :: < 1 > (0b00001111 , a , b) ; let e = _mm512_setr_pd (1. , 2. , 3. , 4. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_castpd128_pd512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd128_pd512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd128_pd512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd128_pd512 () { let a = _mm_setr_pd (17. , 18.) ; let r = _mm512_castpd128_pd512 (a) ; assert_eq_m128d (_mm512_castpd512_pd128 (r) , a) ; }
}

macro_rules! test_mm512_castpd256_pd512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd256_pd512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd256_pd512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd256_pd512 () { let a = _mm256_setr_pd (17. , 18. , 19. , 20.) ; let r = _mm512_castpd256_pd512 (a) ; assert_eq_m256d (_mm512_castpd512_pd256 (r) , a) ; }
}

macro_rules! test_mm512_zextpd128_pd512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_zextpd128_pd512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_zextpd128_pd512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_zextpd128_pd512 () { let a = _mm_setr_pd (17. , 18.) ; let r = _mm512_zextpd128_pd512 (a) ; let e = _mm512_setr_pd (17. , 18. , 0. , 0. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_zextpd256_pd512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_zextpd256_pd512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_zextpd256_pd512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_zextpd256_pd512 () { let a = _mm256_setr_pd (17. , 18. , 19. , 20.) ; let r = _mm512_zextpd256_pd512 (a) ; let e = _mm512_setr_pd (17. , 18. , 19. , 20. , 0. , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_castpd512_pd128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd512_pd128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd512_pd128_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd512_pd128 () { let a = _mm512_setr_pd (17. , 18. , - 1. , - 1. , - 1. , - 1. , - 1. , - 1.) ; let r = _mm512_castpd512_pd128 (a) ; let e = _mm_setr_pd (17. , 18.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_castpd512_pd256_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd512_pd256 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd512_pd256_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd512_pd256 () { let a = _mm512_setr_pd (17. , 18. , 19. , 20. , - 1. , - 1. , - 1. , - 1.) ; let r = _mm512_castpd512_pd256 (a) ; let e = _mm256_setr_pd (17. , 18. , 19. , 20.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_castpd_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd_ps () { let a = _mm512_set1_pd (1.) ; let r = _mm512_castpd_ps (a) ; let e = _mm512_set_ps (1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 , 1.875 , 0.0 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_castpd_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castpd_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castpd_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castpd_si512 () { let a = _mm512_set1_pd (1.) ; let r = _mm512_castpd_si512 (a) ; let e = _mm512_set_epi32 (1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 , 1072693248 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_castsi128_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi128_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi128_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi128_si512 () { let a = _mm_setr_epi64x (17 , 18) ; let r = _mm512_castsi128_si512 (a) ; assert_eq_m128i (_mm512_castsi512_si128 (r) , a) ; }
}

macro_rules! test_mm512_castsi256_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi256_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi256_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi256_si512 () { let a = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_castsi256_si512 (a) ; assert_eq_m256i (_mm512_castsi512_si256 (r) , a) ; }
}

macro_rules! test_mm512_zextsi128_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_zextsi128_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_zextsi128_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_zextsi128_si512 () { let a = _mm_setr_epi64x (17 , 18) ; let r = _mm512_zextsi128_si512 (a) ; let e = _mm512_setr_epi64 (17 , 18 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_zextsi256_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_zextsi256_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_zextsi256_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_zextsi256_si512 () { let a = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_zextsi256_si512 (a) ; let e = _mm512_setr_epi64 (17 , 18 , 19 , 20 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_castsi512_si128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi512_si128 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi512_si128_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi512_si128 () { let a = _mm512_setr_epi64 (17 , 18 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_castsi512_si128 (a) ; let e = _mm_setr_epi64x (17 , 18) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_castsi512_si256_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi512_si256 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi512_si256_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi512_si256 () { let a = _mm512_setr_epi64 (17 , 18 , 19 , 20 , - 1 , - 1 , - 1 , - 1) ; let r = _mm512_castsi512_si256 (a) ; let e = _mm256_setr_epi64x (17 , 18 , 19 , 20) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_castsi512_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi512_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi512_ps_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi512_ps () { let a = _mm512_set1_epi64 (1 << 62) ; let r = _mm512_castsi512_ps (a) ; let e = _mm512_set_ps (2. , 0. , 2. , 0. , 2. , 0. , 2. , 0. , 2. , 0. , 2. , 0. , 2. , 0. , 2. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_castsi512_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_castsi512_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_castsi512_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_castsi512_pd () { let a = _mm512_set1_epi64 (1 << 62) ; let r = _mm512_castsi512_pd (a) ; let e = _mm512_set_pd (2. , 2. , 2. , 2. , 2. , 2. , 2. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_broadcastq_epi64 () { let a = _mm_setr_epi64x (17 , 18) ; let r = _mm512_broadcastq_epi64 (a) ; let e = _mm512_set1_epi64 (17) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_broadcastq_epi64 () { let src = _mm512_set1_epi64 (18) ; let a = _mm_setr_epi64x (17 , 18) ; let r = _mm512_mask_broadcastq_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_broadcastq_epi64 (src , 0b11111111 , a) ; let e = _mm512_set1_epi64 (17) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_broadcastq_epi64 () { let a = _mm_setr_epi64x (17 , 18) ; let r = _mm512_maskz_broadcastq_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_broadcastq_epi64 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 17 , 17 , 17 , 17) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_broadcastq_epi64 () { let src = _mm256_set1_epi64x (18) ; let a = _mm_set_epi64x (17 , 18) ; let r = _mm256_mask_broadcastq_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_broadcastq_epi64 (src , 0b00001111 , a) ; let e = _mm256_set1_epi64x (18) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_broadcastq_epi64 () { let a = _mm_set_epi64x (17 , 18) ; let r = _mm256_maskz_broadcastq_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_broadcastq_epi64 (0b00001111 , a) ; let e = _mm256_set1_epi64x (18) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_broadcastq_epi64 () { let src = _mm_set1_epi64x (18) ; let a = _mm_set_epi64x (17 , 18) ; let r = _mm_mask_broadcastq_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_broadcastq_epi64 (src , 0b00000011 , a) ; let e = _mm_set1_epi64x (18) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_broadcastq_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_broadcastq_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_broadcastq_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_broadcastq_epi64 () { let a = _mm_set_epi64x (17 , 18) ; let r = _mm_maskz_broadcastq_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_broadcastq_epi64 (0b00000011 , a) ; let e = _mm_set1_epi64x (18) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_broadcastsd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcastsd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcastsd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_broadcastsd_pd () { let a = _mm_set_pd (17. , 18.) ; let r = _mm512_broadcastsd_pd (a) ; let e = _mm512_set1_pd (18.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_broadcastsd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcastsd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcastsd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_broadcastsd_pd () { let src = _mm512_set1_pd (18.) ; let a = _mm_set_pd (17. , 18.) ; let r = _mm512_mask_broadcastsd_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_broadcastsd_pd (src , 0b11111111 , a) ; let e = _mm512_set1_pd (18.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcastsd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcastsd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcastsd_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_broadcastsd_pd () { let a = _mm_set_pd (17. , 18.) ; let r = _mm512_maskz_broadcastsd_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_broadcastsd_pd (0b00001111 , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 18. , 18. , 18. , 18.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_broadcastsd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcastsd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcastsd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_broadcastsd_pd () { let src = _mm256_set1_pd (18.) ; let a = _mm_set_pd (17. , 18.) ; let r = _mm256_mask_broadcastsd_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_broadcastsd_pd (src , 0b00001111 , a) ; let e = _mm256_set1_pd (18.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcastsd_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcastsd_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcastsd_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_broadcastsd_pd () { let a = _mm_set_pd (17. , 18.) ; let r = _mm256_maskz_broadcastsd_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_broadcastsd_pd (0b00001111 , a) ; let e = _mm256_set1_pd (18.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_broadcast_i64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_i64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_i64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_broadcast_i64x4 () { let a = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_broadcast_i64x4 (a) ; let e = _mm512_set_epi64 (17 , 18 , 19 , 20 , 17 , 18 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_i64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_i64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_i64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_broadcast_i64x4 () { let src = _mm512_set1_epi64 (18) ; let a = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_mask_broadcast_i64x4 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_broadcast_i64x4 (src , 0b11111111 , a) ; let e = _mm512_set_epi64 (17 , 18 , 19 , 20 , 17 , 18 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_i64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_i64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_i64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_broadcast_i64x4 () { let a = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm512_maskz_broadcast_i64x4 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_broadcast_i64x4 (0b00001111 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 17 , 18 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_broadcast_f64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_f64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_f64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_broadcast_f64x4 () { let a = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm512_broadcast_f64x4 (a) ; let e = _mm512_set_pd (17. , 18. , 19. , 20. , 17. , 18. , 19. , 20.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_f64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_f64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_f64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_broadcast_f64x4 () { let src = _mm512_set1_pd (18.) ; let a = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm512_mask_broadcast_f64x4 (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_broadcast_f64x4 (src , 0b11111111 , a) ; let e = _mm512_set_pd (17. , 18. , 19. , 20. , 17. , 18. , 19. , 20.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_f64x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_f64x4 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_f64x4_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_broadcast_f64x4 () { let a = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm512_maskz_broadcast_f64x4 (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_broadcast_f64x4 (0b00001111 , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 17. , 18. , 19. , 20.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_blend_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_blend_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_blend_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_blend_epi64 () { let a = _mm512_set1_epi64 (1) ; let b = _mm512_set1_epi64 (2) ; let r = _mm512_mask_blend_epi64 (0b11110000 , a , b) ; let e = _mm512_set_epi64 (2 , 2 , 2 , 2 , 1 , 1 , 1 , 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_blend_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_blend_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_blend_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_blend_epi64 () { let a = _mm256_set1_epi64x (1) ; let b = _mm256_set1_epi64x (2) ; let r = _mm256_mask_blend_epi64 (0b00001111 , a , b) ; let e = _mm256_set1_epi64x (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_blend_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_blend_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_blend_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_blend_epi64 () { let a = _mm_set1_epi64x (1) ; let b = _mm_set1_epi64x (2) ; let r = _mm_mask_blend_epi64 (0b00000011 , a , b) ; let e = _mm_set1_epi64x (2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_blend_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_blend_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_blend_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_blend_pd () { let a = _mm512_set1_pd (1.) ; let b = _mm512_set1_pd (2.) ; let r = _mm512_mask_blend_pd (0b11110000 , a , b) ; let e = _mm512_set_pd (2. , 2. , 2. , 2. , 1. , 1. , 1. , 1.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_blend_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_blend_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_blend_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_blend_pd () { let a = _mm256_set1_pd (1.) ; let b = _mm256_set1_pd (2.) ; let r = _mm256_mask_blend_pd (0b00001111 , a , b) ; let e = _mm256_set1_pd (2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_blend_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_blend_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_blend_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_blend_pd () { let a = _mm_set1_pd (1.) ; let b = _mm_set1_pd (2.) ; let r = _mm_mask_blend_pd (0b00000011 , a , b) ; let e = _mm_set1_pd (2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_unpackhi_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_unpackhi_epi64 (a , b) ; let e = _mm512_set_epi64 (17 , 1 , 19 , 3 , 21 , 5 , 23 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_unpackhi_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_mask_unpackhi_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_unpackhi_epi64 (a , 0b11111111 , a , b) ; let e = _mm512_set_epi64 (17 , 1 , 19 , 3 , 21 , 5 , 23 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_unpackhi_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_maskz_unpackhi_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_unpackhi_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 21 , 5 , 23 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_unpackhi_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm256_mask_unpackhi_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_unpackhi_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (17 , 1 , 19 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_unpackhi_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm256_maskz_unpackhi_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_unpackhi_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (17 , 1 , 19 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_unpackhi_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (17 , 18) ; let r = _mm_mask_unpackhi_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_unpackhi_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (17 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_unpackhi_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_unpackhi_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_unpackhi_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_unpackhi_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (17 , 18) ; let r = _mm_maskz_unpackhi_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_unpackhi_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (17 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_unpackhi_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_unpackhi_pd (a , b) ; let e = _mm512_set_pd (17. , 1. , 19. , 3. , 21. , 5. , 23. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_unpackhi_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_mask_unpackhi_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_unpackhi_pd (a , 0b11111111 , a , b) ; let e = _mm512_set_pd (17. , 1. , 19. , 3. , 21. , 5. , 23. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_unpackhi_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_maskz_unpackhi_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_unpackhi_pd (0b00001111 , a , b) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 21. , 5. , 23. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_unpackhi_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm256_mask_unpackhi_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_unpackhi_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (17. , 1. , 19. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_unpackhi_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm256_maskz_unpackhi_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_unpackhi_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (17. , 1. , 19. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_unpackhi_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (17. , 18.) ; let r = _mm_mask_unpackhi_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_unpackhi_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (17. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_unpackhi_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_unpackhi_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_unpackhi_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_unpackhi_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (17. , 18.) ; let r = _mm_maskz_unpackhi_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_unpackhi_pd (0b00000011 , a , b) ; let e = _mm_set_pd (17. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_unpacklo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_unpacklo_epi64 (a , b) ; let e = _mm512_set_epi64 (18 , 2 , 20 , 4 , 22 , 6 , 24 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_unpacklo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_mask_unpacklo_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_unpacklo_epi64 (a , 0b11111111 , a , b) ; let e = _mm512_set_epi64 (18 , 2 , 20 , 4 , 22 , 6 , 24 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_unpacklo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_maskz_unpacklo_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_unpacklo_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 22 , 6 , 24 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_unpacklo_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm256_mask_unpacklo_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_unpacklo_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (18 , 2 , 20 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_unpacklo_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (17 , 18 , 19 , 20) ; let r = _mm256_maskz_unpacklo_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_unpacklo_epi64 (0b00001111 , a , b) ; let e = _mm256_set_epi64x (18 , 2 , 20 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_unpacklo_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (17 , 18) ; let r = _mm_mask_unpacklo_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_unpacklo_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (18 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_unpacklo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_unpacklo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_unpacklo_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_unpacklo_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (17 , 18) ; let r = _mm_maskz_unpacklo_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_unpacklo_epi64 (0b00000011 , a , b) ; let e = _mm_set_epi64x (18 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_unpacklo_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_unpacklo_pd (a , b) ; let e = _mm512_set_pd (18. , 2. , 20. , 4. , 22. , 6. , 24. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_unpacklo_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_mask_unpacklo_pd (a , 0 , a , b) ; assert_eq_m512d (r , a) ; let r = _mm512_mask_unpacklo_pd (a , 0b11111111 , a , b) ; let e = _mm512_set_pd (18. , 2. , 20. , 4. , 22. , 6. , 24. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_unpacklo_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_maskz_unpacklo_pd (0 , a , b) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_unpacklo_pd (0b00001111 , a , b) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 22. , 6. , 24. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_unpacklo_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm256_mask_unpacklo_pd (a , 0 , a , b) ; assert_eq_m256d (r , a) ; let r = _mm256_mask_unpacklo_pd (a , 0b00001111 , a , b) ; let e = _mm256_set_pd (18. , 2. , 20. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_unpacklo_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (17. , 18. , 19. , 20.) ; let r = _mm256_maskz_unpacklo_pd (0 , a , b) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_unpacklo_pd (0b00001111 , a , b) ; let e = _mm256_set_pd (18. , 2. , 20. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_unpacklo_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (17. , 18.) ; let r = _mm_mask_unpacklo_pd (a , 0 , a , b) ; assert_eq_m128d (r , a) ; let r = _mm_mask_unpacklo_pd (a , 0b00000011 , a , b) ; let e = _mm_set_pd (18. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_unpacklo_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_unpacklo_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_unpacklo_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_unpacklo_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (17. , 18.) ; let r = _mm_maskz_unpacklo_pd (0 , a , b) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_unpacklo_pd (0b00000011 , a , b) ; let e = _mm_set_pd (18. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_alignr_epi64 () { let a = _mm512_set_epi64 (8 , 7 , 6 , 5 , 4 , 3 , 2 , 1) ; let b = _mm512_set_epi64 (16 , 15 , 14 , 13 , 12 , 11 , 10 , 9) ; let r = _mm512_alignr_epi64 :: < 0 > (a , b) ; assert_eq_m512i (r , b) ; let r = _mm512_alignr_epi64 :: < 8 > (a , b) ; assert_eq_m512i (r , b) ; let r = _mm512_alignr_epi64 :: < 1 > (a , b) ; let e = _mm512_set_epi64 (1 , 16 , 15 , 14 , 13 , 12 , 11 , 10) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_alignr_epi64 () { let a = _mm512_set_epi64 (8 , 7 , 6 , 5 , 4 , 3 , 2 , 1) ; let b = _mm512_set_epi64 (16 , 15 , 14 , 13 , 12 , 11 , 10 , 9) ; let r = _mm512_mask_alignr_epi64 :: < 1 > (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_alignr_epi64 :: < 1 > (a , 0b11111111 , a , b) ; let e = _mm512_set_epi64 (1 , 16 , 15 , 14 , 13 , 12 , 11 , 10) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_alignr_epi64 () { let a = _mm512_set_epi64 (8 , 7 , 6 , 5 , 4 , 3 , 2 , 1) ; let b = _mm512_set_epi64 (16 , 15 , 14 , 13 , 12 , 11 , 10 , 9) ; let r = _mm512_maskz_alignr_epi64 :: < 1 > (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_alignr_epi64 :: < 1 > (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 13 , 12 , 11 , 10) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_alignr_epi64 () { let a = _mm256_set_epi64x (4 , 3 , 2 , 1) ; let b = _mm256_set_epi64x (8 , 7 , 6 , 5) ; let r = _mm256_alignr_epi64 :: < 0 > (a , b) ; let e = _mm256_set_epi64x (8 , 7 , 6 , 5) ; assert_eq_m256i (r , e) ; let r = _mm256_alignr_epi64 :: < 1 > (a , b) ; let e = _mm256_set_epi64x (1 , 8 , 7 , 6) ; assert_eq_m256i (r , e) ; let r = _mm256_alignr_epi64 :: < 6 > (a , b) ; let e = _mm256_set_epi64x (2 , 1 , 8 , 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_alignr_epi64 () { let a = _mm256_set_epi64x (4 , 3 , 2 , 1) ; let b = _mm256_set_epi64x (8 , 7 , 6 , 5) ; let r = _mm256_mask_alignr_epi64 :: < 1 > (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_alignr_epi64 :: < 0 > (a , 0b00001111 , a , b) ; let e = _mm256_set_epi64x (8 , 7 , 6 , 5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_alignr_epi64 () { let a = _mm256_set_epi64x (4 , 3 , 2 , 1) ; let b = _mm256_set_epi64x (8 , 7 , 6 , 5) ; let r = _mm256_maskz_alignr_epi64 :: < 1 > (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_alignr_epi64 :: < 0 > (0b00001111 , a , b) ; let e = _mm256_set_epi64x (8 , 7 , 6 , 5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_alignr_epi64 () { let a = _mm_set_epi64x (2 , 1) ; let b = _mm_set_epi64x (4 , 3) ; let r = _mm_alignr_epi64 :: < 0 > (a , b) ; let e = _mm_set_epi64x (4 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_alignr_epi64 () { let a = _mm_set_epi64x (2 , 1) ; let b = _mm_set_epi64x (4 , 3) ; let r = _mm_mask_alignr_epi64 :: < 1 > (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_alignr_epi64 :: < 0 > (a , 0b00000011 , a , b) ; let e = _mm_set_epi64x (4 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_alignr_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_alignr_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_alignr_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_alignr_epi64 () { let a = _mm_set_epi64x (2 , 1) ; let b = _mm_set_epi64x (4 , 3) ; let r = _mm_maskz_alignr_epi64 :: < 1 > (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_alignr_epi64 :: < 0 > (0b00000011 , a , b) ; let e = _mm_set_epi64x (4 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_and_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_and_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_and_epi64 (a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_and_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_and_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_mask_and_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_and_epi64 (a , 0b01111111 , a , b) ; let e = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_and_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_and_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_maskz_and_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_and_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_and_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_and_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 0) ; let r = _mm256_mask_and_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_and_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_and_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_and_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 0) ; let r = _mm256_maskz_and_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_and_epi64 (0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_and_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_and_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 0) ; let r = _mm_mask_and_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_and_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_and_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_and_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 0) ; let r = _mm_maskz_and_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_and_epi64 (0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_and_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_and_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_and_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_and_si512 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_and_epi64 (a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_or_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_or_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_or_epi64 (a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_or_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_or_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_mask_or_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_or_epi64 (a , 0b11111111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_or_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_or_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_maskz_or_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_or_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_or_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_or_epi64 (a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_or_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_mask_or_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_or_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_or_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_maskz_or_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_or_epi64 (0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_or_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_or_epi64 (a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_or_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_mask_or_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_or_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_or_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_or_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_maskz_or_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_or_epi64 (0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_or_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_or_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_or_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_or_si512 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_or_epi64 (a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_xor_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_xor_epi64 (a , b) ; let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_xor_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_mask_xor_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_xor_epi64 (a , 0b11111111 , a , b) ; let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_xor_epi64 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_maskz_xor_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_xor_epi64 (0b00001111 , a , b) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_xor_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_xor_epi64 (a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_xor_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_mask_xor_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_xor_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_xor_epi64 () { let a = _mm256_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm256_set1_epi64x (1 << 13) ; let r = _mm256_maskz_xor_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_xor_epi64 (0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_xor_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_xor_epi64 (a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_xor_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_mask_xor_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_xor_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_xor_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_xor_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_xor_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_xor_epi64 () { let a = _mm_set1_epi64x (1 << 0 | 1 << 15) ; let b = _mm_set1_epi64x (1 << 13) ; let r = _mm_maskz_xor_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_xor_epi64 (0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 0 | 1 << 13 | 1 << 15) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_xor_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_xor_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_xor_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_xor_si512 () { let a = _mm512_set_epi64 (1 << 0 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let b = _mm512_set_epi64 (1 << 13 , 0 , 0 , 0 , 0 , 0 , 0 , 1 << 1 | 1 << 2 | 1 << 3) ; let r = _mm512_xor_epi64 (a , b) ; let e = _mm512_set_epi64 (1 << 0 | 1 << 13 | 1 << 15 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_andnot_epi64 () { let a = _mm512_set1_epi64 (0) ; let b = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; let r = _mm512_andnot_epi64 (a , b) ; let e = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_andnot_epi64 () { let a = _mm512_set1_epi64 (1 << 1 | 1 << 2) ; let b = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; let r = _mm512_mask_andnot_epi64 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_andnot_epi64 (a , 0b11111111 , a , b) ; let e = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_andnot_epi64 () { let a = _mm512_set1_epi64 (1 << 1 | 1 << 2) ; let b = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; let r = _mm512_maskz_andnot_epi64 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_andnot_epi64 (0b00001111 , a , b) ; #[rustfmt :: skip] let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 << 3 | 1 << 4 , 1 << 3 | 1 << 4 , 1 << 3 | 1 << 4 , 1 << 3 | 1 << 4 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_andnot_epi64 () { let a = _mm256_set1_epi64x (1 << 1 | 1 << 2) ; let b = _mm256_set1_epi64x (1 << 3 | 1 << 4) ; let r = _mm256_mask_andnot_epi64 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_andnot_epi64 (a , 0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 3 | 1 << 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_andnot_epi64 () { let a = _mm256_set1_epi64x (1 << 1 | 1 << 2) ; let b = _mm256_set1_epi64x (1 << 3 | 1 << 4) ; let r = _mm256_maskz_andnot_epi64 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_andnot_epi64 (0b00001111 , a , b) ; let e = _mm256_set1_epi64x (1 << 3 | 1 << 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_andnot_epi64 () { let a = _mm_set1_epi64x (1 << 1 | 1 << 2) ; let b = _mm_set1_epi64x (1 << 3 | 1 << 4) ; let r = _mm_mask_andnot_epi64 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_andnot_epi64 (a , 0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 3 | 1 << 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_andnot_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_andnot_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_andnot_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_andnot_epi64 () { let a = _mm_set1_epi64x (1 << 1 | 1 << 2) ; let b = _mm_set1_epi64x (1 << 3 | 1 << 4) ; let r = _mm_maskz_andnot_epi64 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_andnot_epi64 (0b00000011 , a , b) ; let e = _mm_set1_epi64x (1 << 3 | 1 << 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_andnot_si512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_andnot_si512 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_andnot_si512_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_andnot_si512 () { let a = _mm512_set1_epi64 (0) ; let b = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; let r = _mm512_andnot_si512 (a , b) ; let e = _mm512_set1_epi64 (1 << 3 | 1 << 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_reduce_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_add_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_add_epi64 () { let a = _mm512_set1_epi64 (1) ; let e : i64 = _mm512_reduce_add_epi64 (a) ; assert_eq ! (8 , e) ; }
}

macro_rules! test_mm512_mask_reduce_add_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_add_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_add_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_add_epi64 () { let a = _mm512_set1_epi64 (1) ; let e : i64 = _mm512_mask_reduce_add_epi64 (0b11110000 , a) ; assert_eq ! (4 , e) ; }
}

macro_rules! test_mm512_reduce_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_add_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_add_pd () { let a = _mm512_set1_pd (1.) ; let e : f64 = _mm512_reduce_add_pd (a) ; assert_eq ! (8. , e) ; }
}

macro_rules! test_mm512_mask_reduce_add_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_add_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_add_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_add_pd () { let a = _mm512_set1_pd (1.) ; let e : f64 = _mm512_mask_reduce_add_pd (0b11110000 , a) ; assert_eq ! (4. , e) ; }
}

macro_rules! test_mm512_reduce_mul_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_mul_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_mul_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_mul_epi64 () { let a = _mm512_set1_epi64 (2) ; let e : i64 = _mm512_reduce_mul_epi64 (a) ; assert_eq ! (256 , e) ; }
}

macro_rules! test_mm512_mask_reduce_mul_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_mul_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_mul_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_mul_epi64 () { let a = _mm512_set1_epi64 (2) ; let e : i64 = _mm512_mask_reduce_mul_epi64 (0b11110000 , a) ; assert_eq ! (16 , e) ; }
}

macro_rules! test_mm512_reduce_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_mul_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_mul_pd () { let a = _mm512_set1_pd (2.) ; let e : f64 = _mm512_reduce_mul_pd (a) ; assert_eq ! (256. , e) ; }
}

macro_rules! test_mm512_mask_reduce_mul_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_mul_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_mul_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_mul_pd () { let a = _mm512_set1_pd (2.) ; let e : f64 = _mm512_mask_reduce_mul_pd (0b11110000 , a) ; assert_eq ! (16. , e) ; }
}

macro_rules! test_mm512_reduce_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_max_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_max_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : i64 = _mm512_reduce_max_epi64 (a) ; assert_eq ! (7 , e) ; }
}

macro_rules! test_mm512_mask_reduce_max_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_max_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_max_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_max_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : i64 = _mm512_mask_reduce_max_epi64 (0b11110000 , a) ; assert_eq ! (3 , e) ; }
}

macro_rules! test_mm512_reduce_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_max_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_max_epu64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : u64 = _mm512_reduce_max_epu64 (a) ; assert_eq ! (7 , e) ; }
}

macro_rules! test_mm512_mask_reduce_max_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_max_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_max_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_max_epu64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : u64 = _mm512_mask_reduce_max_epu64 (0b11110000 , a) ; assert_eq ! (3 , e) ; }
}

macro_rules! test_mm512_reduce_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_max_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_max_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let e : f64 = _mm512_reduce_max_pd (a) ; assert_eq ! (7. , e) ; }
}

macro_rules! test_mm512_mask_reduce_max_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_max_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_max_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_max_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let e : f64 = _mm512_mask_reduce_max_pd (0b11110000 , a) ; assert_eq ! (3. , e) ; }
}

macro_rules! test_mm512_reduce_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_min_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_min_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : i64 = _mm512_reduce_min_epi64 (a) ; assert_eq ! (0 , e) ; }
}

macro_rules! test_mm512_mask_reduce_min_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_min_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_min_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_min_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : i64 = _mm512_mask_reduce_min_epi64 (0b11110000 , a) ; assert_eq ! (0 , e) ; }
}

macro_rules! test_mm512_reduce_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_min_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_min_epu64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : u64 = _mm512_reduce_min_epu64 (a) ; assert_eq ! (0 , e) ; }
}

macro_rules! test_mm512_mask_reduce_min_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_min_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_min_epu64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_min_epu64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let e : u64 = _mm512_mask_reduce_min_epu64 (0b11110000 , a) ; assert_eq ! (0 , e) ; }
}

macro_rules! test_mm512_reduce_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_min_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_min_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let e : f64 = _mm512_reduce_min_pd (a) ; assert_eq ! (0. , e) ; }
}

macro_rules! test_mm512_mask_reduce_min_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_min_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_min_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_min_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let e : f64 = _mm512_mask_reduce_min_pd (0b11110000 , a) ; assert_eq ! (0. , e) ; }
}

macro_rules! test_mm512_reduce_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_and_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_and_epi64 () { let a = _mm512_set_epi64 (1 , 1 , 1 , 1 , 2 , 2 , 2 , 2) ; let e : i64 = _mm512_reduce_and_epi64 (a) ; assert_eq ! (0 , e) ; }
}

macro_rules! test_mm512_mask_reduce_and_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_and_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_and_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_and_epi64 () { let a = _mm512_set_epi64 (1 , 1 , 1 , 1 , 2 , 2 , 2 , 2) ; let e : i64 = _mm512_mask_reduce_and_epi64 (0b11110000 , a) ; assert_eq ! (1 , e) ; }
}

macro_rules! test_mm512_reduce_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_or_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_reduce_or_epi64 () { let a = _mm512_set_epi64 (1 , 1 , 1 , 1 , 2 , 2 , 2 , 2) ; let e : i64 = _mm512_reduce_or_epi64 (a) ; assert_eq ! (3 , e) ; }
}

macro_rules! test_mm512_mask_reduce_or_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_or_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_or_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_reduce_or_epi64 () { let a = _mm512_set_epi64 (1 , 1 , 1 , 1 , 2 , 2 , 2 , 2) ; let e : i64 = _mm512_mask_reduce_or_epi64 (0b11110000 , a) ; assert_eq ! (1 , e) ; }
}

macro_rules! test_mm512_extractf64x4_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extractf64x4_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extractf64x4_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_extractf64x4_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_extractf64x4_pd :: < 1 > (a) ; let e = _mm256_setr_pd (5. , 6. , 7. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_mask_extractf64x4_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extractf64x4_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extractf64x4_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_extractf64x4_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let src = _mm256_set1_pd (100.) ; let r = _mm512_mask_extractf64x4_pd :: < 1 > (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm512_mask_extractf64x4_pd :: < 1 > (src , 0b11111111 , a) ; let e = _mm256_setr_pd (5. , 6. , 7. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_maskz_extractf64x4_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extractf64x4_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extractf64x4_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_extractf64x4_pd () { let a = _mm512_setr_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_extractf64x4_pd :: < 1 > (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm512_maskz_extractf64x4_pd :: < 1 > (0b00000001 , a) ; let e = _mm256_setr_pd (5. , 0. , 0. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_extracti64x4_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extracti64x4_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extracti64x4_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_extracti64x4_epi64 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_extracti64x4_epi64 :: < 0x1 > (a) ; let e = _mm256_setr_epi64x (5 , 6 , 7 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_extracti64x4_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extracti64x4_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extracti64x4_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_extracti64x4_epi64 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let src = _mm256_set1_epi64x (100) ; let r = _mm512_mask_extracti64x4_epi64 :: < 0x1 > (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm512_mask_extracti64x4_epi64 :: < 0x1 > (src , 0b11111111 , a) ; let e = _mm256_setr_epi64x (5 , 6 , 7 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_extracti64x4_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extracti64x4_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extracti64x4_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_extracti64x4_epi64 () { let a = _mm512_setr_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_extracti64x4_epi64 :: < 0x1 > (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm512_maskz_extracti64x4_epi64 :: < 0x1 > (0b00000001 , a) ; let e = _mm256_setr_epi64x (5 , 0 , 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_compress_epi64 () { let src = _mm512_set1_epi64 (200) ; let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_compress_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_compress_epi64 (src , 0b01010101 , a) ; let e = _mm512_set_epi64 (200 , 200 , 200 , 200 , 1 , 3 , 5 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_compress_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_maskz_compress_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_compress_epi64 (0b01010101 , a) ; let e = _mm512_set_epi64 (0 , 0 , 0 , 0 , 1 , 3 , 5 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_compress_epi64 () { let src = _mm256_set1_epi64x (200) ; let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_compress_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_compress_epi64 (src , 0b00000101 , a) ; let e = _mm256_set_epi64x (200 , 200 , 1 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_compress_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_compress_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_compress_epi64 (0b00000101 , a) ; let e = _mm256_set_epi64x (0 , 0 , 1 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_compress_epi64 () { let src = _mm_set1_epi64x (200) ; let a = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_compress_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_compress_epi64 (src , 0b00000001 , a) ; let e = _mm_set_epi64x (200 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_compress_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_compress_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_compress_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_compress_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_compress_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_compress_epi64 (0b00000001 , a) ; let e = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_compress_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_compress_pd () { let src = _mm512_set1_pd (200.) ; let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_mask_compress_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_compress_pd (src , 0b01010101 , a) ; let e = _mm512_set_pd (200. , 200. , 200. , 200. , 1. , 3. , 5. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_compress_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_compress_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_maskz_compress_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_compress_pd (0b01010101 , a) ; let e = _mm512_set_pd (0. , 0. , 0. , 0. , 1. , 3. , 5. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_compress_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_compress_pd () { let src = _mm256_set1_pd (200.) ; let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_mask_compress_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_compress_pd (src , 0b00000101 , a) ; let e = _mm256_set_pd (200. , 200. , 1. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_compress_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_compress_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_maskz_compress_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_compress_pd (0b00000101 , a) ; let e = _mm256_set_pd (0. , 0. , 1. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_compress_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_compress_pd () { let src = _mm_set1_pd (200.) ; let a = _mm_set_pd (0. , 1.) ; let r = _mm_mask_compress_pd (src , 0 , a) ; assert_eq_m128d (r , src) ; let r = _mm_mask_compress_pd (src , 0b00000001 , a) ; let e = _mm_set_pd (200. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_compress_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_compress_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_compress_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_compress_pd () { let a = _mm_set_pd (0. , 1.) ; let r = _mm_maskz_compress_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_compress_pd (0b00000001 , a) ; let e = _mm_set_pd (0. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_mask_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_expand_epi64 () { let src = _mm512_set1_epi64 (200) ; let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_mask_expand_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_expand_epi64 (src , 0b01010101 , a) ; let e = _mm512_set_epi64 (200 , 4 , 200 , 5 , 200 , 6 , 200 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_expand_epi64 () { let a = _mm512_set_epi64 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7) ; let r = _mm512_maskz_expand_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_expand_epi64 (0b01010101 , a) ; let e = _mm512_set_epi64 (0 , 4 , 0 , 5 , 0 , 6 , 0 , 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_expand_epi64 () { let src = _mm256_set1_epi64x (200) ; let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_mask_expand_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_expand_epi64 (src , 0b00000101 , a) ; let e = _mm256_set_epi64x (200 , 2 , 200 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_expand_epi64 () { let a = _mm256_set_epi64x (0 , 1 , 2 , 3) ; let r = _mm256_maskz_expand_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_expand_epi64 (0b00000101 , a) ; let e = _mm256_set_epi64x (0 , 2 , 0 , 3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_expand_epi64 () { let src = _mm_set1_epi64x (200) ; let a = _mm_set_epi64x (0 , 1) ; let r = _mm_mask_expand_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_expand_epi64 (src , 0b00000001 , a) ; let e = _mm_set_epi64x (200 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_expand_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_expand_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_expand_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_expand_epi64 () { let a = _mm_set_epi64x (0 , 1) ; let r = _mm_maskz_expand_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_expand_epi64 (0b00000001 , a) ; let e = _mm_set_epi64x (0 , 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_expand_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_expand_pd () { let src = _mm512_set1_pd (200.) ; let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_mask_expand_pd (src , 0 , a) ; assert_eq_m512d (r , src) ; let r = _mm512_mask_expand_pd (src , 0b01010101 , a) ; let e = _mm512_set_pd (200. , 4. , 200. , 5. , 200. , 6. , 200. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_expand_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_expand_pd () { let a = _mm512_set_pd (0. , 1. , 2. , 3. , 4. , 5. , 6. , 7.) ; let r = _mm512_maskz_expand_pd (0 , a) ; assert_eq_m512d (r , _mm512_setzero_pd ()) ; let r = _mm512_maskz_expand_pd (0b01010101 , a) ; let e = _mm512_set_pd (0. , 4. , 0. , 5. , 0. , 6. , 0. , 7.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm256_mask_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_expand_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_expand_pd () { let src = _mm256_set1_pd (200.) ; let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_mask_expand_pd (src , 0 , a) ; assert_eq_m256d (r , src) ; let r = _mm256_mask_expand_pd (src , 0b00000101 , a) ; let e = _mm256_set_pd (200. , 2. , 200. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_expand_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_expand_pd () { let a = _mm256_set_pd (0. , 1. , 2. , 3.) ; let r = _mm256_maskz_expand_pd (0 , a) ; assert_eq_m256d (r , _mm256_setzero_pd ()) ; let r = _mm256_maskz_expand_pd (0b00000101 , a) ; let e = _mm256_set_pd (0. , 2. , 0. , 3.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm_mask_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_expand_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_expand_pd () { let src = _mm_set1_pd (200.) ; let a = _mm_set_pd (0. , 1.) ; let r = _mm_mask_expand_pd (src , 0 , a) ; assert_eq_m128d (r , src) ; let r = _mm_mask_expand_pd (src , 0b00000001 , a) ; let e = _mm_set_pd (200. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_expand_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_expand_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_expand_pd_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_expand_pd () { let a = _mm_set_pd (0. , 1.) ; let r = _mm_maskz_expand_pd (0 , a) ; assert_eq_m128d (r , _mm_setzero_pd ()) ; let r = _mm_maskz_expand_pd (0b00000001 , a) ; let e = _mm_set_pd (0. , 1.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_loadu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_loadu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_loadu_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_loadu_epi64 () { let a = & [4 , 3 , 2 , 5 , - 8 , - 9 , - 64 , - 50] ; let p = a . as_ptr () ; let r = _mm512_loadu_epi64 (black_box (p)) ; let e = _mm512_setr_epi64 (4 , 3 , 2 , 5 , - 8 , - 9 , - 64 , - 50) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_loadu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_loadu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_loadu_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_loadu_epi64 () { let a = & [4 , 3 , 2 , 5] ; let p = a . as_ptr () ; let r = _mm256_loadu_epi64 (black_box (p)) ; let e = _mm256_setr_epi64x (4 , 3 , 2 , 5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_loadu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_loadu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_loadu_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_loadu_epi64 () { let a = & [4 , 3] ; let p = a . as_ptr () ; let r = _mm_loadu_epi64 (black_box (p)) ; let e = _mm_setr_epi64x (4 , 3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_storeu_epi16 () { let a = _mm512_set1_epi64 (9) ; let mut r = _mm_undefined_si128 () ; _mm512_mask_cvtepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set1_epi16 (9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_storeu_epi16 () { let a = _mm256_set1_epi64x (9) ; let mut r = _mm_set1_epi16 (0) ; _mm256_mask_cvtepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 9 , 9 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_storeu_epi16 () { let a = _mm_set1_epi64x (9) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_storeu_epi16 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm_undefined_si128 () ; _mm512_mask_cvtsepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set1_epi16 (i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_storeu_epi16 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm256_mask_cvtsepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , i16 :: MAX , i16 :: MAX , i16 :: MAX , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_storeu_epi16 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtsepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , i16 :: MAX , i16 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_storeu_epi16 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm_undefined_si128 () ; _mm512_mask_cvtusepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set1_epi16 (u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_storeu_epi16 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm256_mask_cvtusepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , u16 :: MAX as i16 , u16 :: MAX as i16 , u16 :: MAX as i16 , u16 :: MAX as i16 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_storeu_epi16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_storeu_epi16 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_storeu_epi16_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_storeu_epi16 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtusepi64_storeu_epi16 (& mut r as * mut _ as * mut i16 , 0b11111111 , a) ; let e = _mm_set_epi16 (0 , 0 , 0 , 0 , 0 , 0 , u16 :: MAX as i16 , u16 :: MAX as i16) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_storeu_epi8 () { let a = _mm512_set1_epi64 (9) ; let mut r = _mm_set1_epi8 (0) ; _mm512_mask_cvtepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_storeu_epi8 () { let a = _mm256_set1_epi64x (9) ; let mut r = _mm_set1_epi8 (0) ; _mm256_mask_cvtepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 9 , 9 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_storeu_epi8 () { let a = _mm_set1_epi64x (9) ; let mut r = _mm_set1_epi8 (0) ; _mm_mask_cvtepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_storeu_epi8 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm512_mask_cvtsepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_storeu_epi8 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm256_mask_cvtsepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MAX , i8 :: MAX , i8 :: MAX , i8 :: MAX ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_storeu_epi8 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm_mask_cvtsepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , i8 :: MAX , i8 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_storeu_epi8 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm512_mask_cvtusepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_storeu_epi8 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm256_mask_cvtusepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 , u8 :: MAX as i8 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_storeu_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_storeu_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_storeu_epi8_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_storeu_epi8 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi8 (0) ; _mm_mask_cvtusepi64_storeu_epi8 (& mut r as * mut _ as * mut i8 , 0b11111111 , a) ; #[rustfmt :: skip] let e = _mm_set_epi8 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , u8 :: MAX as i8 , u8 :: MAX as i8 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtepi64_storeu_epi32 () { let a = _mm512_set1_epi64 (9) ; let mut r = _mm256_undefined_si256 () ; _mm512_mask_cvtepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b11111111 , a) ; let e = _mm256_set1_epi32 (9) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_storeu_epi32 () { let a = _mm256_set1_epi64x (9) ; let mut r = _mm_set1_epi32 (0) ; _mm256_mask_cvtepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b11111111 , a) ; let e = _mm_set_epi32 (9 , 9 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtepi64_storeu_epi32 () { let a = _mm_set1_epi64x (9) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b11111111 , a) ; let e = _mm_set_epi32 (0 , 0 , 9 , 9) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtsepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtsepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtsepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtsepi64_storeu_epi32 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm256_undefined_si256 () ; _mm512_mask_cvtsepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b11111111 , a) ; let e = _mm256_set1_epi32 (i32 :: MAX) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtsepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtsepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtsepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtsepi64_storeu_epi32 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi32 (0) ; _mm256_mask_cvtsepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b00001111 , a) ; let e = _mm_set1_epi32 (i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtsepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtsepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtsepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtsepi64_storeu_epi32 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtsepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , i32 :: MAX , i32 :: MAX) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtusepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtusepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtusepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_cvtusepi64_storeu_epi32 () { let a = _mm512_set1_epi64 (i64 :: MAX) ; let mut r = _mm256_undefined_si256 () ; _mm512_mask_cvtusepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b11111111 , a) ; let e = _mm256_set1_epi32 (u32 :: MAX as i32) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtusepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtusepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtusepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_cvtusepi64_storeu_epi32 () { let a = _mm256_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi32 (0) ; _mm256_mask_cvtusepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b00001111 , a) ; let e = _mm_set1_epi32 (u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtusepi64_storeu_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtusepi64_storeu_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtusepi64_storeu_epi32_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_cvtusepi64_storeu_epi32 () { let a = _mm_set1_epi64x (i64 :: MAX) ; let mut r = _mm_set1_epi16 (0) ; _mm_mask_cvtusepi64_storeu_epi32 (& mut r as * mut _ as * mut i32 , 0b00000011 , a) ; let e = _mm_set_epi32 (0 , 0 , u32 :: MAX as i32 , u32 :: MAX as i32) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_storeu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_storeu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_storeu_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_storeu_epi64 () { let a = _mm512_set1_epi64 (9) ; let mut r = _mm512_set1_epi64 (0) ; _mm512_storeu_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m512i (r , a) ; }
}

macro_rules! test_mm256_storeu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_storeu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_storeu_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_storeu_epi64 () { let a = _mm256_set1_epi64x (9) ; let mut r = _mm256_set1_epi64x (0) ; _mm256_storeu_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m256i (r , a) ; }
}

macro_rules! test_mm_storeu_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_storeu_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_storeu_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_storeu_epi64 () { let a = _mm_set1_epi64x (9) ; let mut r = _mm_set1_epi64x (0) ; _mm_storeu_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm512_load_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_load_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_load_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_load_epi64 () { #[repr (align (64))] struct Align { data : [i64 ; 8] , } let a = Align { data : [4 , 3 , 2 , 5 , - 8 , - 9 , - 64 , - 50] , } ; let p = (a . data) . as_ptr () ; let r = _mm512_load_epi64 (black_box (p)) ; let e = _mm512_setr_epi64 (4 , 3 , 2 , 5 , - 8 , - 9 , - 64 , - 50) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_load_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_load_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_load_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_load_epi64 () { #[repr (align (64))] struct Align { data : [i64 ; 4] , } let a = Align { data : [4 , 3 , 2 , 5] } ; let p = (a . data) . as_ptr () ; let r = _mm256_load_epi64 (black_box (p)) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_load_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_load_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_load_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_load_epi64 () { #[repr (align (64))] struct Align { data : [i64 ; 2] , } let a = Align { data : [4 , 3] } ; let p = (a . data) . as_ptr () ; let r = _mm_load_epi64 (black_box (p)) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_store_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_store_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_store_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_store_epi64 () { let a = _mm512_set1_epi64 (9) ; let mut r = _mm512_set1_epi64 (0) ; _mm512_store_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m512i (r , a) ; }
}

macro_rules! test_mm256_store_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_store_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_store_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_store_epi64 () { let a = _mm256_set1_epi64x (9) ; let mut r = _mm256_set1_epi64x (0) ; _mm256_store_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m256i (r , a) ; }
}

macro_rules! test_mm_store_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_store_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_store_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_store_epi64 () { let a = _mm_set1_epi64x (9) ; let mut r = _mm_set1_epi64x (0) ; _mm_store_epi64 (& mut r as * mut _ as * mut i64 , a) ; assert_eq_m128i (r , a) ; }
}

macro_rules! test_mm512_load_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_load_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_load_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_load_pd () { #[repr (align (64))] struct Align { data : [f64 ; 8] , } let a = Align { data : [4. , 3. , 2. , 5. , - 8. , - 9. , - 64. , - 50.] , } ; let p = (a . data) . as_ptr () ; let r = _mm512_load_pd (black_box (p)) ; let e = _mm512_setr_pd (4. , 3. , 2. , 5. , - 8. , - 9. , - 64. , - 50.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_store_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_store_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_store_pd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_store_pd () { let a = _mm512_set1_pd (9.) ; let mut r = _mm512_undefined_pd () ; _mm512_store_pd (& mut r as * mut _ as * mut f64 , a) ; assert_eq_m512d (r , a) ; }
}

macro_rules! test_mm512_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_test_epi64_mask () { let a = _mm512_set1_epi64 (1 << 0) ; let b = _mm512_set1_epi64 (1 << 0 | 1 << 1) ; let r = _mm512_test_epi64_mask (a , b) ; let e : __mmask8 = 0b11111111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_mask_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_test_epi64_mask () { let a = _mm512_set1_epi64 (1 << 0) ; let b = _mm512_set1_epi64 (1 << 0 | 1 << 1) ; let r = _mm512_mask_test_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm512_mask_test_epi64_mask (0b11111111 , a , b) ; let e : __mmask8 = 0b11111111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_test_epi64_mask () { let a = _mm256_set1_epi64x (1 << 0) ; let b = _mm256_set1_epi64x (1 << 0 | 1 << 1) ; let r = _mm256_test_epi64_mask (a , b) ; let e : __mmask8 = 0b00001111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_mask_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_test_epi64_mask () { let a = _mm256_set1_epi64x (1 << 0) ; let b = _mm256_set1_epi64x (1 << 0 | 1 << 1) ; let r = _mm256_mask_test_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm256_mask_test_epi64_mask (0b00001111 , a , b) ; let e : __mmask8 = 0b00001111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_test_epi64_mask () { let a = _mm_set1_epi64x (1 << 0) ; let b = _mm_set1_epi64x (1 << 0 | 1 << 1) ; let r = _mm_test_epi64_mask (a , b) ; let e : __mmask8 = 0b00000011 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_test_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_test_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_test_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_test_epi64_mask () { let a = _mm_set1_epi64x (1 << 0) ; let b = _mm_set1_epi64x (1 << 0 | 1 << 1) ; let r = _mm_mask_test_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm_mask_test_epi64_mask (0b00000011 , a , b) ; let e : __mmask8 = 0b00000011 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_testn_epi64_mask () { let a = _mm512_set1_epi64 (1 << 0) ; let b = _mm512_set1_epi64 (1 << 0 | 1 << 1) ; let r = _mm512_testn_epi64_mask (a , b) ; let e : __mmask8 = 0b00000000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_mask_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_testn_epi64_mask () { let a = _mm512_set1_epi64 (1 << 0) ; let b = _mm512_set1_epi64 (1 << 1) ; let r = _mm512_mask_testn_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm512_mask_testn_epi64_mask (0b11111111 , a , b) ; let e : __mmask8 = 0b11111111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_testn_epi64_mask () { let a = _mm256_set1_epi64x (1 << 0) ; let b = _mm256_set1_epi64x (1 << 1) ; let r = _mm256_testn_epi64_mask (a , b) ; let e : __mmask8 = 0b00001111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_mask_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_testn_epi64_mask () { let a = _mm256_set1_epi64x (1 << 0) ; let b = _mm256_set1_epi64x (1 << 1) ; let r = _mm256_mask_testn_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm256_mask_testn_epi64_mask (0b11111111 , a , b) ; let e : __mmask8 = 0b00001111 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_testn_epi64_mask () { let a = _mm_set1_epi64x (1 << 0) ; let b = _mm_set1_epi64x (1 << 1) ; let r = _mm_testn_epi64_mask (a , b) ; let e : __mmask8 = 0b00000011 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_testn_epi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_testn_epi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_testn_epi64_mask_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_testn_epi64_mask () { let a = _mm_set1_epi64x (1 << 0) ; let b = _mm_set1_epi64x (1 << 1) ; let r = _mm_mask_testn_epi64_mask (0 , a , b) ; assert_eq ! (r , 0) ; let r = _mm_mask_testn_epi64_mask (0b11111111 , a , b) ; let e : __mmask8 = 0b00000011 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_mask_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_mask_set1_epi64 () { let src = _mm512_set1_epi64 (2) ; let a : i64 = 11 ; let r = _mm512_mask_set1_epi64 (src , 0 , a) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_set1_epi64 (src , 0b11111111 , a) ; let e = _mm512_set1_epi64 (11) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm512_maskz_set1_epi64 () { let a : i64 = 11 ; let r = _mm512_maskz_set1_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_set1_epi64 (0b11111111 , a) ; let e = _mm512_set1_epi64 (11) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_mask_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_mask_set1_epi64 () { let src = _mm256_set1_epi64x (2) ; let a : i64 = 11 ; let r = _mm256_mask_set1_epi64 (src , 0 , a) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_set1_epi64 (src , 0b00001111 , a) ; let e = _mm256_set1_epi64x (11) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm256_maskz_set1_epi64 () { let a : i64 = 11 ; let r = _mm256_maskz_set1_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_set1_epi64 (0b00001111 , a) ; let e = _mm256_set1_epi64x (11) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_mask_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_mask_set1_epi64 () { let src = _mm_set1_epi64x (2) ; let a : i64 = 11 ; let r = _mm_mask_set1_epi64 (src , 0 , a) ; assert_eq_m128i (r , src) ; let r = _mm_mask_set1_epi64 (src , 0b00000011 , a) ; let e = _mm_set1_epi64x (11) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_set1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_set1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_set1_epi64_introspect!();
    #[simd_test (enable = "avx512f,avx512vl")] unsafe fn test_mm_maskz_set1_epi64 () { let a : i64 = 11 ; let r = _mm_maskz_set1_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_set1_epi64 (0b00000011 , a) ; let e = _mm_set1_epi64x (11) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_cvtsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtsd_i64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvtsd_i64 (a) ; let e : i64 = - 2 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtss_i64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvtss_i64 (a) ; let e : i64 = - 2 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundi64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundi64_ss_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundi64_ss () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvt_roundi64_ss :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_ps (0. , - 0.5 , 1. , 9.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cvt_roundsi64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundsi64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundsi64_ss_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundsi64_ss () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvt_roundsi64_ss :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_ps (0. , - 0.5 , 1. , 9.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cvti64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvti64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvti64_ss_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvti64_ss () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvti64_ss (a , b) ; let e = _mm_set_ps (0. , - 0.5 , 1. , 9.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cvti64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvti64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvti64_sd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvti64_sd () { let a = _mm_set_pd (1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvti64_sd (a , b) ; let e = _mm_set_pd (1. , 9.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_cvt_roundsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundsd_si64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundsd_si64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvt_roundsd_si64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundsd_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundsd_i64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvt_roundsd_i64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundsd_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundsd_u64 () { let a = _mm_set_pd (1. , f64 :: MAX) ; let r = _mm_cvt_roundsd_u64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtsd_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtsd_u64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvtsd_u64 (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundss_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundss_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundss_i64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvt_roundss_i64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundss_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundss_si64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundss_si64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvt_roundss_si64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvt_roundss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundss_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundss_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundss_u64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvt_roundss_u64 :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtss_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtss_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtss_u64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvtss_u64 (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvttsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttsd_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvttsd_i64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvttsd_i64 (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundsd_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundsd_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundsd_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundsd_i64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvtt_roundsd_i64 :: < _MM_FROUND_NO_EXC > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundsd_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundsd_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundsd_si64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundsd_si64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvtt_roundsd_si64 :: < _MM_FROUND_NO_EXC > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundsd_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundsd_u64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvtt_roundsd_u64 :: < _MM_FROUND_NO_EXC > (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvttsd_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttsd_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttsd_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvttsd_u64 () { let a = _mm_set_pd (1. , - 1.5) ; let r = _mm_cvttsd_u64 (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvttss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttss_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttss_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvttss_i64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvttss_i64 (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundss_i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundss_i64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundss_i64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundss_i64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvtt_roundss_i64 :: < _MM_FROUND_NO_EXC > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundss_si64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundss_si64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundss_si64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundss_si64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvtt_roundss_si64 :: < _MM_FROUND_NO_EXC > (a) ; let e : i64 = - 1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtt_roundss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtt_roundss_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtt_roundss_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtt_roundss_u64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvtt_roundss_u64 :: < _MM_FROUND_NO_EXC > (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvttss_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttss_u64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttss_u64_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvttss_u64 () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let r = _mm_cvttss_u64 (a) ; let e : u64 = u64 :: MAX ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_cvtu64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtu64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtu64_ss_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtu64_ss () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let b : u64 = 9 ; let r = _mm_cvtu64_ss (a , b) ; let e = _mm_set_ps (0. , - 0.5 , 1. , 9.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cvtu64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtu64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtu64_sd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvtu64_sd () { let a = _mm_set_pd (1. , - 1.5) ; let b : u64 = 9 ; let r = _mm_cvtu64_sd (a , b) ; let e = _mm_set_pd (1. , 9.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_cvt_roundu64_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundu64_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundu64_ss_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundu64_ss () { let a = _mm_set_ps (0. , - 0.5 , 1. , - 1.5) ; let b : u64 = 9 ; let r = _mm_cvt_roundu64_ss :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_ps (0. , - 0.5 , 1. , 9.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_cvt_roundu64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundu64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundu64_sd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundu64_sd () { let a = _mm_set_pd (1. , - 1.5) ; let b : u64 = 9 ; let r = _mm_cvt_roundu64_sd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_pd (1. , 9.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_cvt_roundi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundi64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundi64_sd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundi64_sd () { let a = _mm_set_pd (1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvt_roundi64_sd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_pd (1. , 9.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_cvt_roundsi64_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvt_roundsi64_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvt_roundsi64_sd_introspect!();
    #[simd_test (enable = "avx512f")] unsafe fn test_mm_cvt_roundsi64_sd () { let a = _mm_set_pd (1. , - 1.5) ; let b : i64 = 9 ; let r = _mm_cvt_roundsi64_sd :: < { _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC } > (a , b) ; let e = _mm_set_pd (1. , 9.) ; assert_eq_m128d (r , e) ; }
} 
            }}