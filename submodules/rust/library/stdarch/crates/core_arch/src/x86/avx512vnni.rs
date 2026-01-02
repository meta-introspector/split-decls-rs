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
mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkuse!{use crate :: intrinsics :: simd :: * ;}
mkuse!{#[cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm512_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_dpwssd_epi32&expand=2219)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm512_dpwssd_epi32 (src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpdpwssd (src . as_i32x16 () , a . as_i32x16 () , b . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_dpwssd_epi32&expand=2220)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm512_mask_dpwssd_epi32 (src : __m512i , k : __mmask16 , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpwssd_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_dpwssd_epi32&expand=2221)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm512_maskz_dpwssd_epi32 (k : __mmask16 , src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpwssd_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_dpwssd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwssd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwssd_avx_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwssd_avx_epi32&expand=2713)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm256_dpwssd_avx_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwssd256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwssd_epi32&expand=2216)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm256_dpwssd_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwssd256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_dpwssd_epi32&expand=2217)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm256_mask_dpwssd_epi32 (src : __m256i , k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpwssd_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_dpwssd_epi32&expand=2218)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm256_maskz_dpwssd_epi32 (k : __mmask8 , src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpwssd_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , i32x8 :: ZERO)) } }
}

macro_rules! _mm_dpwssd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwssd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwssd_avx_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwssd_avx_epi32&expand=2712)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm_dpwssd_avx_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwssd128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwssd_epi32&expand=2213)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm_dpwssd_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwssd128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_dpwssd_epi32&expand=2214)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm_mask_dpwssd_epi32 (src : __m128i , k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpwssd_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_dpwssd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_dpwssd_epi32&expand=2215)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssd))] pub fn _mm_maskz_dpwssd_epi32 (k : __mmask8 , src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpwssd_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , i32x4 :: ZERO)) } }
}

macro_rules! _mm512_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_dpwssds_epi32&expand=2228)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm512_dpwssds_epi32 (src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpdpwssds (src . as_i32x16 () , a . as_i32x16 () , b . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_dpwssds_epi32&expand=2229)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm512_mask_dpwssds_epi32 (src : __m512i , k : __mmask16 , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpwssds_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_dpwssds_epi32&expand=2230)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm512_maskz_dpwssds_epi32 (k : __mmask16 , src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpwssds_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_dpwssds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwssds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwssds_avx_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwssds_avx_epi32&expand=2726)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm256_dpwssds_avx_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwssds256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwssds_epi32&expand=2225)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm256_dpwssds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwssds256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_dpwssds_epi32&expand=2226)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm256_mask_dpwssds_epi32 (src : __m256i , k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpwssds_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_dpwssds_epi32&expand=2227)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm256_maskz_dpwssds_epi32 (k : __mmask8 , src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpwssds_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , i32x8 :: ZERO)) } }
}

macro_rules! _mm_dpwssds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwssds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwssds_avx_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwssds_avx_epi32&expand=2725)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm_dpwssds_avx_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwssds128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwssds_epi32&expand=2222)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm_dpwssds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwssds128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_dpwssds_epi32&expand=2223)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm_mask_dpwssds_epi32 (src : __m128i , k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpwssds_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_dpwssds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding 16-bit integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_dpwssds_epi32&expand=2224)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpwssds))] pub fn _mm_maskz_dpwssds_epi32 (k : __mmask8 , src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpwssds_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , i32x4 :: ZERO)) } }
}

macro_rules! _mm512_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_dpbusd_epi32&expand=2201)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm512_dpbusd_epi32 (src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpdpbusd (src . as_i32x16 () , a . as_i32x16 () , b . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_dpbusd_epi32&expand=2202)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm512_mask_dpbusd_epi32 (src : __m512i , k : __mmask16 , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpbusd_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_dpbusd_epi32&expand=2203)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm512_maskz_dpbusd_epi32 (k : __mmask16 , src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpbusd_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_dpbusd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbusd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbusd_avx_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbusd_avx_epi32&expand=2683)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm256_dpbusd_avx_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbusd256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbusd_epi32&expand=2198)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm256_dpbusd_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbusd256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_dpbusd_epi32&expand=2199)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm256_mask_dpbusd_epi32 (src : __m256i , k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpbusd_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_dpbusd_epi32&expand=2200)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm256_maskz_dpbusd_epi32 (k : __mmask8 , src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpbusd_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , i32x8 :: ZERO)) } }
}

macro_rules! _mm_dpbusd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbusd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbusd_avx_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbusd_avx_epi32&expand=2682)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm_dpbusd_avx_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbusd128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbusd_epi32&expand=2195)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm_dpbusd_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbusd128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_dpbusd_epi32&expand=2196)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm_mask_dpbusd_epi32 (src : __m128i , k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpbusd_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_dpbusd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_dpbusd_epi32&expand=2197)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusd))] pub fn _mm_maskz_dpbusd_epi32 (k : __mmask8 , src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpbusd_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , i32x4 :: ZERO)) } }
}

macro_rules! _mm512_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_dpbusds_epi32&expand=2210)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm512_dpbusds_epi32 (src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpdpbusds (src . as_i32x16 () , a . as_i32x16 () , b . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_dpbusds_epi32&expand=2211)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm512_mask_dpbusds_epi32 (src : __m512i , k : __mmask16 , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpbusds_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_dpbusds_epi32&expand=2212)"] #[inline] #[target_feature (enable = "avx512vnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm512_maskz_dpbusds_epi32 (k : __mmask16 , src : __m512i , a : __m512i , b : __m512i) -> __m512i { unsafe { let r = _mm512_dpbusds_epi32 (src , a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , r , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_dpbusds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbusds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbusds_avx_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbusds_avx_epi32&expand=2696)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm256_dpbusds_avx_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbusds256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbusds_epi32&expand=2207)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm256_dpbusds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbusds256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_dpbusds_epi32&expand=2208)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm256_mask_dpbusds_epi32 (src : __m256i , k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpbusds_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_dpbusds_epi32&expand=2209)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm256_maskz_dpbusds_epi32 (k : __mmask8 , src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { let r = _mm256_dpbusds_epi32 (src , a , b) . as_i32x8 () ; transmute (simd_select_bitmask (k , r , i32x8 :: ZERO)) } }
}

macro_rules! _mm_dpbusds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbusds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbusds_avx_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbusds_avx_epi32&expand=2695)"] #[inline] #[target_feature (enable = "avxvnni")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm_dpbusds_avx_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbusds128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbusds_epi32&expand=2204)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm_dpbusds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbusds128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_dpbusds_epi32&expand=2205)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm_mask_dpbusds_epi32 (src : __m128i , k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpbusds_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_dpbusds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding signed 8-bit integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding 32-bit integer in src using signed saturation, and store the packed 32-bit results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_dpbusds_epi32&expand=2206)"] #[inline] #[target_feature (enable = "avx512vnni,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpdpbusds))] pub fn _mm_maskz_dpbusds_epi32 (k : __mmask8 , src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { let r = _mm_dpbusds_epi32 (src , a , b) . as_i32x4 () ; transmute (simd_select_bitmask (k , r , i32x4 :: ZERO)) } }
}

macro_rules! _mm_dpbssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbssd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding signed 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbssd_epi32&expand=2674)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbssd))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbssd_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbssd_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbssd_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding signed 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbssd_epi32&expand=2675)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbssd))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbssd_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbssd_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpbssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbssds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding signed 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbssds_epi32&expand=2676)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbssds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbssds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbssds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbssds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding signed 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbssds_epi32&expand=2677)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbssds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbssds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbssds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpbsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbsud_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbsud_epi32&expand=2678)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbsud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbsud_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbsud_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbsud_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbsud_epi32&expand=2679)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbsud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbsud_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbsud_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpbsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbsuds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbsuds_epi32&expand=2680)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbsuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbsuds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbsuds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbsuds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbsuds_epi32&expand=2681)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbsuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbsuds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbsuds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpbuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbuud_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbuud_epi32&expand=2708)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbuud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbuud_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbuud_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbuud_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbuud_epi32&expand=2709)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbuud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbuud_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbuud_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpbuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpbuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpbuuds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpbuuds_epi32&expand=2710)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbuuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpbuuds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpbuuds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpbuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpbuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpbuuds_epi32_introspect!();
    #[doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding unsigned 8-bit"] #[doc = " integers in b, producing 4 intermediate signed 16-bit results. Sum these 4 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpbuuds_epi32&expand=2711)"] #[inline] #[target_feature (enable = "avxvnniint8")] #[cfg_attr (test , assert_instr (vpdpbuuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpbuuds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpbuuds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwsud_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwsud_epi32&expand=2738)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwsud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwsud_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwsud_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwsud_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwsud_epi32&expand=2739)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwsud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwsud_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwsud_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwsuds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwsuds_epi32&expand=2740)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwsuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwsuds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwsuds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwsuds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of signed 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwsuds_epi32&expand=2741)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwsuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwsuds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwsuds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwusd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding signed 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwusd_epi32&expand=2742)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwusd))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwusd_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwusd_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwusd_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding signed 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwusd_epi32&expand=2743)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwusd))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwusd_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwusd_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwusds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding signed 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwusds_epi32&expand=2744)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwusds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwusds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwusds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwusds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding signed 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwusds_epi32&expand=2745)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwusds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwusds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwusds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwuud_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwuud_epi32&expand=2746)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwuud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwuud_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwuud_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwuud_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwuud_epi32&expand=2747)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwuud))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwuud_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwuud_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_dpwuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_dpwuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_dpwuuds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_dpwuuds_epi32&expand=2748)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwuuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_dpwuuds_epi32 (src : __m128i , a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpdpwuuds_128 (src . as_i32x4 () , a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_dpwuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_dpwuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_dpwuuds_epi32_introspect!();
    #[doc = " Multiply groups of 2 adjacent pairs of unsigned 16-bit integers in a with corresponding unsigned 16-bit"] #[doc = " integers in b, producing 2 intermediate signed 32-bit results. Sum these 2 results with the corresponding"] #[doc = " 32-bit integer in src with signed saturation, and store the packed 32-bit results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_dpwuuds_epi32&expand=2749)"] #[inline] #[target_feature (enable = "avxvnniint16")] #[cfg_attr (test , assert_instr (vpdpwuuds))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_dpwuuds_epi32 (src : __m256i , a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpdpwuuds_256 (src . as_i32x8 () , a . as_i32x8 () , b . as_i32x8 ())) } }
}
mkitem!{#[allow (improper_ctypes)] unsafe extern "C" { #[link_name = "llvm.x86.avx512.vpdpwssd.512"] fn vpdpwssd (src : i32x16 , a : i32x16 , b : i32x16) -> i32x16 ; #[link_name = "llvm.x86.avx512.vpdpwssd.256"] fn vpdpwssd256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx512.vpdpwssd.128"] fn vpdpwssd128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx512.vpdpwssds.512"] fn vpdpwssds (src : i32x16 , a : i32x16 , b : i32x16) -> i32x16 ; #[link_name = "llvm.x86.avx512.vpdpwssds.256"] fn vpdpwssds256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx512.vpdpwssds.128"] fn vpdpwssds128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx512.vpdpbusd.512"] fn vpdpbusd (src : i32x16 , a : i32x16 , b : i32x16) -> i32x16 ; #[link_name = "llvm.x86.avx512.vpdpbusd.256"] fn vpdpbusd256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx512.vpdpbusd.128"] fn vpdpbusd128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx512.vpdpbusds.512"] fn vpdpbusds (src : i32x16 , a : i32x16 , b : i32x16) -> i32x16 ; #[link_name = "llvm.x86.avx512.vpdpbusds.256"] fn vpdpbusds256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx512.vpdpbusds.128"] fn vpdpbusds128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbssd.128"] fn vpdpbssd_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbssd.256"] fn vpdpbssd_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpbssds.128"] fn vpdpbssds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbssds.256"] fn vpdpbssds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpbsud.128"] fn vpdpbsud_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbsud.256"] fn vpdpbsud_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpbsuds.128"] fn vpdpbsuds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbsuds.256"] fn vpdpbsuds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpbuud.128"] fn vpdpbuud_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbuud.256"] fn vpdpbuud_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpbuuds.128"] fn vpdpbuuds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpbuuds.256"] fn vpdpbuuds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwsud.128"] fn vpdpwsud_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwsud.256"] fn vpdpwsud_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwsuds.128"] fn vpdpwsuds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwsuds.256"] fn vpdpwsuds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwusd.128"] fn vpdpwusd_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwusd.256"] fn vpdpwusd_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwusds.128"] fn vpdpwusds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwusds.256"] fn vpdpwusds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwuud.128"] fn vpdpwuud_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwuud.256"] fn vpdpwuud_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.avx2.vpdpwuuds.128"] fn vpdpwuuds_128 (src : i32x4 , a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.avx2.vpdpwuuds.256"] fn vpdpwuuds_256 (src : i32x8 , a : i32x8 , b : i32x8) -> i32x8 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm512_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_dpwssd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_dpwssd_epi32 (src , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_mask_dpwssd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_mask_dpwssd_epi32 (src , 0b00000000_00000000 , a , b) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_dpwssd_epi32 (src , 0b11111111_11111111 , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_maskz_dpwssd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_maskz_dpwssd_epi32 (0b00000000_00000000 , src , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_dpwssd_epi32 (0b11111111_11111111 , src , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_dpwssd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwssd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwssd_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm256_dpwssd_avx_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwssd_avx_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_dpwssd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwssd_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_mask_dpwssd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_mask_dpwssd_epi32 (src , 0b00000000 , a , b) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_dpwssd_epi32 (src , 0b11111111 , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_maskz_dpwssd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_maskz_dpwssd_epi32 (0b00000000 , src , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_dpwssd_epi32 (0b11111111 , src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwssd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwssd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwssd_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm_dpwssd_avx_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwssd_avx_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_dpwssd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwssd_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_mask_dpwssd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_mask_dpwssd_epi32 (src , 0b00000000 , a , b) ; assert_eq_m128i (r , src) ; let r = _mm_mask_dpwssd_epi32 (src , 0b00001111 , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_dpwssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_dpwssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_dpwssd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_maskz_dpwssd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_maskz_dpwssd_epi32 (0b00000000 , src , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_dpwssd_epi32 (0b00001111 , src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_dpwssds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_dpwssds_epi32 (src , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_mask_dpwssds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_mask_dpwssds_epi32 (src , 0b00000000_00000000 , a , b) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_dpwssds_epi32 (src , 0b11111111_11111111 , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_maskz_dpwssds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm512_maskz_dpwssds_epi32 (0b00000000_00000000 , src , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_dpwssds_epi32 (0b11111111_11111111 , src , a , b) ; let e = _mm512_set1_epi32 (3) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_dpwssds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwssds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwssds_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm256_dpwssds_avx_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwssds_avx_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_dpwssds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwssds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_mask_dpwssds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_mask_dpwssds_epi32 (src , 0b00000000 , a , b) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_dpwssds_epi32 (src , 0b11111111 , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_maskz_dpwssds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_maskz_dpwssds_epi32 (0b00000000 , src , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_dpwssds_epi32 (0b11111111 , src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwssds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwssds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwssds_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm_dpwssds_avx_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwssds_avx_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_dpwssds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwssds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_mask_dpwssds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_mask_dpwssds_epi32 (src , 0b00000000 , a , b) ; assert_eq_m128i (r , src) ; let r = _mm_mask_dpwssds_epi32 (src , 0b00001111 , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_dpwssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_dpwssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_dpwssds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_maskz_dpwssds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_maskz_dpwssds_epi32 (0b00000000 , src , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_dpwssds_epi32 (0b00001111 , src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_dpbusd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_dpbusd_epi32 (src , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_mask_dpbusd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_mask_dpbusd_epi32 (src , 0b00000000_00000000 , a , b) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_dpbusd_epi32 (src , 0b11111111_11111111 , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_maskz_dpbusd_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_maskz_dpbusd_epi32 (0b00000000_00000000 , src , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_dpbusd_epi32 (0b11111111_11111111 , src , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_dpbusd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbusd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbusd_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm256_dpbusd_avx_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbusd_avx_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_dpbusd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbusd_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_mask_dpbusd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_mask_dpbusd_epi32 (src , 0b00000000 , a , b) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_dpbusd_epi32 (src , 0b11111111 , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_maskz_dpbusd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_maskz_dpbusd_epi32 (0b00000000 , src , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_dpbusd_epi32 (0b11111111 , src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbusd_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbusd_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbusd_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm_dpbusd_avx_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbusd_avx_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_dpbusd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbusd_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_mask_dpbusd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_mask_dpbusd_epi32 (src , 0b00000000 , a , b) ; assert_eq_m128i (r , src) ; let r = _mm_mask_dpbusd_epi32 (src , 0b00001111 , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_dpbusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_dpbusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_dpbusd_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_maskz_dpbusd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_maskz_dpbusd_epi32 (0b00000000 , src , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_dpbusd_epi32 (0b00001111 , src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_dpbusds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_dpbusds_epi32 (src , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_mask_dpbusds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_mask_dpbusds_epi32 (src , 0b00000000_00000000 , a , b) ; assert_eq_m512i (r , src) ; let r = _mm512_mask_dpbusds_epi32 (src , 0b11111111_11111111 , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni")] unsafe fn test_mm512_maskz_dpbusds_epi32 () { let src = _mm512_set1_epi32 (1) ; let a = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm512_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm512_maskz_dpbusds_epi32 (0b00000000_00000000 , src , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_dpbusds_epi32 (0b11111111_11111111 , src , a , b) ; let e = _mm512_set1_epi32 (5) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_dpbusds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbusds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbusds_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm256_dpbusds_avx_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbusds_avx_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_dpbusds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbusds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_mask_dpbusds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_mask_dpbusds_epi32 (src , 0b00000000 , a , b) ; assert_eq_m256i (r , src) ; let r = _mm256_mask_dpbusds_epi32 (src , 0b11111111 , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm256_maskz_dpbusds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_maskz_dpbusds_epi32 (0b00000000 , src , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_dpbusds_epi32 (0b11111111 , src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbusds_avx_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbusds_avx_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbusds_avx_epi32_introspect!();
    #[simd_test (enable = "avxvnni")] unsafe fn test_mm_dpbusds_avx_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbusds_avx_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_dpbusds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbusds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_mask_dpbusds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_mask_dpbusds_epi32 (src , 0b00000000 , a , b) ; assert_eq_m128i (r , src) ; let r = _mm_mask_dpbusds_epi32 (src , 0b00001111 , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_dpbusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_dpbusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_dpbusds_epi32_introspect!();
    #[simd_test (enable = "avx512vnni,avx512vl")] unsafe fn test_mm_maskz_dpbusds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_maskz_dpbusds_epi32 (0b00000000 , src , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_dpbusds_epi32 (0b00001111 , src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_dpbssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbssd_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbssd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbssd_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbssd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbssd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbssd_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbssd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbssd_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbssds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbssds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbssds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbssds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbssds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbssds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbssds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbssds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbsud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbsud_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbsud_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbsud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbsud_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbsud_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbsuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbsuds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbsuds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbsuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbsuds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbsuds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbuud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbuud_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbuud_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbuud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbuud_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbuud_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpbuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpbuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpbuuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm_dpbuuds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm_dpbuuds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (5) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpbuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpbuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpbuuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint8")] unsafe fn test_mm256_dpbuuds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 24 | 1 << 16 | 1 << 8 | 1 << 0) ; let r = _mm256_dpbuuds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (5) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwsud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwsud_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwsud_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwsud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwsud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwsud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwsud_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwsud_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwsuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwsuds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwsuds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwsuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwsuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwsuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwsuds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwsuds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwusd_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwusd_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwusd_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwusd_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwusd_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwusd_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwusd_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwusd_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwusds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwusds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwusds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwusds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwusds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwusds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwusds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwusds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwuud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwuud_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwuud_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwuud_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwuud_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwuud_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwuud_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwuud_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_dpwuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_dpwuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_dpwuuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm_dpwuuds_epi32 () { let src = _mm_set1_epi32 (1) ; let a = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm_dpwuuds_epi32 (src , a , b) ; let e = _mm_set1_epi32 (3) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_dpwuuds_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_dpwuuds_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_dpwuuds_epi32_introspect!();
    #[simd_test (enable = "avxvnniint16")] unsafe fn test_mm256_dpwuuds_epi32 () { let src = _mm256_set1_epi32 (1) ; let a = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let b = _mm256_set1_epi32 (1 << 16 | 1 << 0) ; let r = _mm256_dpwuuds_epi32 (src , a , b) ; let e = _mm256_set1_epi32 (3) ; assert_eq_m256i (r , e) ; }
} 
            }}