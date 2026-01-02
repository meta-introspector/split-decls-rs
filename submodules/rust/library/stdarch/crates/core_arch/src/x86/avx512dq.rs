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
mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , intrinsics :: simd :: * , mem :: transmute , } ;}

macro_rules! _mm_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_and_pd&ig_expand=288)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_and_pd (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let and = _mm_and_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , and , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_and_pd&ig_expand=289)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_and_pd (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let and = _mm_and_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , and , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_and_pd&ig_expand=291)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_and_pd (src : __m256d , k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let and = _mm256_and_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , and , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_and_pd&ig_expand=292)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_and_pd (k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let and = _mm256_and_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , and , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_and_pd&ig_expand=293)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandp))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_and_pd (a : __m512d , b : __m512d) -> __m512d { unsafe { transmute (simd_and (transmute :: < _ , u64x8 > (a) , transmute :: < _ , u64x8 > (b))) } }
}

macro_rules! _mm512_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_and_pd&ig_expand=294)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_and_pd (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let and = _mm512_and_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , and , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_and_pd_introspect!();
    # [doc = " Compute the bitwise AND of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_and_pd&ig_expand=295)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_and_pd (k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let and = _mm512_and_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , and , f64x8 :: ZERO)) } }
}

macro_rules! _mm_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_and_ps&ig_expand=297)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_and_ps (src : __m128 , k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let and = _mm_and_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , and , src . as_f32x4 ())) } }
}

macro_rules! _mm_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_and_ps&ig_expand=298)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_and_ps (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let and = _mm_and_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , and , f32x4 :: ZERO)) } }
}

macro_rules! _mm256_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_and_ps&ig_expand=300)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_and_ps (src : __m256 , k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let and = _mm256_and_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , and , src . as_f32x8 ())) } }
}

macro_rules! _mm256_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_and_ps&ig_expand=301)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_and_ps (k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let and = _mm256_and_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , and , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_and_ps&ig_expand=303)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_and_ps (a : __m512 , b : __m512) -> __m512 { unsafe { transmute (simd_and (transmute :: < _ , u32x16 > (a) , transmute :: < _ , u32x16 > (b) ,)) } }
}

macro_rules! _mm512_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_and_ps&ig_expand=304)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_and_ps (src : __m512 , k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let and = _mm512_and_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , and , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_and_ps_introspect!();
    # [doc = " Compute the bitwise AND of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_and_ps&ig_expand=305)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_and_ps (k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let and = _mm512_and_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , and , f32x16 :: ZERO)) } }
}

macro_rules! _mm_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_andnot_pd&ig_expand=326)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_andnot_pd (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let andnot = _mm_andnot_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , andnot , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_andnot_pd&ig_expand=327)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_andnot_pd (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let andnot = _mm_andnot_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , andnot , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_andnot_pd&ig_expand=329)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_andnot_pd (src : __m256d , k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let andnot = _mm256_andnot_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , andnot , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_andnot_pd&ig_expand=330)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_andnot_pd (k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let andnot = _mm256_andnot_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , andnot , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_andnot_pd&ig_expand=331)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnp))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_andnot_pd (a : __m512d , b : __m512d) -> __m512d { unsafe { _mm512_and_pd (_mm512_xor_pd (a , transmute (_mm512_set1_epi64 (- 1))) , b) } }
}

macro_rules! _mm512_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_andnot_pd&ig_expand=332)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_andnot_pd (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let andnot = _mm512_andnot_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , andnot , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_andnot_pd_introspect!();
    # [doc = " Compute the bitwise NOT of packed double-precision (64-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_andnot_pd&ig_expand=333)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_andnot_pd (k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let andnot = _mm512_andnot_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , andnot , f64x8 :: ZERO)) } }
}

macro_rules! _mm_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_andnot_ps&ig_expand=335)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_andnot_ps (src : __m128 , k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let andnot = _mm_andnot_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , andnot , src . as_f32x4 ())) } }
}

macro_rules! _mm_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_andnot_ps&ig_expand=336)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_andnot_ps (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let andnot = _mm_andnot_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , andnot , f32x4 :: ZERO)) } }
}

macro_rules! _mm256_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_andnot_ps&ig_expand=338)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_andnot_ps (src : __m256 , k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let andnot = _mm256_andnot_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , andnot , src . as_f32x8 ())) } }
}

macro_rules! _mm256_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_andnot_ps&ig_expand=339)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_andnot_ps (k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let andnot = _mm256_andnot_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , andnot , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_andnot_ps&ig_expand=340)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_andnot_ps (a : __m512 , b : __m512) -> __m512 { unsafe { _mm512_and_ps (_mm512_xor_ps (a , transmute (_mm512_set1_epi32 (- 1))) , b) } }
}

macro_rules! _mm512_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_andnot_ps&ig_expand=341)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_andnot_ps (src : __m512 , k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let andnot = _mm512_andnot_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , andnot , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_andnot_ps_introspect!();
    # [doc = " Compute the bitwise NOT of packed single-precision (32-bit) floating point numbers in a and then"] # [doc = " bitwise AND with b and store the results in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_andnot_ps&ig_expand=342)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vandnps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_andnot_ps (k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let andnot = _mm512_andnot_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , andnot , f32x16 :: ZERO)) } }
}

macro_rules! _mm_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_or_pd&ig_expand=4824)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_or_pd (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let or = _mm_or_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , or , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_or_pd&ig_expand=4825)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_or_pd (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let or = _mm_or_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , or , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_or_pd&ig_expand=4827)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_or_pd (src : __m256d , k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let or = _mm256_or_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , or , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_or_pd&ig_expand=4828)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_or_pd (k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let or = _mm256_or_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , or , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_or_pd&ig_expand=4829)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorp))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_or_pd (a : __m512d , b : __m512d) -> __m512d { unsafe { transmute (simd_or (transmute :: < _ , u64x8 > (a) , transmute :: < _ , u64x8 > (b))) } }
}

macro_rules! _mm512_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_or_pd&ig_expand=4830)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_or_pd (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let or = _mm512_or_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , or , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_or_pd_introspect!();
    # [doc = " Compute the bitwise OR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_or_pd&ig_expand=4831)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_or_pd (k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let or = _mm512_or_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , or , f64x8 :: ZERO)) } }
}

macro_rules! _mm_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_or_ps&ig_expand=4833)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_or_ps (src : __m128 , k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let or = _mm_or_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , or , src . as_f32x4 ())) } }
}

macro_rules! _mm_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_or_ps&ig_expand=4834)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_or_ps (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let or = _mm_or_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , or , f32x4 :: ZERO)) } }
}

macro_rules! _mm256_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_or_ps&ig_expand=4836)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_or_ps (src : __m256 , k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let or = _mm256_or_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , or , src . as_f32x8 ())) } }
}

macro_rules! _mm256_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_or_ps&ig_expand=4837)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_or_ps (k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let or = _mm256_or_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , or , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_or_ps&ig_expand=4838)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_or_ps (a : __m512 , b : __m512) -> __m512 { unsafe { transmute (simd_or (transmute :: < _ , u32x16 > (a) , transmute :: < _ , u32x16 > (b) ,)) } }
}

macro_rules! _mm512_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_or_ps&ig_expand=4839)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_or_ps (src : __m512 , k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let or = _mm512_or_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , or , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_or_ps_introspect!();
    # [doc = " Compute the bitwise OR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_or_ps&ig_expand=4840)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_or_ps (k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let or = _mm512_or_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , or , f32x16 :: ZERO)) } }
}

macro_rules! _mm_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_xor_pd&ig_expand=7094)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_xor_pd (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let xor = _mm_xor_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , xor , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_xor_pd&ig_expand=7095)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_xor_pd (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { unsafe { let xor = _mm_xor_pd (a , b) . as_f64x2 () ; transmute (simd_select_bitmask (k , xor , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_xor_pd&ig_expand=7097)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_xor_pd (src : __m256d , k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let xor = _mm256_xor_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , xor , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_xor_pd&ig_expand=7098)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_xor_pd (k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { unsafe { let xor = _mm256_xor_pd (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , xor , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_xor_pd&ig_expand=7102)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorp))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_xor_pd (a : __m512d , b : __m512d) -> __m512d { unsafe { transmute (simd_xor (transmute :: < _ , u64x8 > (a) , transmute :: < _ , u64x8 > (b))) } }
}

macro_rules! _mm512_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_xor_pd&ig_expand=7100)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_xor_pd (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let xor = _mm512_xor_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , xor , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_xor_pd_introspect!();
    # [doc = " Compute the bitwise XOR of packed double-precision (64-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_xor_pd&ig_expand=7101)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorpd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_xor_pd (k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { unsafe { let xor = _mm512_xor_pd (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , xor , f64x8 :: ZERO)) } }
}

macro_rules! _mm_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_xor_ps&ig_expand=7103)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_xor_ps (src : __m128 , k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let xor = _mm_xor_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , xor , src . as_f32x4 ())) } }
}

macro_rules! _mm_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_xor_ps&ig_expand=7104)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_xor_ps (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { unsafe { let xor = _mm_xor_ps (a , b) . as_f32x4 () ; transmute (simd_select_bitmask (k , xor , f32x4 :: ZERO)) } }
}

macro_rules! _mm256_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_xor_ps&ig_expand=7106)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_xor_ps (src : __m256 , k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let xor = _mm256_xor_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , xor , src . as_f32x8 ())) } }
}

macro_rules! _mm256_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_xor_ps&ig_expand=7107)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_xor_ps (k : __mmask8 , a : __m256 , b : __m256) -> __m256 { unsafe { let xor = _mm256_xor_ps (a , b) . as_f32x8 () ; transmute (simd_select_bitmask (k , xor , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_xor_ps&ig_expand=7111)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_xor_ps (a : __m512 , b : __m512) -> __m512 { unsafe { transmute (simd_xor (transmute :: < _ , u32x16 > (a) , transmute :: < _ , u32x16 > (b) ,)) } }
}

macro_rules! _mm512_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_xor_ps&ig_expand=7109)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_xor_ps (src : __m512 , k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let xor = _mm512_xor_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , xor , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_xor_ps_introspect!();
    # [doc = " Compute the bitwise XOR of packed single-precision (32-bit) floating point numbers in a and b and"] # [doc = " store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_xor_ps&ig_expand=7110)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vxorps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_xor_ps (k : __mmask16 , a : __m512 , b : __m512) -> __m512 { unsafe { let xor = _mm512_xor_ps (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , xor , f32x16 :: ZERO)) } }
}

macro_rules! _mm256_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcast_f32x2&ig_expand=509)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_broadcast_f32x2 (a : __m128) -> __m256 { unsafe { let b : f32x8 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm256_mask_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_broadcast_f32x2&ig_expand=510)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcastf32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_broadcast_f32x2 (src : __m256 , k : __mmask8 , a : __m128) -> __m256 { unsafe { let b = _mm256_broadcast_f32x2 (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , src . as_f32x8 ())) } }
}

macro_rules! _mm256_maskz_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_broadcast_f32x2&ig_expand=511)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcastf32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_broadcast_f32x2 (k : __mmask8 , a : __m128) -> __m256 { unsafe { let b = _mm256_broadcast_f32x2 (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_f32x2&ig_expand=512)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_f32x2 (a : __m128) -> __m512 { unsafe { let b : f32x16 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_f32x2&ig_expand=513)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vbroadcastf32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_f32x2 (src : __m512 , k : __mmask16 , a : __m128) -> __m512 { unsafe { let b = _mm512_broadcast_f32x2 (a) . as_f32x16 () ; transmute (simd_select_bitmask (k , b , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_f32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_f32x2&ig_expand=514)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vbroadcastf32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_f32x2 (k : __mmask16 , a : __m128) -> __m512 { unsafe { let b = _mm512_broadcast_f32x2 (a) . as_f32x16 () ; transmute (simd_select_bitmask (k , b , f32x16 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_f32x8_introspect!();
    # [doc = " Broadcasts the 8 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_f32x8&ig_expand=521)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_f32x8 (a : __m256) -> __m512 { unsafe { let b : f32x16 = simd_shuffle ! (a , a , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_f32x8_introspect!();
    # [doc = " Broadcasts the 8 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_f32x8&ig_expand=522)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_f32x8 (src : __m512 , k : __mmask16 , a : __m256) -> __m512 { unsafe { let b = _mm512_broadcast_f32x8 (a) . as_f32x16 () ; transmute (simd_select_bitmask (k , b , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_f32x8_introspect!();
    # [doc = " Broadcasts the 8 packed single-precision (32-bit) floating-point elements from a to all"] # [doc = " elements of dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_f32x8&ig_expand=523)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_f32x8 (k : __mmask16 , a : __m256) -> __m512 { unsafe { let b = _mm512_broadcast_f32x8 (a) . as_f32x16 () ; transmute (simd_select_bitmask (k , b , f32x16 :: ZERO)) } }
}

macro_rules! _mm256_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcast_f64x2&ig_expand=524)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_broadcast_f64x2 (a : __m128d) -> __m256d { unsafe { let b : f64x4 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm256_mask_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_broadcast_f64x2&ig_expand=525)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_broadcast_f64x2 (src : __m256d , k : __mmask8 , a : __m128d) -> __m256d { unsafe { let b = _mm256_broadcast_f64x2 (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_broadcast_f64x2&ig_expand=526)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_broadcast_f64x2 (k : __mmask8 , a : __m128d) -> __m256d { unsafe { let b = _mm256_broadcast_f64x2 (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_f64x2&ig_expand=527)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_f64x2 (a : __m128d) -> __m512d { unsafe { let b : f64x8 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_f64x2&ig_expand=528)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_f64x2 (src : __m512d , k : __mmask8 , a : __m128d) -> __m512d { unsafe { let b = _mm512_broadcast_f64x2 (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_f64x2_introspect!();
    # [doc = " Broadcasts the 2 packed double-precision (64-bit) floating-point elements from a to all"] # [doc = " elements of dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_f64x2&ig_expand=529)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_f64x2 (k : __mmask8 , a : __m128d) -> __m512d { unsafe { let b = _mm512_broadcast_f64x2 (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , f64x8 :: ZERO)) } }
}

macro_rules! _mm_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_broadcast_i32x2&ig_expand=533)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_broadcast_i32x2 (a : __m128i) -> __m128i { unsafe { let a = a . as_i32x4 () ; let b : i32x4 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_broadcast_i32x2&ig_expand=534)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_broadcast_i32x2 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { let b = _mm_broadcast_i32x2 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , b , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_broadcast_i32x2&ig_expand=535)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_broadcast_i32x2 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { let b = _mm_broadcast_i32x2 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , b , i32x4 :: ZERO)) } }
}

macro_rules! _mm256_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcast_i32x2&ig_expand=536)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_broadcast_i32x2 (a : __m128i) -> __m256i { unsafe { let a = a . as_i32x4 () ; let b : i32x8 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm256_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_broadcast_i32x2&ig_expand=537)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_broadcast_i32x2 (src : __m256i , k : __mmask8 , a : __m128i) -> __m256i { unsafe { let b = _mm256_broadcast_i32x2 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , b , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_broadcast_i32x2&ig_expand=538)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_broadcast_i32x2 (k : __mmask8 , a : __m128i) -> __m256i { unsafe { let b = _mm256_broadcast_i32x2 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , b , i32x8 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_i32x2&ig_expand=539)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_i32x2 (a : __m128i) -> __m512i { unsafe { let a = a . as_i32x4 () ; let b : i32x16 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_i32x2&ig_expand=540)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_i32x2 (src : __m512i , k : __mmask16 , a : __m128i) -> __m512i { unsafe { let b = _mm512_broadcast_i32x2 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , b , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_i32x2_introspect!();
    # [doc = " Broadcasts the lower 2 packed 32-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_i32x2&ig_expand=541)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vbroadcasti32x2))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_i32x2 (k : __mmask16 , a : __m128i) -> __m512i { unsafe { let b = _mm512_broadcast_i32x2 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , b , i32x16 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_i32x8_introspect!();
    # [doc = " Broadcasts the 8 packed 32-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_i32x8&ig_expand=548)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_i32x8 (a : __m256i) -> __m512i { unsafe { let a = a . as_i32x8 () ; let b : i32x16 = simd_shuffle ! (a , a , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_i32x8_introspect!();
    # [doc = " Broadcasts the 8 packed 32-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_i32x8&ig_expand=549)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_i32x8 (src : __m512i , k : __mmask16 , a : __m256i) -> __m512i { unsafe { let b = _mm512_broadcast_i32x8 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , b , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_i32x8_introspect!();
    # [doc = " Broadcasts the 8 packed 32-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_i32x8&ig_expand=550)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_i32x8 (k : __mmask16 , a : __m256i) -> __m512i { unsafe { let b = _mm512_broadcast_i32x8 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , b , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcast_i64x2&ig_expand=551)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_broadcast_i64x2 (a : __m128i) -> __m256i { unsafe { let a = a . as_i64x2 () ; let b : i64x4 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm256_mask_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_broadcast_i64x2&ig_expand=552)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_broadcast_i64x2 (src : __m256i , k : __mmask8 , a : __m128i) -> __m256i { unsafe { let b = _mm256_broadcast_i64x2 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , b , src . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_broadcast_i64x2&ig_expand=553)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_broadcast_i64x2 (k : __mmask8 , a : __m128i) -> __m256i { unsafe { let b = _mm256_broadcast_i64x2 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , b , i64x4 :: ZERO)) } }
}

macro_rules! _mm512_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcast_i64x2&ig_expand=554)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_broadcast_i64x2 (a : __m128i) -> __m512i { unsafe { let a = a . as_i64x2 () ; let b : i64x8 = simd_shuffle ! (a , a , [0 , 1 , 0 , 1 , 0 , 1 , 0 , 1]) ; transmute (b) } }
}

macro_rules! _mm512_mask_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_broadcast_i64x2&ig_expand=555)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_broadcast_i64x2 (src : __m512i , k : __mmask8 , a : __m128i) -> __m512i { unsafe { let b = _mm512_broadcast_i64x2 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , b , src . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_broadcast_i64x2_introspect!();
    # [doc = " Broadcasts the 2 packed 64-bit integers from a to all elements of dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_broadcast_i64x2&ig_expand=556)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_broadcast_i64x2 (k : __mmask8 , a : __m128i) -> __m512i { unsafe { let b = _mm512_broadcast_i64x2 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , b , i64x8 :: ZERO)) } }
}

macro_rules! _mm512_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_extractf32x8_ps_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed single-precision (32-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_extractf32x8_ps&ig_expand=2946)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_extractf32x8_ps < const IMM8 : i32 > (a : __m512) -> __m256 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; match IMM8 & 1 { 0 => simd_shuffle ! (a , a , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) , _ => simd_shuffle ! (a , a , [8 , 9 , 10 , 11 , 12 , 13 , 14 , 15]) , } } }
}

macro_rules! _mm512_mask_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_extractf32x8_ps_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed single-precision (32-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using writemask k (elements are copied from src"] # [doc = " if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_extractf32x8_ps&ig_expand=2947)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextractf32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_extractf32x8_ps < const IMM8 : i32 > (src : __m256 , k : __mmask8 , a : __m512) -> __m256 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm512_extractf32x8_ps :: < IMM8 > (a) ; transmute (simd_select_bitmask (k , b . as_f32x8 () , src . as_f32x8 ())) } }
}

macro_rules! _mm512_maskz_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_extractf32x8_ps_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed single-precision (32-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_extractf32x8_ps&ig_expand=2948)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextractf32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_extractf32x8_ps < const IMM8 : i32 > (k : __mmask8 , a : __m512) -> __m256 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm512_extractf32x8_ps :: < IMM8 > (a) ; transmute (simd_select_bitmask (k , b . as_f32x8 () , f32x8 :: ZERO)) } }
}

macro_rules! _mm256_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_extractf64x2_pd&ig_expand=2949)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_extractf64x2_pd < const IMM8 : i32 > (a : __m256d) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; match IMM8 & 1 { 0 => simd_shuffle ! (a , a , [0 , 1]) , _ => simd_shuffle ! (a , a , [2 , 3]) , } } }
}

macro_rules! _mm256_mask_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using writemask k (elements are copied from src"] # [doc = " if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_extractf64x2_pd&ig_expand=2950)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vextractf64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_extractf64x2_pd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m256d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm256_extractf64x2_pd :: < IMM8 > (a) ; transmute (simd_select_bitmask (k , b . as_f64x2 () , src . as_f64x2 ())) } }
}

macro_rules! _mm256_maskz_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_extractf64x2_pd&ig_expand=2951)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vextractf64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_extractf64x2_pd < const IMM8 : i32 > (k : __mmask8 , a : __m256d) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm256_extractf64x2_pd :: < IMM8 > (a) ; transmute (simd_select_bitmask (k , b . as_f64x2 () , f64x2 :: ZERO)) } }
}

macro_rules! _mm512_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_extractf64x2_pd&ig_expand=2952)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_extractf64x2_pd < const IMM8 : i32 > (a : __m512d) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; match IMM8 & 3 { 0 => simd_shuffle ! (a , a , [0 , 1]) , 1 => simd_shuffle ! (a , a , [2 , 3]) , 2 => simd_shuffle ! (a , a , [4 , 5]) , _ => simd_shuffle ! (a , a , [6 , 7]) , } } }
}

macro_rules! _mm512_mask_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using writemask k (elements are copied from src"] # [doc = " if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_extractf64x2_pd&ig_expand=2953)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextractf64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_extractf64x2_pd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m512d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let b = _mm512_extractf64x2_pd :: < IMM8 > (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , src . as_f64x2 ())) } }
}

macro_rules! _mm512_maskz_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_extractf64x2_pd_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed double-precision (64-bit) floating-point elements) from a,"] # [doc = " selected with IMM8, and stores the result in dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_extractf64x2_pd&ig_expand=2954)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextractf64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_extractf64x2_pd < const IMM8 : i32 > (k : __mmask8 , a : __m512d) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let b = _mm512_extractf64x2_pd :: < IMM8 > (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , f64x2 :: ZERO)) } }
}

macro_rules! _mm512_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_extracti32x8_epi32_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed 32-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_extracti32x8_epi32&ig_expand=2965)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_extracti32x8_epi32 < const IMM8 : i32 > (a : __m512i) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let a = a . as_i32x16 () ; let b : i32x8 = match IMM8 & 1 { 0 => simd_shuffle ! (a , a , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) , _ => simd_shuffle ! (a , a , [8 , 9 , 10 , 11 , 12 , 13 , 14 , 15]) , } ; transmute (b) } }
}

macro_rules! _mm512_mask_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_extracti32x8_epi32_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed 32-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_extracti32x8_epi32&ig_expand=2966)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextracti32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_extracti32x8_epi32 < const IMM8 : i32 > (src : __m256i , k : __mmask8 , a : __m512i ,) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm512_extracti32x8_epi32 :: < IMM8 > (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , b , src . as_i32x8 ())) } }
}

macro_rules! _mm512_maskz_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_extracti32x8_epi32_introspect!();
    # [doc = " Extracts 256 bits (composed of 8 packed 32-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_extracti32x8_epi32&ig_expand=2967)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextracti32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_extracti32x8_epi32 < const IMM8 : i32 > (k : __mmask8 , a : __m512i) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm512_extracti32x8_epi32 :: < IMM8 > (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , b , i32x8 :: ZERO)) } }
}

macro_rules! _mm256_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_extracti64x2_epi64&ig_expand=2968)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_extracti64x2_epi64 < const IMM8 : i32 > (a : __m256i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let a = a . as_i64x4 () ; match IMM8 & 1 { 0 => simd_shuffle ! (a , a , [0 , 1]) , _ => simd_shuffle ! (a , a , [2 , 3]) , } } }
}

macro_rules! _mm256_mask_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_extracti64x2_epi64&ig_expand=2969)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vextracti64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_extracti64x2_epi64 < const IMM8 : i32 > (src : __m128i , k : __mmask8 , a : __m256i ,) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm256_extracti64x2_epi64 :: < IMM8 > (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , src . as_i64x2 ())) } }
}

macro_rules! _mm256_maskz_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_extracti64x2_epi64&ig_expand=2970)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vextracti64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_extracti64x2_epi64 < const IMM8 : i32 > (k : __mmask8 , a : __m256i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm256_extracti64x2_epi64 :: < IMM8 > (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , i64x2 :: ZERO)) } }
}

macro_rules! _mm512_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_extracti64x2_epi64&ig_expand=2971)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_extracti64x2_epi64 < const IMM8 : i32 > (a : __m512i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let a = a . as_i64x8 () ; match IMM8 & 3 { 0 => simd_shuffle ! (a , a , [0 , 1]) , 1 => simd_shuffle ! (a , a , [2 , 3]) , 2 => simd_shuffle ! (a , a , [4 , 5]) , _ => simd_shuffle ! (a , a , [6 , 7]) , } } }
}

macro_rules! _mm512_mask_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using writemask k (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_extracti64x2_epi64&ig_expand=2972)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextracti64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_extracti64x2_epi64 < const IMM8 : i32 > (src : __m128i , k : __mmask8 , a : __m512i ,) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let b = _mm512_extracti64x2_epi64 :: < IMM8 > (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , src . as_i64x2 ())) } }
}

macro_rules! _mm512_maskz_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_extracti64x2_epi64_introspect!();
    # [doc = " Extracts 128 bits (composed of 2 packed 64-bit integers) from a, selected with IMM8, and stores"] # [doc = " the result in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_extracti64x2_epi64&ig_expand=2973)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vextracti64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_extracti64x2_epi64 < const IMM8 : i32 > (k : __mmask8 , a : __m512i) -> __m128i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let b = _mm512_extracti64x2_epi64 :: < IMM8 > (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , i64x2 :: ZERO)) } }
}

macro_rules! _mm512_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_insertf32x8_introspect!();
    # [doc = " Copy a to dst, then insert 256 bits (composed of 8 packed single-precision (32-bit) floating-point"] # [doc = " elements) from b into dst at the location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_insertf32x8&ig_expand=3850)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_insertf32x8 < const IMM8 : i32 > (a : __m512 , b : __m256) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm512_castps256_ps512 (b) ; match IMM8 & 1 { 0 => { simd_shuffle ! (a , b , [16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15]) } _ => { simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23]) } } } }
}

macro_rules! _mm512_mask_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_insertf32x8_introspect!();
    # [doc = " Copy a to tmp, then insert 256 bits (composed of 8 packed single-precision (32-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_insertf32x8&ig_expand=3851)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinsertf32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_insertf32x8 < const IMM8 : i32 > (src : __m512 , k : __mmask16 , a : __m512 , b : __m256 ,) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm512_insertf32x8 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_f32x16 () , src . as_f32x16 ())) } }
}

macro_rules! _mm512_maskz_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_insertf32x8_introspect!();
    # [doc = " Copy a to tmp, then insert 256 bits (composed of 8 packed single-precision (32-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_insertf32x8&ig_expand=3852)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinsertf32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_insertf32x8 < const IMM8 : i32 > (k : __mmask16 , a : __m512 , b : __m256) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm512_insertf32x8 :: < IMM8 > (a , b) . as_f32x16 () ; transmute (simd_select_bitmask (k , c , f32x16 :: ZERO)) } }
}

macro_rules! _mm256_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_insertf64x2_introspect!();
    # [doc = " Copy a to dst, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into dst at the location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_insertf64x2&ig_expand=3853)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_insertf64x2 < const IMM8 : i32 > (a : __m256d , b : __m128d) -> __m256d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let b = _mm256_castpd128_pd256 (b) ; match IMM8 & 1 { 0 => simd_shuffle ! (a , b , [4 , 5 , 2 , 3]) , _ => simd_shuffle ! (a , b , [0 , 1 , 4 , 5]) , } } }
}

macro_rules! _mm256_mask_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_insertf64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_insertf64x2&ig_expand=3854)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vinsertf64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_insertf64x2 < const IMM8 : i32 > (src : __m256d , k : __mmask8 , a : __m256d , b : __m128d ,) -> __m256d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm256_insertf64x2 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_f64x4 () , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_insertf64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_insertf64x2&ig_expand=3855)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vinsertf64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_insertf64x2 < const IMM8 : i32 > (k : __mmask8 , a : __m256d , b : __m128d) -> __m256d { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm256_insertf64x2 :: < IMM8 > (a , b) . as_f64x4 () ; transmute (simd_select_bitmask (k , c , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_insertf64x2_introspect!();
    # [doc = " Copy a to dst, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into dst at the location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_insertf64x2&ig_expand=3856)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_insertf64x2 < const IMM8 : i32 > (a : __m512d , b : __m128d) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let b = _mm512_castpd128_pd512 (b) ; match IMM8 & 3 { 0 => simd_shuffle ! (a , b , [8 , 9 , 2 , 3 , 4 , 5 , 6 , 7]) , 1 => simd_shuffle ! (a , b , [0 , 1 , 8 , 9 , 4 , 5 , 6 , 7]) , 2 => simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 8 , 9 , 6 , 7]) , _ => simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 4 , 5 , 8 , 9]) , } } }
}

macro_rules! _mm512_mask_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_insertf64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using writemask k"] # [doc = " (elements are copied from src if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_insertf64x2&ig_expand=3857)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinsertf64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_insertf64x2 < const IMM8 : i32 > (src : __m512d , k : __mmask8 , a : __m512d , b : __m128d ,) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let c = _mm512_insertf64x2 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_f64x8 () , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_insertf64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed double-precision (64-bit) floating-point"] # [doc = " elements) from b into tmp at the location specified by IMM8, and copy tmp to dst using zeromask k"] # [doc = " (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_insertf64x2&ig_expand=3858)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinsertf64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_insertf64x2 < const IMM8 : i32 > (k : __mmask8 , a : __m512d , b : __m128d) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let c = _mm512_insertf64x2 :: < IMM8 > (a , b) . as_f64x8 () ; transmute (simd_select_bitmask (k , c , f64x8 :: ZERO)) } }
}

macro_rules! _mm512_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_inserti32x8_introspect!();
    # [doc = " Copy a to dst, then insert 256 bits (composed of 8 packed 32-bit integers) from b into dst at the"] # [doc = " location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_inserti32x8&ig_expand=3869)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_inserti32x8 < const IMM8 : i32 > (a : __m512i , b : __m256i) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let a = a . as_i32x16 () ; let b = _mm512_castsi256_si512 (b) . as_i32x16 () ; let r : i32x16 = match IMM8 & 1 { 0 => { simd_shuffle ! (a , b , [16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15]) } _ => { simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23]) } } ; transmute (r) } }
}

macro_rules! _mm512_mask_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_inserti32x8_introspect!();
    # [doc = " Copy a to tmp, then insert 256 bits (composed of 8 packed 32-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using writemask k (elements are copied from src if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_inserti32x8&ig_expand=3870)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinserti32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_inserti32x8 < const IMM8 : i32 > (src : __m512i , k : __mmask16 , a : __m512i , b : __m256i ,) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm512_inserti32x8 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_i32x16 () , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_inserti32x8_introspect!();
    # [doc = " Copy a to tmp, then insert 256 bits (composed of 8 packed 32-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_inserti32x8&ig_expand=3871)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinserti32x8 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_inserti32x8 < const IMM8 : i32 > (k : __mmask16 , a : __m512i , b : __m256i) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm512_inserti32x8 :: < IMM8 > (a , b) . as_i32x16 () ; transmute (simd_select_bitmask (k , c , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_inserti64x2_introspect!();
    # [doc = " Copy a to dst, then insert 128 bits (composed of 2 packed 64-bit integers) from b into dst at the"] # [doc = " location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_inserti64x2&ig_expand=3872)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_inserti64x2 < const IMM8 : i32 > (a : __m256i , b : __m128i) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let a = a . as_i64x4 () ; let b = _mm256_castsi128_si256 (b) . as_i64x4 () ; match IMM8 & 1 { 0 => simd_shuffle ! (a , b , [4 , 5 , 2 , 3]) , _ => simd_shuffle ! (a , b , [0 , 1 , 4 , 5]) , } } }
}

macro_rules! _mm256_mask_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_inserti64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed 64-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using writemask k (elements are copied from src if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_inserti64x2&ig_expand=3873)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vinserti64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_inserti64x2 < const IMM8 : i32 > (src : __m256i , k : __mmask8 , a : __m256i , b : __m128i ,) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm256_inserti64x2 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_i64x4 () , src . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_inserti64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed 64-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_inserti64x2&ig_expand=3874)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vinserti64x2 , IMM8 = 1))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_inserti64x2 < const IMM8 : i32 > (k : __mmask8 , a : __m256i , b : __m128i) -> __m256i { unsafe { static_assert_uimm_bits ! (IMM8 , 1) ; let c = _mm256_inserti64x2 :: < IMM8 > (a , b) . as_i64x4 () ; transmute (simd_select_bitmask (k , c , i64x4 :: ZERO)) } }
}

macro_rules! _mm512_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_inserti64x2_introspect!();
    # [doc = " Copy a to dst, then insert 128 bits (composed of 2 packed 64-bit integers) from b into dst at the"] # [doc = " location specified by IMM8."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_inserti64x2&ig_expand=3875)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_inserti64x2 < const IMM8 : i32 > (a : __m512i , b : __m128i) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let a = a . as_i64x8 () ; let b = _mm512_castsi128_si512 (b) . as_i64x8 () ; match IMM8 & 3 { 0 => simd_shuffle ! (a , b , [8 , 9 , 2 , 3 , 4 , 5 , 6 , 7]) , 1 => simd_shuffle ! (a , b , [0 , 1 , 8 , 9 , 4 , 5 , 6 , 7]) , 2 => simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 8 , 9 , 6 , 7]) , _ => simd_shuffle ! (a , b , [0 , 1 , 2 , 3 , 4 , 5 , 8 , 9]) , } } }
}

macro_rules! _mm512_mask_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_inserti64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed 64-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using writemask k (elements are copied from src if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_inserti64x2&ig_expand=3876)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinserti64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_inserti64x2 < const IMM8 : i32 > (src : __m512i , k : __mmask8 , a : __m512i , b : __m128i ,) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let c = _mm512_inserti64x2 :: < IMM8 > (a , b) ; transmute (simd_select_bitmask (k , c . as_i64x8 () , src . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_inserti64x2_introspect!();
    # [doc = " Copy a to tmp, then insert 128 bits (composed of 2 packed 64-bit integers) from b into tmp at the"] # [doc = " location specified by IMM8, and copy tmp to dst using zeromask k (elements are zeroed out if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_inserti64x2&ig_expand=3877)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vinserti64x2 , IMM8 = 3))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_inserti64x2 < const IMM8 : i32 > (k : __mmask8 , a : __m512i , b : __m128i) -> __m512i { unsafe { static_assert_uimm_bits ! (IMM8 , 2) ; let c = _mm512_inserti64x2 :: < IMM8 > (a , b) . as_i64x8 () ; transmute (simd_select_bitmask (k , c , i64x8 :: ZERO)) } }
}

macro_rules! _mm512_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundepi64_pd&ig_expand=1437)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundepi64_pd < const ROUNDING : i32 > (a : __m512i) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtqq2pd_512 (a . as_i64x8 () , ROUNDING)) } }
}

macro_rules! _mm512_mask_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundepi64_pd&ig_expand=1438)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundepi64_pd < const ROUNDING : i32 > (src : __m512d , k : __mmask8 , a : __m512i ,) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepi64_pd :: < ROUNDING > (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundepi64_pd&ig_expand=1439)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundepi64_pd < const ROUNDING : i32 > (k : __mmask8 , a : __m512i) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepi64_pd :: < ROUNDING > (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , f64x8 :: ZERO)) } }
}

macro_rules! _mm_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepi64_pd&ig_expand=1705)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtepi64_pd (a : __m128i) -> __m128d { unsafe { transmute (vcvtqq2pd_128 (a . as_i64x2 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtepi64_pd&ig_expand=1706)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtepi64_pd (src : __m128d , k : __mmask8 , a : __m128i) -> __m128d { unsafe { let b = _mm_cvtepi64_pd (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtepi64_pd&ig_expand=1707)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtepi64_pd (k : __mmask8 , a : __m128i) -> __m128d { unsafe { let b = _mm_cvtepi64_pd (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtepi64_pd&ig_expand=1708)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtepi64_pd (a : __m256i) -> __m256d { unsafe { transmute (vcvtqq2pd_256 (a . as_i64x4 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm256_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtepi64_pd&ig_expand=1709)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtepi64_pd (src : __m256d , k : __mmask8 , a : __m256i) -> __m256d { unsafe { let b = _mm256_cvtepi64_pd (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtepi64_pd&ig_expand=1710)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtepi64_pd (k : __mmask8 , a : __m256i) -> __m256d { unsafe { let b = _mm256_cvtepi64_pd (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtepi64_pd&ig_expand=1711)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtepi64_pd (a : __m512i) -> __m512d { unsafe { transmute (vcvtqq2pd_512 (a . as_i64x8 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm512_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtepi64_pd&ig_expand=1712)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtepi64_pd (src : __m512d , k : __mmask8 , a : __m512i) -> __m512d { unsafe { let b = _mm512_cvtepi64_pd (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtepi64_pd_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtepi64_pd&ig_expand=1713)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtepi64_pd (k : __mmask8 , a : __m512i) -> __m512d { unsafe { let b = _mm512_cvtepi64_pd (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , f64x8 :: ZERO)) } }
}

macro_rules! _mm512_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundepi64_ps&ig_expand=1443)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundepi64_ps < const ROUNDING : i32 > (a : __m512i) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtqq2ps_512 (a . as_i64x8 () , ROUNDING)) } }
}

macro_rules! _mm512_mask_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundepi64_ps&ig_expand=1444)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundepi64_ps < const ROUNDING : i32 > (src : __m256 , k : __mmask8 , a : __m512i ,) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepi64_ps :: < ROUNDING > (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , src . as_f32x8 ())) } }
}

macro_rules! _mm512_maskz_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundepi64_ps&ig_expand=1445)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundepi64_ps < const ROUNDING : i32 > (k : __mmask8 , a : __m512i) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepi64_ps :: < ROUNDING > (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , f32x8 :: ZERO)) } }
}

macro_rules! _mm_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepi64_ps&ig_expand=1723)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtepi64_ps (a : __m128i) -> __m128 { _mm_mask_cvtepi64_ps (_mm_undefined_ps () , 0xff , a) }
}

macro_rules! _mm_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtepi64_ps&ig_expand=1724)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtepi64_ps (src : __m128 , k : __mmask8 , a : __m128i) -> __m128 { unsafe { transmute (vcvtqq2ps_128 (a . as_i64x2 () , src . as_f32x4 () , k)) } }
}

macro_rules! _mm_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtepi64_ps&ig_expand=1725)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtepi64_ps (k : __mmask8 , a : __m128i) -> __m128 { _mm_mask_cvtepi64_ps (_mm_setzero_ps () , k , a) }
}

macro_rules! _mm256_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtepi64_ps&ig_expand=1726)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtepi64_ps (a : __m256i) -> __m128 { unsafe { transmute (vcvtqq2ps_256 (a . as_i64x4 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm256_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtepi64_ps&ig_expand=1727)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtepi64_ps (src : __m128 , k : __mmask8 , a : __m256i) -> __m128 { unsafe { let b = _mm256_cvtepi64_ps (a) . as_f32x4 () ; transmute (simd_select_bitmask (k , b , src . as_f32x4 ())) } }
}

macro_rules! _mm256_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtepi64_ps&ig_expand=1728)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtepi64_ps (k : __mmask8 , a : __m256i) -> __m128 { unsafe { let b = _mm256_cvtepi64_ps (a) . as_f32x4 () ; transmute (simd_select_bitmask (k , b , f32x4 :: ZERO)) } }
}

macro_rules! _mm512_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtepi64_ps&ig_expand=1729)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtepi64_ps (a : __m512i) -> __m256 { unsafe { transmute (vcvtqq2ps_512 (a . as_i64x8 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm512_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtepi64_ps&ig_expand=1730)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtepi64_ps (src : __m256 , k : __mmask8 , a : __m512i) -> __m256 { unsafe { let b = _mm512_cvtepi64_ps (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , src . as_f32x8 ())) } }
}

macro_rules! _mm512_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtepi64_ps_introspect!();
    # [doc = " Convert packed signed 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtepi64_ps&ig_expand=1731)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtepi64_ps (k : __mmask8 , a : __m512i) -> __m256 { unsafe { let b = _mm512_cvtepi64_ps (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundepu64_pd&ig_expand=1455)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundepu64_pd < const ROUNDING : i32 > (a : __m512i) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtuqq2pd_512 (a . as_u64x8 () , ROUNDING)) } }
}

macro_rules! _mm512_mask_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundepu64_pd&ig_expand=1456)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundepu64_pd < const ROUNDING : i32 > (src : __m512d , k : __mmask8 , a : __m512i ,) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepu64_pd :: < ROUNDING > (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundepu64_pd&ig_expand=1457)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundepu64_pd < const ROUNDING : i32 > (k : __mmask8 , a : __m512i) -> __m512d { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepu64_pd :: < ROUNDING > (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , f64x8 :: ZERO)) } }
}

macro_rules! _mm_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepu64_pd&ig_expand=1827)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtepu64_pd (a : __m128i) -> __m128d { unsafe { transmute (vcvtuqq2pd_128 (a . as_u64x2 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtepu64_pd&ig_expand=1828)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtepu64_pd (src : __m128d , k : __mmask8 , a : __m128i) -> __m128d { unsafe { let b = _mm_cvtepu64_pd (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , src . as_f64x2 ())) } }
}

macro_rules! _mm_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtepu64_pd&ig_expand=1829)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtepu64_pd (k : __mmask8 , a : __m128i) -> __m128d { unsafe { let b = _mm_cvtepu64_pd (a) . as_f64x2 () ; transmute (simd_select_bitmask (k , b , f64x2 :: ZERO)) } }
}

macro_rules! _mm256_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtepu64_pd&ig_expand=1830)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtepu64_pd (a : __m256i) -> __m256d { unsafe { transmute (vcvtuqq2pd_256 (a . as_u64x4 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm256_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtepu64_pd&ig_expand=1831)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtepu64_pd (src : __m256d , k : __mmask8 , a : __m256i) -> __m256d { unsafe { let b = _mm256_cvtepu64_pd (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , src . as_f64x4 ())) } }
}

macro_rules! _mm256_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtepu64_pd&ig_expand=1832)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtepu64_pd (k : __mmask8 , a : __m256i) -> __m256d { unsafe { let b = _mm256_cvtepu64_pd (a) . as_f64x4 () ; transmute (simd_select_bitmask (k , b , f64x4 :: ZERO)) } }
}

macro_rules! _mm512_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtepu64_pd&ig_expand=1833)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtepu64_pd (a : __m512i) -> __m512d { unsafe { transmute (vcvtuqq2pd_512 (a . as_u64x8 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm512_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtepu64_pd&ig_expand=1834)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtepu64_pd (src : __m512d , k : __mmask8 , a : __m512i) -> __m512d { unsafe { let b = _mm512_cvtepu64_pd (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , src . as_f64x8 ())) } }
}

macro_rules! _mm512_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtepu64_pd_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed double-precision (64-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtepu64_pd&ig_expand=1835)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2pd))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtepu64_pd (k : __mmask8 , a : __m512i) -> __m512d { unsafe { let b = _mm512_cvtepu64_pd (a) . as_f64x8 () ; transmute (simd_select_bitmask (k , b , f64x8 :: ZERO)) } }
}

macro_rules! _mm512_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundepu64_ps&ig_expand=1461)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundepu64_ps < const ROUNDING : i32 > (a : __m512i) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtuqq2ps_512 (a . as_u64x8 () , ROUNDING)) } }
}

macro_rules! _mm512_mask_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundepu64_ps&ig_expand=1462)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundepu64_ps < const ROUNDING : i32 > (src : __m256 , k : __mmask8 , a : __m512i ,) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepu64_ps :: < ROUNDING > (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , src . as_f32x8 ())) } }
}

macro_rules! _mm512_maskz_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundepu64_ps&ig_expand=1463)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundepu64_ps < const ROUNDING : i32 > (k : __mmask8 , a : __m512i) -> __m256 { unsafe { static_assert_rounding ! (ROUNDING) ; let b = _mm512_cvt_roundepu64_ps :: < ROUNDING > (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , f32x8 :: ZERO)) } }
}

macro_rules! _mm_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtepu64_ps&ig_expand=1845)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtepu64_ps (a : __m128i) -> __m128 { _mm_mask_cvtepu64_ps (_mm_undefined_ps () , 0xff , a) }
}

macro_rules! _mm_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtepu64_ps&ig_expand=1846)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtepu64_ps (src : __m128 , k : __mmask8 , a : __m128i) -> __m128 { unsafe { transmute (vcvtuqq2ps_128 (a . as_u64x2 () , src . as_f32x4 () , k)) } }
}

macro_rules! _mm_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtepu64_ps&ig_expand=1847)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtepu64_ps (k : __mmask8 , a : __m128i) -> __m128 { _mm_mask_cvtepu64_ps (_mm_setzero_ps () , k , a) }
}

macro_rules! _mm256_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtepu64_ps&ig_expand=1848)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtepu64_ps (a : __m256i) -> __m128 { unsafe { transmute (vcvtuqq2ps_256 (a . as_u64x4 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm256_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtepu64_ps&ig_expand=1849)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtepu64_ps (src : __m128 , k : __mmask8 , a : __m256i) -> __m128 { unsafe { let b = _mm256_cvtepu64_ps (a) . as_f32x4 () ; transmute (simd_select_bitmask (k , b , src . as_f32x4 ())) } }
}

macro_rules! _mm256_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtepu64_ps&ig_expand=1850)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtepu64_ps (k : __mmask8 , a : __m256i) -> __m128 { unsafe { let b = _mm256_cvtepu64_ps (a) . as_f32x4 () ; transmute (simd_select_bitmask (k , b , f32x4 :: ZERO)) } }
}

macro_rules! _mm512_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtepu64_ps&ig_expand=1851)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtepu64_ps (a : __m512i) -> __m256 { unsafe { transmute (vcvtuqq2ps_512 (a . as_u64x8 () , _MM_FROUND_CUR_DIRECTION)) } }
}

macro_rules! _mm512_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtepu64_ps&ig_expand=1852)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtepu64_ps (src : __m256 , k : __mmask8 , a : __m512i) -> __m256 { unsafe { let b = _mm512_cvtepu64_ps (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , src . as_f32x8 ())) } }
}

macro_rules! _mm512_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtepu64_ps_introspect!();
    # [doc = " Convert packed unsigned 64-bit integers in a to packed single-precision (32-bit) floating-point elements,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtepu64_ps&ig_expand=1853)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtuqq2ps))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtepu64_ps (k : __mmask8 , a : __m512i) -> __m256 { unsafe { let b = _mm512_cvtepu64_ps (a) . as_f32x8 () ; transmute (simd_select_bitmask (k , b , f32x8 :: ZERO)) } }
}

macro_rules! _mm512_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundpd_epi64&ig_expand=1472)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundpd_epi64 < const ROUNDING : i32 > (a : __m512d) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundpd_epi64 :: < ROUNDING > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundpd_epi64&ig_expand=1473)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundpd_epi64 < const ROUNDING : i32 > (src : __m512i , k : __mmask8 , a : __m512d ,) -> __m512i { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtpd2qq_512 (a . as_f64x8 () , src . as_i64x8 () , k , ROUNDING)) } }
}

macro_rules! _mm512_maskz_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundpd_epi64&ig_expand=1474)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundpd_epi64 < const ROUNDING : i32 > (k : __mmask8 , a : __m512d) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundpd_epi64 :: < ROUNDING > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtpd_epi64&ig_expand=1941)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtpd_epi64 (a : __m128d) -> __m128i { _mm_mask_cvtpd_epi64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtpd_epi64&ig_expand=1942)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtpd_epi64 (src : __m128i , k : __mmask8 , a : __m128d) -> __m128i { unsafe { transmute (vcvtpd2qq_128 (a . as_f64x2 () , src . as_i64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtpd_epi64&ig_expand=1943)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtpd_epi64 (k : __mmask8 , a : __m128d) -> __m128i { _mm_mask_cvtpd_epi64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtpd_epi64&ig_expand=1944)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtpd_epi64 (a : __m256d) -> __m256i { _mm256_mask_cvtpd_epi64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtpd_epi64&ig_expand=1945)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtpd_epi64 (src : __m256i , k : __mmask8 , a : __m256d) -> __m256i { unsafe { transmute (vcvtpd2qq_256 (a . as_f64x4 () , src . as_i64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtpd_epi64&ig_expand=1946)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtpd_epi64 (k : __mmask8 , a : __m256d) -> __m256i { _mm256_mask_cvtpd_epi64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtpd_epi64&ig_expand=1947)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtpd_epi64 (a : __m512d) -> __m512i { _mm512_mask_cvtpd_epi64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtpd_epi64&ig_expand=1948)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtpd_epi64 (src : __m512i , k : __mmask8 , a : __m512d) -> __m512i { unsafe { transmute (vcvtpd2qq_512 (a . as_f64x8 () , src . as_i64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtpd_epi64&ig_expand=1949)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtpd_epi64 (k : __mmask8 , a : __m512d) -> __m512i { _mm512_mask_cvtpd_epi64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundps_epi64&ig_expand=1514)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundps_epi64 < const ROUNDING : i32 > (a : __m256) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundps_epi64 :: < ROUNDING > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundps_epi64&ig_expand=1515)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundps_epi64 < const ROUNDING : i32 > (src : __m512i , k : __mmask8 , a : __m256 ,) -> __m512i { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtps2qq_512 (a . as_f32x8 () , src . as_i64x8 () , k , ROUNDING)) } }
}

macro_rules! _mm512_maskz_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundps_epi64&ig_expand=1516)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundps_epi64 < const ROUNDING : i32 > (k : __mmask8 , a : __m256) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundps_epi64 :: < ROUNDING > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtps_epi64&ig_expand=2075)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtps_epi64 (a : __m128) -> __m128i { _mm_mask_cvtps_epi64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtps_epi64&ig_expand=2076)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtps_epi64 (src : __m128i , k : __mmask8 , a : __m128) -> __m128i { unsafe { transmute (vcvtps2qq_128 (a . as_f32x4 () , src . as_i64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtps_epi64&ig_expand=2077)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtps_epi64 (k : __mmask8 , a : __m128) -> __m128i { _mm_mask_cvtps_epi64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtps_epi64&ig_expand=2078)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtps_epi64 (a : __m128) -> __m256i { _mm256_mask_cvtps_epi64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtps_epi64&ig_expand=2079)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtps_epi64 (src : __m256i , k : __mmask8 , a : __m128) -> __m256i { unsafe { transmute (vcvtps2qq_256 (a . as_f32x4 () , src . as_i64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtps_epi64&ig_expand=2080)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtps_epi64 (k : __mmask8 , a : __m128) -> __m256i { _mm256_mask_cvtps_epi64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtps_epi64&ig_expand=2081)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtps_epi64 (a : __m256) -> __m512i { _mm512_mask_cvtps_epi64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtps_epi64&ig_expand=2082)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtps_epi64 (src : __m512i , k : __mmask8 , a : __m256) -> __m512i { unsafe { transmute (vcvtps2qq_512 (a . as_f32x8 () , src . as_i64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtps_epi64&ig_expand=2083)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtps_epi64 (k : __mmask8 , a : __m256) -> __m512i { _mm512_mask_cvtps_epi64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundpd_epu64&ig_expand=1478)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundpd_epu64 < const ROUNDING : i32 > (a : __m512d) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundpd_epu64 :: < ROUNDING > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundpd_epu64&ig_expand=1479)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundpd_epu64 < const ROUNDING : i32 > (src : __m512i , k : __mmask8 , a : __m512d ,) -> __m512i { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtpd2uqq_512 (a . as_f64x8 () , src . as_u64x8 () , k , ROUNDING)) } }
}

macro_rules! _mm512_maskz_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundpd_epu64&ig_expand=1480)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundpd_epu64 < const ROUNDING : i32 > (k : __mmask8 , a : __m512d) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundpd_epu64 :: < ROUNDING > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtpd_epu64&ig_expand=1959)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtpd_epu64 (a : __m128d) -> __m128i { _mm_mask_cvtpd_epu64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtpd_epu64&ig_expand=1960)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtpd_epu64 (src : __m128i , k : __mmask8 , a : __m128d) -> __m128i { unsafe { transmute (vcvtpd2uqq_128 (a . as_f64x2 () , src . as_u64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtpd_epu64&ig_expand=1961)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtpd_epu64 (k : __mmask8 , a : __m128d) -> __m128i { _mm_mask_cvtpd_epu64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtpd_epu64&ig_expand=1962)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtpd_epu64 (a : __m256d) -> __m256i { _mm256_mask_cvtpd_epu64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtpd_epu64&ig_expand=1963)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtpd_epu64 (src : __m256i , k : __mmask8 , a : __m256d) -> __m256i { unsafe { transmute (vcvtpd2uqq_256 (a . as_f64x4 () , src . as_u64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtpd_epu64&ig_expand=1964)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtpd_epu64 (k : __mmask8 , a : __m256d) -> __m256i { _mm256_mask_cvtpd_epu64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtpd_epu64&ig_expand=1965)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtpd_epu64 (a : __m512d) -> __m512i { _mm512_mask_cvtpd_epu64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtpd_epu64&ig_expand=1966)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtpd_epu64 (src : __m512i , k : __mmask8 , a : __m512d) -> __m512i { unsafe { transmute (vcvtpd2uqq_512 (a . as_f64x8 () , src . as_u64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtpd_epu64&ig_expand=1967)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtpd_epu64 (k : __mmask8 , a : __m512d) -> __m512i { _mm512_mask_cvtpd_epu64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst. Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvt_roundps_epu64&ig_expand=1520)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvt_roundps_epu64 < const ROUNDING : i32 > (a : __m256) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundps_epu64 :: < ROUNDING > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set). Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvt_roundps_epu64&ig_expand=1521)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvt_roundps_epu64 < const ROUNDING : i32 > (src : __m512i , k : __mmask8 , a : __m256 ,) -> __m512i { unsafe { static_assert_rounding ! (ROUNDING) ; transmute (vcvtps2uqq_512 (a . as_f32x8 () , src . as_u64x8 () , k , ROUNDING)) } }
}

macro_rules! _mm512_maskz_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = " Rounding is done according to the ROUNDING parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] | [`_MM_FROUND_NO_EXC`] : round to nearest and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] | [`_MM_FROUND_NO_EXC`] : round down and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] | [`_MM_FROUND_NO_EXC`] : round up and suppress exceptions"] # [doc = " * [`_MM_FROUND_TO_ZERO`] | [`_MM_FROUND_NO_EXC`] : truncate and suppress exceptions"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvt_roundps_epu64&ig_expand=1522)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq , ROUNDING = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvt_roundps_epu64 < const ROUNDING : i32 > (k : __mmask8 , a : __m256) -> __m512i { static_assert_rounding ! (ROUNDING) ; _mm512_mask_cvt_roundps_epu64 :: < ROUNDING > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvtps_epu64&ig_expand=2093)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtps_epu64 (a : __m128) -> __m128i { _mm_mask_cvtps_epu64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvtps_epu64&ig_expand=2094)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvtps_epu64 (src : __m128i , k : __mmask8 , a : __m128) -> __m128i { unsafe { transmute (vcvtps2uqq_128 (a . as_f32x4 () , src . as_u64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvtps_epu64&ig_expand=2095)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvtps_epu64 (k : __mmask8 , a : __m128) -> __m128i { _mm_mask_cvtps_epu64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvtps_epu64&ig_expand=2096)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtps_epu64 (a : __m128) -> __m256i { _mm256_mask_cvtps_epu64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvtps_epu64&ig_expand=2097)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvtps_epu64 (src : __m256i , k : __mmask8 , a : __m128) -> __m256i { unsafe { transmute (vcvtps2uqq_256 (a . as_f32x4 () , src . as_u64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvtps_epu64&ig_expand=2098)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvtps_epu64 (k : __mmask8 , a : __m128) -> __m256i { _mm256_mask_cvtps_epu64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtps_epu64&ig_expand=2099)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtps_epu64 (a : __m256) -> __m512i { _mm512_mask_cvtps_epu64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using writemask k (elements are copied from src if the corresponding bit is"] # [doc = " not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtps_epu64&ig_expand=2100)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtps_epu64 (src : __m512i , k : __mmask8 , a : __m256) -> __m512i { unsafe { transmute (vcvtps2uqq_512 (a . as_f32x8 () , src . as_u64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers,"] # [doc = " and store the results in dst using zeromask k (elements are zeroed out if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtps_epu64&ig_expand=2101)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvtps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtps_epu64 (k : __mmask8 , a : __m256) -> __m512i { _mm512_mask_cvtps_epu64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC"] # [doc = " to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtt_roundpd_epi64&ig_expand=2264)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq , SAE = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtt_roundpd_epi64 < const SAE : i32 > (a : __m512d) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundpd_epi64 :: < SAE > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtt_roundpd_epi64&ig_expand=2265)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq , SAE = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtt_roundpd_epi64 < const SAE : i32 > (src : __m512i , k : __mmask8 , a : __m512d ,) -> __m512i { unsafe { static_assert_sae ! (SAE) ; transmute (vcvttpd2qq_512 (a . as_f64x8 () , src . as_i64x8 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtt_roundpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtt_roundpd_epi64&ig_expand=2266)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq , SAE = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtt_roundpd_epi64 < const SAE : i32 > (k : __mmask8 , a : __m512d) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundpd_epi64 :: < SAE > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttpd_epi64&ig_expand=2329)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvttpd_epi64 (a : __m128d) -> __m128i { _mm_mask_cvttpd_epi64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvttpd_epi64&ig_expand=2330)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvttpd_epi64 (src : __m128i , k : __mmask8 , a : __m128d) -> __m128i { unsafe { transmute (vcvttpd2qq_128 (a . as_f64x2 () , src . as_i64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvttpd_epi64&ig_expand=2331)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvttpd_epi64 (k : __mmask8 , a : __m128d) -> __m128i { _mm_mask_cvttpd_epi64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvttpd_epi64&ig_expand=2332)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvttpd_epi64 (a : __m256d) -> __m256i { _mm256_mask_cvttpd_epi64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvttpd_epi64&ig_expand=2333)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvttpd_epi64 (src : __m256i , k : __mmask8 , a : __m256d) -> __m256i { unsafe { transmute (vcvttpd2qq_256 (a . as_f64x4 () , src . as_i64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvttpd_epi64&ig_expand=2334)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvttpd_epi64 (k : __mmask8 , a : __m256d) -> __m256i { _mm256_mask_cvttpd_epi64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvttpd_epi64&ig_expand=2335)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvttpd_epi64 (a : __m512d) -> __m512i { _mm512_mask_cvttpd_epi64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvttpd_epi64&ig_expand=2336)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvttpd_epi64 (src : __m512i , k : __mmask8 , a : __m512d) -> __m512i { unsafe { transmute (vcvttpd2qq_512 (a . as_f64x8 () , src . as_i64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvttpd_epi64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvttpd_epi64&ig_expand=2337)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvttpd_epi64 (k : __mmask8 , a : __m512d) -> __m512i { _mm512_mask_cvttpd_epi64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC"] # [doc = " to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtt_roundps_epi64&ig_expand=2294)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq , SAE = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtt_roundps_epi64 < const SAE : i32 > (a : __m256) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundps_epi64 :: < SAE > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtt_roundps_epi64&ig_expand=2295)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq , SAE = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtt_roundps_epi64 < const SAE : i32 > (src : __m512i , k : __mmask8 , a : __m256 ,) -> __m512i { unsafe { static_assert_sae ! (SAE) ; transmute (vcvttps2qq_512 (a . as_f32x8 () , src . as_i64x8 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtt_roundps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtt_roundps_epi64&ig_expand=2296)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq , SAE = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtt_roundps_epi64 < const SAE : i32 > (k : __mmask8 , a : __m256) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundps_epi64 :: < SAE > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttps_epi64&ig_expand=2420)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvttps_epi64 (a : __m128) -> __m128i { _mm_mask_cvttps_epi64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvttps_epi64&ig_expand=2421)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvttps_epi64 (src : __m128i , k : __mmask8 , a : __m128) -> __m128i { unsafe { transmute (vcvttps2qq_128 (a . as_f32x4 () , src . as_i64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvttps_epi64&ig_expand=2422)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvttps_epi64 (k : __mmask8 , a : __m128) -> __m128i { _mm_mask_cvttps_epi64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvttps_epi64&ig_expand=2423)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvttps_epi64 (a : __m128) -> __m256i { _mm256_mask_cvttps_epi64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvttps_epi64&ig_expand=2424)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvttps_epi64 (src : __m256i , k : __mmask8 , a : __m128) -> __m256i { unsafe { transmute (vcvttps2qq_256 (a . as_f32x4 () , src . as_i64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvttps_epi64&ig_expand=2425)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvttps_epi64 (k : __mmask8 , a : __m128) -> __m256i { _mm256_mask_cvttps_epi64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvttps_epi64&ig_expand=2426)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvttps_epi64 (a : __m256) -> __m512i { _mm512_mask_cvttps_epi64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvttps_epi64&ig_expand=2427)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvttps_epi64 (src : __m512i , k : __mmask8 , a : __m256) -> __m512i { unsafe { transmute (vcvttps2qq_512 (a . as_f32x8 () , src . as_i64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvttps_epi64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed signed 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvttps_epi64&ig_expand=2428)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2qq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvttps_epi64 (k : __mmask8 , a : __m256) -> __m512i { _mm512_mask_cvttps_epi64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC"] # [doc = " to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtt_roundpd_epu64&ig_expand=1965)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq , SAE = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtt_roundpd_epu64 < const SAE : i32 > (a : __m512d) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundpd_epu64 :: < SAE > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtt_roundpd_epu64&ig_expand=1966)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq , SAE = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtt_roundpd_epu64 < const SAE : i32 > (src : __m512i , k : __mmask8 , a : __m512d ,) -> __m512i { unsafe { static_assert_sae ! (SAE) ; transmute (vcvttpd2uqq_512 (a . as_f64x8 () , src . as_u64x8 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtt_roundpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtt_roundpd_epu64&ig_expand=1967)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq , SAE = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtt_roundpd_epu64 < const SAE : i32 > (k : __mmask8 , a : __m512d) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundpd_epu64 :: < SAE > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttpd_epu64&ig_expand=2347)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvttpd_epu64 (a : __m128d) -> __m128i { _mm_mask_cvttpd_epu64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvttpd_epu64&ig_expand=2348)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvttpd_epu64 (src : __m128i , k : __mmask8 , a : __m128d) -> __m128i { unsafe { transmute (vcvttpd2uqq_128 (a . as_f64x2 () , src . as_u64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvttpd_epu64&ig_expand=2349)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvttpd_epu64 (k : __mmask8 , a : __m128d) -> __m128i { _mm_mask_cvttpd_epu64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvttpd_epu64&ig_expand=2350)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvttpd_epu64 (a : __m256d) -> __m256i { _mm256_mask_cvttpd_epu64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the results in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvttpd_epu64&ig_expand=2351)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvttpd_epu64 (src : __m256i , k : __mmask8 , a : __m256d) -> __m256i { unsafe { transmute (vcvttpd2uqq_256 (a . as_f64x4 () , src . as_u64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the results in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvttpd_epu64&ig_expand=2352)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvttpd_epu64 (k : __mmask8 , a : __m256d) -> __m256i { _mm256_mask_cvttpd_epu64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvttpd_epu64&ig_expand=2353)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvttpd_epu64 (a : __m512d) -> __m512i { _mm512_mask_cvttpd_epu64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvttpd_epu64&ig_expand=2354)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvttpd_epu64 (src : __m512i , k : __mmask8 , a : __m512d) -> __m512i { unsafe { transmute (vcvttpd2uqq_512 (a . as_f64x8 () , src . as_u64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvttpd_epu64_introspect!();
    # [doc = " Convert packed double-precision (64-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = ""] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvttpd_epu64&ig_expand=2355)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttpd2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvttpd_epu64 (k : __mmask8 , a : __m512d) -> __m512i { _mm512_mask_cvttpd_epu64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm512_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvtt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC"] # [doc = " to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvtt_roundps_epu64&ig_expand=2300)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq , SAE = 8))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvtt_roundps_epu64 < const SAE : i32 > (a : __m256) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundps_epu64 :: < SAE > (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvtt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvtt_roundps_epu64&ig_expand=2301)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq , SAE = 8))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvtt_roundps_epu64 < const SAE : i32 > (src : __m512i , k : __mmask8 , a : __m256 ,) -> __m512i { unsafe { static_assert_sae ! (SAE) ; transmute (vcvttps2uqq_512 (a . as_f32x8 () , src . as_u64x8 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvtt_roundps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC to the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvtt_roundps_epu64&ig_expand=2302)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq , SAE = 8))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvtt_roundps_epu64 < const SAE : i32 > (k : __mmask8 , a : __m256) -> __m512i { static_assert_sae ! (SAE) ; _mm512_mask_cvtt_roundps_epu64 :: < SAE > (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_cvttps_epu64&ig_expand=2438)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvttps_epu64 (a : __m128) -> __m128i { _mm_mask_cvttps_epu64 (_mm_undefined_si128 () , 0xff , a) }
}

macro_rules! _mm_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_cvttps_epu64&ig_expand=2439)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_cvttps_epu64 (src : __m128i , k : __mmask8 , a : __m128) -> __m128i { unsafe { transmute (vcvttps2uqq_128 (a . as_f32x4 () , src . as_u64x2 () , k)) } }
}

macro_rules! _mm_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_cvttps_epu64&ig_expand=2440)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_cvttps_epu64 (k : __mmask8 , a : __m128) -> __m128i { _mm_mask_cvttps_epu64 (_mm_setzero_si128 () , k , a) }
}

macro_rules! _mm256_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_cvttps_epu64&ig_expand=2441)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvttps_epu64 (a : __m128) -> __m256i { _mm256_mask_cvttps_epu64 (_mm256_undefined_si256 () , 0xff , a) }
}

macro_rules! _mm256_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_cvttps_epu64&ig_expand=2442)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_cvttps_epu64 (src : __m256i , k : __mmask8 , a : __m128) -> __m256i { unsafe { transmute (vcvttps2uqq_256 (a . as_f32x4 () , src . as_u64x4 () , k)) } }
}

macro_rules! _mm256_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_cvttps_epu64&ig_expand=2443)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_cvttps_epu64 (k : __mmask8 , a : __m128) -> __m256i { _mm256_mask_cvttps_epu64 (_mm256_setzero_si256 () , k , a) }
}

macro_rules! _mm512_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_cvttps_epu64&ig_expand=2444)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_cvttps_epu64 (a : __m256) -> __m512i { _mm512_mask_cvttps_epu64 (_mm512_undefined_epi32 () , 0xff , a) }
}

macro_rules! _mm512_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using writemask k (elements are copied from src if the"] # [doc = " corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_cvttps_epu64&ig_expand=2445)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_cvttps_epu64 (src : __m512i , k : __mmask8 , a : __m256) -> __m512i { unsafe { transmute (vcvttps2uqq_512 (a . as_f32x8 () , src . as_u64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_cvttps_epu64_introspect!();
    # [doc = " Convert packed single-precision (32-bit) floating-point elements in a to packed unsigned 64-bit integers"] # [doc = " with truncation, and store the result in dst using zeromask k (elements are zeroed out if the corresponding"] # [doc = " bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_cvttps_epu64&ig_expand=2446)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vcvttps2uqq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_cvttps_epu64 (k : __mmask8 , a : __m256) -> __m512i { _mm512_mask_cvttps_epu64 (_mm512_setzero_si512 () , k , a) }
}

macro_rules! _mm_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst`."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mullo_epi64&ig_expand=4778)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mullo_epi64 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (simd_mul (a . as_i64x2 () , b . as_i64x2 ())) } }
}

macro_rules! _mm_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using writemask `k` (elements are copied from"] # [doc = " `src` if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_mullo_epi64&ig_expand=4776)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_mullo_epi64 (src : __m128i , k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let b = _mm_mullo_epi64 (a , b) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , src . as_i64x2 ())) } }
}

macro_rules! _mm_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using zeromask `k` (elements are zeroed out if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_mullo_epi64&ig_expand=4777)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_mullo_epi64 (k : __mmask8 , a : __m128i , b : __m128i) -> __m128i { unsafe { let b = _mm_mullo_epi64 (a , b) . as_i64x2 () ; transmute (simd_select_bitmask (k , b , i64x2 :: ZERO)) } }
}

macro_rules! _mm256_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst`."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mullo_epi64&ig_expand=4781)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mullo_epi64 (a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (simd_mul (a . as_i64x4 () , b . as_i64x4 ())) } }
}

macro_rules! _mm256_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using writemask `k` (elements are copied from"] # [doc = " `src` if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_mullo_epi64&ig_expand=4779)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_mullo_epi64 (src : __m256i , k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let b = _mm256_mullo_epi64 (a , b) . as_i64x4 () ; transmute (simd_select_bitmask (k , b , src . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using zeromask `k` (elements are zeroed out if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_mullo_epi64&ig_expand=4780)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_mullo_epi64 (k : __mmask8 , a : __m256i , b : __m256i) -> __m256i { unsafe { let b = _mm256_mullo_epi64 (a , b) . as_i64x4 () ; transmute (simd_select_bitmask (k , b , i64x4 :: ZERO)) } }
}

macro_rules! _mm512_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst`."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mullo_epi64&ig_expand=4784)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mullo_epi64 (a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (simd_mul (a . as_i64x8 () , b . as_i64x8 ())) } }
}

macro_rules! _mm512_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using writemask `k` (elements are copied from"] # [doc = " `src` if the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_mullo_epi64&ig_expand=4782)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_mullo_epi64 (src : __m512i , k : __mmask8 , a : __m512i , b : __m512i) -> __m512i { unsafe { let b = _mm512_mullo_epi64 (a , b) . as_i64x8 () ; transmute (simd_select_bitmask (k , b , src . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_mullo_epi64_introspect!();
    # [doc = " Multiply packed 64-bit integers in `a` and `b`, producing intermediate 128-bit integers, and store"] # [doc = " the low 64 bits of the intermediate integers in `dst` using zeromask `k` (elements are zeroed out if"] # [doc = " the corresponding bit is not set)."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_mullo_epi64&ig_expand=4783)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vpmullq))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_mullo_epi64 (k : __mmask8 , a : __m512i , b : __m512i) -> __m512i { unsafe { let b = _mm512_mullo_epi64 (a , b) . as_i64x8 () ; transmute (simd_select_bitmask (k , b , i64x8 :: ZERO)) } }
}

macro_rules! _cvtmask8_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _cvtmask8_u32 in module {}", module_path!());
    };
}

mkfn!{
    _cvtmask8_u32_introspect!();
    # [doc = " Convert 8-bit mask a to a 32-bit integer value and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_cvtmask8_u32&ig_expand=1891)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _cvtmask8_u32 (a : __mmask8) -> u32 { a as u32 }
}

macro_rules! _cvtu32_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _cvtu32_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _cvtu32_mask8_introspect!();
    # [doc = " Convert 32-bit integer value a to an 8-bit mask and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_cvtu32_mask8&ig_expand=2467)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _cvtu32_mask8 (a : u32) -> __mmask8 { a as __mmask8 }
}

macro_rules! _kadd_mask16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kadd_mask16 in module {}", module_path!());
    };
}

mkfn!{
    _kadd_mask16_introspect!();
    # [doc = " Add 16-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kadd_mask16&ig_expand=3903)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kadd_mask16 (a : __mmask16 , b : __mmask16) -> __mmask16 { a + b }
}

macro_rules! _kadd_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kadd_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kadd_mask8_introspect!();
    # [doc = " Add 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kadd_mask8&ig_expand=3906)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kadd_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { a + b }
}

macro_rules! _kand_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kand_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kand_mask8_introspect!();
    # [doc = " Bitwise AND of 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kand_mask8&ig_expand=3911)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kand_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { a & b }
}

macro_rules! _kandn_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kandn_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kandn_mask8_introspect!();
    # [doc = " Bitwise AND NOT of 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kandn_mask8&ig_expand=3916)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kandn_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { _knot_mask8 (a) & b }
}

macro_rules! _knot_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _knot_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _knot_mask8_introspect!();
    # [doc = " Bitwise NOT of 8-bit mask a, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_knot_mask8&ig_expand=3922)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _knot_mask8 (a : __mmask8) -> __mmask8 { a ^ 0b11111111 }
}

macro_rules! _kor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kor_mask8_introspect!();
    # [doc = " Bitwise OR of 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kor_mask8&ig_expand=3927)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kor_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { a | b }
}

macro_rules! _kxnor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kxnor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kxnor_mask8_introspect!();
    # [doc = " Bitwise XNOR of 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kxnor_mask8&ig_expand=3969)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kxnor_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { _knot_mask8 (_kxor_mask8 (a , b)) }
}

macro_rules! _kxor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kxor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kxor_mask8_introspect!();
    # [doc = " Bitwise XOR of 8-bit masks a and b, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kxor_mask8&ig_expand=3974)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kxor_mask8 (a : __mmask8 , b : __mmask8) -> __mmask8 { a ^ b }
}

macro_rules! _kortest_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kortest_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _kortest_mask8_u8_introspect!();
    # [doc = " Compute the bitwise OR of 8-bit masks a and b. If the result is all zeros, store 1 in dst, otherwise"] # [doc = " store 0 in dst. If the result is all ones, store 1 in all_ones, otherwise store 0 in all_ones."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kortest_mask8_u8&ig_expand=3931)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _kortest_mask8_u8 (a : __mmask8 , b : __mmask8 , all_ones : * mut u8) -> u8 { let tmp = _kor_mask8 (a , b) ; * all_ones = (tmp == 0xff) as u8 ; (tmp == 0) as u8 }
}

macro_rules! _kortestc_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kortestc_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _kortestc_mask8_u8_introspect!();
    # [doc = " Compute the bitwise OR of 8-bit masks a and b. If the result is all ones, store 1 in dst, otherwise"] # [doc = " store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kortestc_mask8_u8&ig_expand=3936)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kortestc_mask8_u8 (a : __mmask8 , b : __mmask8) -> u8 { (_kor_mask8 (a , b) == 0xff) as u8 }
}

macro_rules! _kortestz_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kortestz_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _kortestz_mask8_u8_introspect!();
    # [doc = " Compute the bitwise OR of 8-bit masks a and b. If the result is all zeros, store 1 in dst, otherwise"] # [doc = " store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kortestz_mask8_u8&ig_expand=3941)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kortestz_mask8_u8 (a : __mmask8 , b : __mmask8) -> u8 { (_kor_mask8 (a , b) == 0) as u8 }
}

macro_rules! _kshiftli_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kshiftli_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kshiftli_mask8_introspect!();
    # [doc = " Shift 8-bit mask a left by count bits while shifting in zeros, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kshiftli_mask8&ig_expand=3945)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kshiftli_mask8 < const COUNT : u32 > (a : __mmask8) -> __mmask8 { a << COUNT }
}

macro_rules! _kshiftri_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _kshiftri_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _kshiftri_mask8_introspect!();
    # [doc = " Shift 8-bit mask a right by count bits while shifting in zeros, and store the result in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_kshiftri_mask8&ig_expand=3949)"] # [inline] # [target_feature (enable = "avx512dq")] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _kshiftri_mask8 < const COUNT : u32 > (a : __mmask8) -> __mmask8 { a >> COUNT }
}

macro_rules! _ktest_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktest_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktest_mask16_u8_introspect!();
    # [doc = " Compute the bitwise AND of 16-bit masks a and b, and if the result is all zeros, store 1 in dst,"] # [doc = " otherwise store 0 in dst. Compute the bitwise NOT of a and then AND with b, if the result is all"] # [doc = " zeros, store 1 in and_not, otherwise store 0 in and_not."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktest_mask16_u8&ig_expand=3950)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _ktest_mask16_u8 (a : __mmask16 , b : __mmask16 , and_not : * mut u8) -> u8 { * and_not = (_kandn_mask16 (a , b) == 0) as u8 ; (_kand_mask16 (a , b) == 0) as u8 }
}

macro_rules! _ktest_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktest_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktest_mask8_u8_introspect!();
    # [doc = " Compute the bitwise AND of 8-bit masks a and b, and if the result is all zeros, store 1 in dst,"] # [doc = " otherwise store 0 in dst. Compute the bitwise NOT of a and then AND with b, if the result is all"] # [doc = " zeros, store 1 in and_not, otherwise store 0 in and_not."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktest_mask8_u8&ig_expand=3953)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _ktest_mask8_u8 (a : __mmask8 , b : __mmask8 , and_not : * mut u8) -> u8 { * and_not = (_kandn_mask8 (a , b) == 0) as u8 ; (_kand_mask8 (a , b) == 0) as u8 }
}

macro_rules! _ktestc_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktestc_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktestc_mask16_u8_introspect!();
    # [doc = " Compute the bitwise NOT of 16-bit mask a and then AND with 16-bit mask b, if the result is all"] # [doc = " zeros, store 1 in dst, otherwise store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktestc_mask16_u8&ig_expand=3954)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _ktestc_mask16_u8 (a : __mmask16 , b : __mmask16) -> u8 { (_kandn_mask16 (a , b) == 0) as u8 }
}

macro_rules! _ktestc_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktestc_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktestc_mask8_u8_introspect!();
    # [doc = " Compute the bitwise NOT of 8-bit mask a and then AND with 8-bit mask b, if the result is all"] # [doc = " zeros, store 1 in dst, otherwise store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktestc_mask8_u8&ig_expand=3957)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _ktestc_mask8_u8 (a : __mmask8 , b : __mmask8) -> u8 { (_kandn_mask8 (a , b) == 0) as u8 }
}

macro_rules! _ktestz_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktestz_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktestz_mask16_u8_introspect!();
    # [doc = " Compute the bitwise AND of 16-bit masks a and  b, if the result is all zeros, store 1 in dst, otherwise"] # [doc = " store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktestz_mask16_u8&ig_expand=3958)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _ktestz_mask16_u8 (a : __mmask16 , b : __mmask16) -> u8 { (_kand_mask16 (a , b) == 0) as u8 }
}

macro_rules! _ktestz_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _ktestz_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    _ktestz_mask8_u8_introspect!();
    # [doc = " Compute the bitwise AND of 8-bit masks a and  b, if the result is all zeros, store 1 in dst, otherwise"] # [doc = " store 0 in dst."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_ktestz_mask8_u8&ig_expand=3961)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _ktestz_mask8_u8 (a : __mmask8 , b : __mmask8) -> u8 { (_kand_mask8 (a , b) == 0) as u8 }
}

macro_rules! _load_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _load_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _load_mask8_introspect!();
    # [doc = " Load 8-bit mask from memory"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_load_mask8&ig_expand=3999)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _load_mask8 (mem_addr : * const __mmask8) -> __mmask8 { * mem_addr }
}

macro_rules! _store_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _store_mask8 in module {}", module_path!());
    };
}

mkfn!{
    _store_mask8_introspect!();
    # [doc = " Store 8-bit mask to memory"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_store_mask8&ig_expand=6468)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _store_mask8 (mem_addr : * mut __mmask8 , a : __mmask8) { * mem_addr = a ; }
}

macro_rules! _mm_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_movepi32_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 32-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movepi32_mask&ig_expand=4612)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_movepi32_mask (a : __m128i) -> __mmask8 { let zero = _mm_setzero_si128 () ; _mm_cmplt_epi32_mask (a , zero) }
}

macro_rules! _mm256_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_movepi32_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 32-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_movepi32_mask&ig_expand=4613)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_movepi32_mask (a : __m256i) -> __mmask8 { let zero = _mm256_setzero_si256 () ; _mm256_cmplt_epi32_mask (a , zero) }
}

macro_rules! _mm512_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_movepi32_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 32-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_movepi32_mask&ig_expand=4614)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_movepi32_mask (a : __m512i) -> __mmask16 { let zero = _mm512_setzero_si512 () ; _mm512_cmplt_epi32_mask (a , zero) }
}

macro_rules! _mm_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_movepi64_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 64-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movepi64_mask&ig_expand=4615)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_movepi64_mask (a : __m128i) -> __mmask8 { let zero = _mm_setzero_si128 () ; _mm_cmplt_epi64_mask (a , zero) }
}

macro_rules! _mm256_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_movepi64_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 64-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_movepi64_mask&ig_expand=4616)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_movepi64_mask (a : __m256i) -> __mmask8 { let zero = _mm256_setzero_si256 () ; _mm256_cmplt_epi64_mask (a , zero) }
}

macro_rules! _mm512_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_movepi64_mask_introspect!();
    # [doc = " Set each bit of mask register k based on the most significant bit of the corresponding packed 64-bit"] # [doc = " integer in a."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_movepi64_mask&ig_expand=4617)"] # [inline] # [target_feature (enable = "avx512dq")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_movepi64_mask (a : __m512i) -> __mmask8 { let zero = _mm512_setzero_si512 () ; _mm512_cmplt_epi64_mask (a , zero) }
}

macro_rules! _mm_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_movm_epi32_introspect!();
    # [doc = " Set each packed 32-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movm_epi32&ig_expand=4625)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmovm2d))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_movm_epi32 (k : __mmask8) -> __m128i { let ones = _mm_set1_epi32 (- 1) ; _mm_maskz_mov_epi32 (k , ones) }
}

macro_rules! _mm256_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_movm_epi32_introspect!();
    # [doc = " Set each packed 32-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_movm_epi32&ig_expand=4626)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmovm2d))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_movm_epi32 (k : __mmask8) -> __m256i { let ones = _mm256_set1_epi32 (- 1) ; _mm256_maskz_mov_epi32 (k , ones) }
}

macro_rules! _mm512_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_movm_epi32_introspect!();
    # [doc = " Set each packed 32-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_movm_epi32&ig_expand=4627)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vpmovm2d))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_movm_epi32 (k : __mmask16) -> __m512i { let ones = _mm512_set1_epi32 (- 1) ; _mm512_maskz_mov_epi32 (k , ones) }
}

macro_rules! _mm_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_movm_epi64_introspect!();
    # [doc = " Set each packed 64-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_movm_epi64&ig_expand=4628)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmovm2q))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_movm_epi64 (k : __mmask8) -> __m128i { let ones = _mm_set1_epi64x (- 1) ; _mm_maskz_mov_epi64 (k , ones) }
}

macro_rules! _mm256_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_movm_epi64_introspect!();
    # [doc = " Set each packed 64-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_movm_epi64&ig_expand=4629)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vpmovm2q))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_movm_epi64 (k : __mmask8) -> __m256i { let ones = _mm256_set1_epi64x (- 1) ; _mm256_maskz_mov_epi64 (k , ones) }
}

macro_rules! _mm512_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_movm_epi64_introspect!();
    # [doc = " Set each packed 64-bit integer in dst to all ones or all zeros based on the value of the corresponding"] # [doc = " bit in k."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_movm_epi64&ig_expand=4630)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vpmovm2q))] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_movm_epi64 (k : __mmask8) -> __m512i { let ones = _mm512_set1_epi64 (- 1) ; _mm512_maskz_mov_epi64 (k , ones) }
}

macro_rules! _mm512_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_range_round_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_range_round_pd&ig_expand=5210)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_range_round_pd < const IMM8 : i32 , const SAE : i32 > (a : __m512d , b : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm512_mask_range_round_pd :: < IMM8 , SAE > (_mm512_setzero_pd () , 0xff , a , b) }
}

macro_rules! _mm512_mask_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_range_round_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_range_round_pd&ig_expand=5208)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_range_round_pd < const IMM8 : i32 , const SAE : i32 > (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d ,) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; transmute (vrangepd_512 (a . as_f64x8 () , b . as_f64x8 () , IMM8 , src . as_f64x8 () , k , SAE ,)) } }
}

macro_rules! _mm512_maskz_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_range_round_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_range_round_pd&ig_expand=5209)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_range_round_pd < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m512d , b : __m512d ,) -> __m512d { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm512_mask_range_round_pd :: < IMM8 , SAE > (_mm512_setzero_pd () , k , a , b) }
}

macro_rules! _mm_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_range_pd&ig_expand=5192)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_range_pd < const IMM8 : i32 > (a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_pd :: < IMM8 > (_mm_setzero_pd () , 0xff , a , b) }
}

macro_rules! _mm_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_pd&ig_expand=5190)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_pd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangepd_128 (a . as_f64x2 () , b . as_f64x2 () , IMM8 , src . as_f64x2 () , k ,)) } }
}

macro_rules! _mm_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_pd&ig_expand=5191)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_pd < const IMM8 : i32 > (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_pd :: < IMM8 > (_mm_setzero_pd () , k , a , b) }
}

macro_rules! _mm256_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_range_pd&ig_expand=5195)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_range_pd < const IMM8 : i32 > (a : __m256d , b : __m256d) -> __m256d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm256_mask_range_pd :: < IMM8 > (_mm256_setzero_pd () , 0xff , a , b) }
}

macro_rules! _mm256_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_range_pd&ig_expand=5193)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_range_pd < const IMM8 : i32 > (src : __m256d , k : __mmask8 , a : __m256d , b : __m256d ,) -> __m256d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangepd_256 (a . as_f64x4 () , b . as_f64x4 () , IMM8 , src . as_f64x4 () , k ,)) } }
}

macro_rules! _mm256_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_range_pd&ig_expand=5194)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_range_pd < const IMM8 : i32 > (k : __mmask8 , a : __m256d , b : __m256d) -> __m256d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm256_mask_range_pd :: < IMM8 > (_mm256_setzero_pd () , k , a , b) }
}

macro_rules! _mm512_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_range_pd&ig_expand=5198)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_range_pd < const IMM8 : i32 > (a : __m512d , b : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm512_mask_range_pd :: < IMM8 > (_mm512_setzero_pd () , 0xff , a , b) }
}

macro_rules! _mm512_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_range_pd&ig_expand=5196)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_range_pd < const IMM8 : i32 > (src : __m512d , k : __mmask8 , a : __m512d , b : __m512d ,) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangepd_512 (a . as_f64x8 () , b . as_f64x8 () , IMM8 , src . as_f64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_range_pd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " double-precision (64-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_range_pd&ig_expand=5197)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangepd , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_range_pd < const IMM8 : i32 > (k : __mmask8 , a : __m512d , b : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm512_mask_range_pd :: < IMM8 > (_mm512_setzero_pd () , k , a , b) }
}

macro_rules! _mm512_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_range_round_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_range_round_ps&ig_expand=5213)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_range_round_ps < const IMM8 : i32 , const SAE : i32 > (a : __m512 , b : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm512_mask_range_round_ps :: < IMM8 , SAE > (_mm512_setzero_ps () , 0xffff , a , b) }
}

macro_rules! _mm512_mask_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_range_round_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_range_round_ps&ig_expand=5211)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_range_round_ps < const IMM8 : i32 , const SAE : i32 > (src : __m512 , k : __mmask16 , a : __m512 , b : __m512 ,) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; transmute (vrangeps_512 (a . as_f32x16 () , b . as_f32x16 () , IMM8 , src . as_f32x16 () , k , SAE ,)) } }
}

macro_rules! _mm512_maskz_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_range_round_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_range_round_ps&ig_expand=5212)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_range_round_ps < const IMM8 : i32 , const SAE : i32 > (k : __mmask16 , a : __m512 , b : __m512 ,) -> __m512 { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm512_mask_range_round_ps :: < IMM8 , SAE > (_mm512_setzero_ps () , k , a , b) }
}

macro_rules! _mm_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_range_ps&ig_expand=5201)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_range_ps < const IMM8 : i32 > (a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_ps :: < IMM8 > (_mm_setzero_ps () , 0xff , a , b) }
}

macro_rules! _mm_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_ps&ig_expand=5199)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_ps < const IMM8 : i32 > (src : __m128 , k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangeps_128 (a . as_f32x4 () , b . as_f32x4 () , IMM8 , src . as_f32x4 () , k ,)) } }
}

macro_rules! _mm_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_ps&ig_expand=5200)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_ps < const IMM8 : i32 > (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_ps :: < IMM8 > (_mm_setzero_ps () , k , a , b) }
}

macro_rules! _mm256_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_range_ps&ig_expand=5204)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_range_ps < const IMM8 : i32 > (a : __m256 , b : __m256) -> __m256 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm256_mask_range_ps :: < IMM8 > (_mm256_setzero_ps () , 0xff , a , b) }
}

macro_rules! _mm256_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_range_ps&ig_expand=5202)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_range_ps < const IMM8 : i32 > (src : __m256 , k : __mmask8 , a : __m256 , b : __m256 ,) -> __m256 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangeps_256 (a . as_f32x8 () , b . as_f32x8 () , IMM8 , src . as_f32x8 () , k ,)) } }
}

macro_rules! _mm256_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_range_ps&ig_expand=5203)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_range_ps < const IMM8 : i32 > (k : __mmask8 , a : __m256 , b : __m256) -> __m256 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm256_mask_range_ps :: < IMM8 > (_mm256_setzero_ps () , k , a , b) }
}

macro_rules! _mm512_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_range_ps&ig_expand=5207)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_range_ps < const IMM8 : i32 > (a : __m512 , b : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm512_mask_range_ps :: < IMM8 > (_mm512_setzero_ps () , 0xffff , a , b) }
}

macro_rules! _mm512_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " writemask k (elements are copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_range_ps&ig_expand=5205)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_range_ps < const IMM8 : i32 > (src : __m512 , k : __mmask16 , a : __m512 , b : __m512 ,) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangeps_512 (a . as_f32x16 () , b . as_f32x16 () , IMM8 , src . as_f32x16 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_range_ps_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for packed"] # [doc = " single-precision (32-bit) floating-point elements in a and b, and store the results in dst using"] # [doc = " zeromask k (elements are zeroed out if the corresponding mask bit is not set)."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_range_ps&ig_expand=5206)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangeps , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_range_ps < const IMM8 : i32 > (k : __mmask16 , a : __m512 , b : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm512_mask_range_ps :: < IMM8 > (_mm512_setzero_ps () , k , a , b) }
}

macro_rules! _mm_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_range_round_sd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " double-precision (64-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst, and copy the upper element from a to the upper element of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_range_round_sd&ig_expand=5216)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangesd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_range_round_sd < const IMM8 : i32 , const SAE : i32 > (a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm_mask_range_round_sd :: < IMM8 , SAE > (_mm_setzero_pd () , 0xff , a , b) }
}

macro_rules! _mm_mask_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_round_sd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " double-precision (64-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the"] # [doc = " upper element from a to the upper element of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_round_sd&ig_expand=5214)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangesd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_round_sd < const IMM8 : i32 , const SAE : i32 > (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; transmute (vrangesd (a . as_f64x2 () , b . as_f64x2 () , src . as_f64x2 () , k , IMM8 , SAE ,)) } }
}

macro_rules! _mm_maskz_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_round_sd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " double-precision (64-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper"] # [doc = " element from a to the upper element of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_round_sd&ig_expand=5215)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangesd , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_round_sd < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm_mask_range_round_sd :: < IMM8 , SAE > (_mm_setzero_pd () , k , a , b) }
}

macro_rules! _mm_mask_range_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_sd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " double-precision (64-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the"] # [doc = " upper element from a to the upper element of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_sd&ig_expand=5220)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangesd , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_sd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangesd (a . as_f64x2 () , b . as_f64x2 () , src . as_f64x2 () , k , IMM8 , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm_maskz_range_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_sd_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " double-precision (64-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper"] # [doc = " element from a to the upper element of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_sd&ig_expand=5221)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangesd , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_sd < const IMM8 : i32 > (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_sd :: < IMM8 > (_mm_setzero_pd () , k , a , b) }
}

macro_rules! _mm_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_range_round_ss_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " single-precision (32-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst, and copy the upper 3 packed elements from a to the upper elements of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_range_round_ss&ig_expand=5219)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangess , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_range_round_ss < const IMM8 : i32 , const SAE : i32 > (a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm_mask_range_round_ss :: < IMM8 , SAE > (_mm_setzero_ps () , 0xff , a , b) }
}

macro_rules! _mm_mask_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_round_ss_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " single-precision (32-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the"] # [doc = " upper 3 packed elements from a to the upper elements of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_round_ss&ig_expand=5217)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangess , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_round_ss < const IMM8 : i32 , const SAE : i32 > (src : __m128 , k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; transmute (vrangess (a . as_f32x4 () , b . as_f32x4 () , src . as_f32x4 () , k , IMM8 , SAE ,)) } }
}

macro_rules! _mm_maskz_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_round_ss_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " single-precision (32-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper"] # [doc = " 3 packed elements from a to the upper elements of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_round_ss&ig_expand=5218)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangess , IMM8 = 5 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_round_ss < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { static_assert_uimm_bits ! (IMM8 , 4) ; static_assert_sae ! (SAE) ; _mm_mask_range_round_ss :: < IMM8 , SAE > (_mm_setzero_ps () , k , a , b) }
}

macro_rules! _mm_mask_range_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_range_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_range_ss_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " single-precision (32-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the"] # [doc = " upper 3 packed elements from a to the upper elements of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_range_ss&ig_expand=5222)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangess , IMM8 = 5))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_range_ss < const IMM8 : i32 > (src : __m128 , k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 4) ; transmute (vrangess (a . as_f32x4 () , b . as_f32x4 () , src . as_f32x4 () , k , IMM8 , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm_maskz_range_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_range_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_range_ss_introspect!();
    # [doc = " Calculate the max, min, absolute max, or absolute min (depending on control in imm8) for the lower"] # [doc = " single-precision (32-bit) floating-point element in a and b, store the result in the lower element"] # [doc = " of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper"] # [doc = " 3 packed elements from a to the upper elements of dst."] # [doc = " Lower 2 bits of IMM8 specifies the operation control:"] # [doc = "     00 = min, 01 = max, 10 = absolute min, 11 = absolute max."] # [doc = " Upper 2 bits of IMM8 specifies the sign control:"] # [doc = "     00 = sign from a, 01 = sign from compare result, 10 = clear sign bit, 11 = set sign bit."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_range_ss&ig_expand=5223)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vrangess , IMM8 = 5))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_range_ss < const IMM8 : i32 > (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 4) ; _mm_mask_range_ss :: < IMM8 > (_mm_setzero_ps () , k , a , b) }
}

macro_rules! _mm512_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_reduce_round_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_reduce_round_pd&ig_expand=5438)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (1 , 2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_reduce_round_pd < const IMM8 : i32 , const SAE : i32 > (a : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm512_mask_reduce_round_pd :: < IMM8 , SAE > (_mm512_undefined_pd () , 0xff , a) }
}

macro_rules! _mm512_mask_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_reduce_round_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_reduce_round_pd&ig_expand=5436)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_reduce_round_pd < const IMM8 : i32 , const SAE : i32 > (src : __m512d , k : __mmask8 , a : __m512d ,) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; transmute (vreducepd_512 (a . as_f64x8 () , IMM8 , src . as_f64x8 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_reduce_round_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_reduce_round_pd&ig_expand=5437)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_reduce_round_pd < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m512d ,) -> __m512d { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm512_mask_reduce_round_pd :: < IMM8 , SAE > (_mm512_setzero_pd () , k , a) }
}

macro_rules! _mm_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_pd&ig_expand=5411)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_pd < const IMM8 : i32 > (a : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_pd :: < IMM8 > (_mm_undefined_pd () , 0xff , a) }
}

macro_rules! _mm_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_pd&ig_expand=5409)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_pd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m128d) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreducepd_128 (a . as_f64x2 () , IMM8 , src . as_f64x2 () , k)) } }
}

macro_rules! _mm_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_pd&ig_expand=5410)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_pd < const IMM8 : i32 > (k : __mmask8 , a : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_pd :: < IMM8 > (_mm_setzero_pd () , k , a) }
}

macro_rules! _mm256_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_reduce_pd&ig_expand=5414)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_reduce_pd < const IMM8 : i32 > (a : __m256d) -> __m256d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_reduce_pd :: < IMM8 > (_mm256_undefined_pd () , 0xff , a) }
}

macro_rules! _mm256_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_reduce_pd&ig_expand=5412)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_reduce_pd < const IMM8 : i32 > (src : __m256d , k : __mmask8 , a : __m256d) -> __m256d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreducepd_256 (a . as_f64x4 () , IMM8 , src . as_f64x4 () , k)) } }
}

macro_rules! _mm256_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_reduce_pd&ig_expand=5413)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_reduce_pd < const IMM8 : i32 > (k : __mmask8 , a : __m256d) -> __m256d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_reduce_pd :: < IMM8 > (_mm256_setzero_pd () , k , a) }
}

macro_rules! _mm512_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_reduce_pd&ig_expand=5417)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_reduce_pd < const IMM8 : i32 > (a : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_reduce_pd :: < IMM8 > (_mm512_undefined_pd () , 0xff , a) }
}

macro_rules! _mm512_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_reduce_pd&ig_expand=5415)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_reduce_pd < const IMM8 : i32 > (src : __m512d , k : __mmask8 , a : __m512d) -> __m512d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreducepd_512 (a . as_f64x8 () , IMM8 , src . as_f64x8 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_reduce_pd_introspect!();
    # [doc = " Extract the reduced argument of packed double-precision (64-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_reduce_pd&ig_expand=5416)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducepd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_reduce_pd < const IMM8 : i32 > (k : __mmask8 , a : __m512d) -> __m512d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_reduce_pd :: < IMM8 > (_mm512_setzero_pd () , k , a) }
}

macro_rules! _mm512_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_reduce_round_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_reduce_round_ps&ig_expand=5444)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (1 , 2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_reduce_round_ps < const IMM8 : i32 , const SAE : i32 > (a : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm512_mask_reduce_round_ps :: < IMM8 , SAE > (_mm512_undefined_ps () , 0xffff , a) }
}

macro_rules! _mm512_mask_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_reduce_round_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_reduce_round_ps&ig_expand=5442)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_reduce_round_ps < const IMM8 : i32 , const SAE : i32 > (src : __m512 , k : __mmask16 , a : __m512 ,) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; transmute (vreduceps_512 (a . as_f32x16 () , IMM8 , src . as_f32x16 () , k , SAE)) } }
}

macro_rules! _mm512_maskz_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_reduce_round_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_reduce_round_ps&ig_expand=5443)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_reduce_round_ps < const IMM8 : i32 , const SAE : i32 > (k : __mmask16 , a : __m512 ,) -> __m512 { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm512_mask_reduce_round_ps :: < IMM8 , SAE > (_mm512_setzero_ps () , k , a) }
}

macro_rules! _mm_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_ps&ig_expand=5429)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_ps < const IMM8 : i32 > (a : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_ps :: < IMM8 > (_mm_undefined_ps () , 0xff , a) }
}

macro_rules! _mm_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_ps&ig_expand=5427)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_ps < const IMM8 : i32 > (src : __m128 , k : __mmask8 , a : __m128) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreduceps_128 (a . as_f32x4 () , IMM8 , src . as_f32x4 () , k)) } }
}

macro_rules! _mm_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_ps&ig_expand=5428)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_ps < const IMM8 : i32 > (k : __mmask8 , a : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_ps :: < IMM8 > (_mm_setzero_ps () , k , a) }
}

macro_rules! _mm256_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_reduce_ps&ig_expand=5432)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_reduce_ps < const IMM8 : i32 > (a : __m256) -> __m256 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_reduce_ps :: < IMM8 > (_mm256_undefined_ps () , 0xff , a) }
}

macro_rules! _mm256_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_reduce_ps&ig_expand=5430)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_reduce_ps < const IMM8 : i32 > (src : __m256 , k : __mmask8 , a : __m256) -> __m256 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreduceps_256 (a . as_f32x8 () , IMM8 , src . as_f32x8 () , k)) } }
}

macro_rules! _mm256_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_reduce_ps&ig_expand=5431)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_maskz_reduce_ps < const IMM8 : i32 > (k : __mmask8 , a : __m256) -> __m256 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_reduce_ps :: < IMM8 > (_mm256_setzero_ps () , k , a) }
}

macro_rules! _mm512_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_reduce_ps&ig_expand=5435)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_reduce_ps < const IMM8 : i32 > (a : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_reduce_ps :: < IMM8 > (_mm512_undefined_ps () , 0xffff , a) }
}

macro_rules! _mm512_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using writemask k (elements are"] # [doc = " copied from src to dst if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_reduce_ps&ig_expand=5433)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_reduce_ps < const IMM8 : i32 > (src : __m512 , k : __mmask16 , a : __m512) -> __m512 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreduceps_512 (a . as_f32x16 () , IMM8 , src . as_f32x16 () , k , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm512_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_reduce_ps_introspect!();
    # [doc = " Extract the reduced argument of packed single-precision (32-bit) floating-point elements in a by"] # [doc = " the number of bits specified by imm8, and store the results in dst using zeromask k (elements are"] # [doc = " zeroed out if the corresponding mask bit is not set)."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_reduce_ps&ig_expand=5434)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreduceps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_maskz_reduce_ps < const IMM8 : i32 > (k : __mmask16 , a : __m512) -> __m512 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_reduce_ps :: < IMM8 > (_mm512_setzero_ps () , k , a) }
}

macro_rules! _mm_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_round_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst, and copy"] # [doc = " the upper element from a to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_round_sd&ig_expand=5447)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_round_sd < const IMM8 : i32 , const SAE : i32 > (a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm_mask_reduce_round_sd :: < IMM8 , SAE > (_mm_undefined_pd () , 0xff , a , b) }
}

macro_rules! _mm_mask_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_round_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using writemask"] # [doc = " k (the element is copied from src when mask bit 0 is not set), and copy the upper element from a"] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_round_sd&ig_expand=5445)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_round_sd < const IMM8 : i32 , const SAE : i32 > (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; transmute (vreducesd (a . as_f64x2 () , b . as_f64x2 () , src . as_f64x2 () , k , IMM8 , SAE ,)) } }
}

macro_rules! _mm_maskz_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_round_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using zeromask"] # [doc = " k (the element is zeroed out when mask bit 0 is not set), and copy the upper element from a"] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_round_sd&ig_expand=5446)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_round_sd < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm_mask_reduce_round_sd :: < IMM8 , SAE > (_mm_setzero_pd () , k , a , b) }
}

macro_rules! _mm_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using, and"] # [doc = " copy the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_sd&ig_expand=5456)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_sd < const IMM8 : i32 > (a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_sd :: < IMM8 > (_mm_undefined_pd () , 0xff , a , b) }
}

macro_rules! _mm_mask_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using writemask"] # [doc = " k (the element is copied from src when mask bit 0 is not set), and copy the upper element from a"] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_sd&ig_expand=5454)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_sd < const IMM8 : i32 > (src : __m128d , k : __mmask8 , a : __m128d , b : __m128d ,) -> __m128d { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreducesd (a . as_f64x2 () , b . as_f64x2 () , src . as_f64x2 () , k , IMM8 , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm_maskz_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_sd_introspect!();
    # [doc = " Extract the reduced argument of the lower double-precision (64-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using zeromask"] # [doc = " k (the element is zeroed out when mask bit 0 is not set), and copy the upper element from a"] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_sd&ig_expand=5455)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducesd , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_sd < const IMM8 : i32 > (k : __mmask8 , a : __m128d , b : __m128d) -> __m128d { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_sd :: < IMM8 > (_mm_setzero_pd () , k , a , b) }
}

macro_rules! _mm_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_round_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst, and copy"] # [doc = " the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_round_ss&ig_expand=5453)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (2 , 3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_round_ss < const IMM8 : i32 , const SAE : i32 > (a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm_mask_reduce_round_ss :: < IMM8 , SAE > (_mm_undefined_ps () , 0xff , a , b) }
}

macro_rules! _mm_mask_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_round_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using writemask"] # [doc = " k (the element is copied from src when mask bit 0 is not set), and copy the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_round_ss&ig_expand=5451)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (4 , 5)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_round_ss < const IMM8 : i32 , const SAE : i32 > (src : __m128 , k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; transmute (vreducess (a . as_f32x4 () , b . as_f32x4 () , src . as_f32x4 () , k , IMM8 , SAE ,)) } }
}

macro_rules! _mm_maskz_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_round_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using zeromask"] # [doc = " k (the element is zeroed out when mask bit 0 is not set), and copy the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter."] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_round_ss&ig_expand=5452)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0 , SAE = 8))] # [rustc_legacy_const_generics (3 , 4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_round_ss < const IMM8 : i32 , const SAE : i32 > (k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; static_assert_sae ! (SAE) ; _mm_mask_reduce_round_ss :: < IMM8 , SAE > (_mm_setzero_ps () , k , a , b) }
}

macro_rules! _mm_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_reduce_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst, and copy"] # [doc = " the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_reduce_ss&ig_expand=5462)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_reduce_ss < const IMM8 : i32 > (a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_ss :: < IMM8 > (_mm_undefined_ps () , 0xff , a , b) }
}

macro_rules! _mm_mask_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_reduce_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using writemask"] # [doc = " k (the element is copied from src when mask bit 0 is not set), and copy the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_reduce_ss&ig_expand=5460)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0))] # [rustc_legacy_const_generics (4)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_reduce_ss < const IMM8 : i32 > (src : __m128 , k : __mmask8 , a : __m128 , b : __m128 ,) -> __m128 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vreducess (a . as_f32x4 () , b . as_f32x4 () , src . as_f32x4 () , k , IMM8 , _MM_FROUND_CUR_DIRECTION ,)) } }
}

macro_rules! _mm_maskz_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_reduce_ss_introspect!();
    # [doc = " Extract the reduced argument of the lower single-precision (32-bit) floating-point element in b"] # [doc = " by the number of bits specified by imm8, store the result in the lower element of dst using zeromask"] # [doc = " k (the element is zeroed out when mask bit 0 is not set), and copy the upper element from a."] # [doc = " to the upper element of dst."] # [doc = " Rounding is done according to the imm8 parameter, which can be one of:"] # [doc = ""] # [doc = " * [`_MM_FROUND_TO_NEAREST_INT`] : round to nearest"] # [doc = " * [`_MM_FROUND_TO_NEG_INF`] : round down"] # [doc = " * [`_MM_FROUND_TO_POS_INF`] : round up"] # [doc = " * [`_MM_FROUND_TO_ZERO`] : truncate"] # [doc = " * [`_MM_FROUND_CUR_DIRECTION`] : use `MXCSR.RC` - see [`_MM_SET_ROUNDING_MODE`]"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_reduce_ss&ig_expand=5461)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vreducess , IMM8 = 0))] # [rustc_legacy_const_generics (3)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_maskz_reduce_ss < const IMM8 : i32 > (k : __mmask8 , a : __m128 , b : __m128) -> __m128 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_reduce_ss :: < IMM8 > (_mm_setzero_ps () , k , a , b) }
}

macro_rules! _mm_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fpclass_pd_mask&ig_expand=3493)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_fpclass_pd_mask < const IMM8 : i32 > (a : __m128d) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_fpclass_pd_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_fpclass_pd_mask&ig_expand=3494)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_fpclass_pd_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m128d) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclasspd_128 (a . as_f64x2 () , IMM8 , k1)) } }
}

macro_rules! _mm256_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fpclass_pd_mask&ig_expand=3495)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_fpclass_pd_mask < const IMM8 : i32 > (a : __m256d) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_fpclass_pd_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm256_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_fpclass_pd_mask&ig_expand=3496)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_fpclass_pd_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m256d) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclasspd_256 (a . as_f64x4 () , IMM8 , k1)) } }
}

macro_rules! _mm512_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_fpclass_pd_mask&ig_expand=3497)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_fpclass_pd_mask < const IMM8 : i32 > (a : __m512d) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_fpclass_pd_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm512_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_fpclass_pd_mask_introspect!();
    # [doc = " Test packed double-precision (64-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_fpclass_pd_mask&ig_expand=3498)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclasspd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_fpclass_pd_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m512d) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclasspd_512 (a . as_f64x8 () , IMM8 , k1)) } }
}

macro_rules! _mm_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fpclass_ps_mask&ig_expand=3505)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_fpclass_ps_mask < const IMM8 : i32 > (a : __m128) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_fpclass_ps_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_fpclass_ps_mask&ig_expand=3506)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_fpclass_ps_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m128) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclassps_128 (a . as_f32x4 () , IMM8 , k1)) } }
}

macro_rules! _mm256_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_fpclass_ps_mask&ig_expand=3507)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_fpclass_ps_mask < const IMM8 : i32 > (a : __m256) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm256_mask_fpclass_ps_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm256_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_fpclass_ps_mask&ig_expand=3508)"] # [inline] # [target_feature (enable = "avx512dq,avx512vl")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_mask_fpclass_ps_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m256) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclassps_256 (a . as_f32x8 () , IMM8 , k1)) } }
}

macro_rules! _mm512_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_fpclass_ps_mask&ig_expand=3509)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_fpclass_ps_mask < const IMM8 : i32 > (a : __m512) -> __mmask16 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm512_mask_fpclass_ps_mask :: < IMM8 > (0xffff , a) }
}

macro_rules! _mm512_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_fpclass_ps_mask_introspect!();
    # [doc = " Test packed single-precision (32-bit) floating-point elements in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_fpclass_ps_mask&ig_expand=3510)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclassps , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm512_mask_fpclass_ps_mask < const IMM8 : i32 > (k1 : __mmask16 , a : __m512) -> __mmask16 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; transmute (vfpclassps_512 (a . as_f32x16 () , IMM8 , k1)) } }
}

macro_rules! _mm_fpclass_sd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fpclass_sd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_fpclass_sd_mask_introspect!();
    # [doc = " Test the lower double-precision (64-bit) floating-point element in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fpclass_sd_mask&ig_expand=3511)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclasssd , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_fpclass_sd_mask < const IMM8 : i32 > (a : __m128d) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_fpclass_sd_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm_mask_fpclass_sd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_fpclass_sd_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_fpclass_sd_mask_introspect!();
    # [doc = " Test the lower double-precision (64-bit) floating-point element in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_fpclass_sd_mask&ig_expand=3512)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclasssd , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_fpclass_sd_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m128d) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; vfpclasssd (a . as_f64x2 () , IMM8 , k1) } }
}

macro_rules! _mm_fpclass_ss_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_fpclass_ss_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_fpclass_ss_mask_introspect!();
    # [doc = " Test the lower single-precision (32-bit) floating-point element in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_fpclass_ss_mask&ig_expand=3515)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclassss , IMM8 = 0))] # [rustc_legacy_const_generics (1)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_fpclass_ss_mask < const IMM8 : i32 > (a : __m128) -> __mmask8 { static_assert_uimm_bits ! (IMM8 , 8) ; _mm_mask_fpclass_ss_mask :: < IMM8 > (0xff , a) }
}

macro_rules! _mm_mask_fpclass_ss_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_fpclass_ss_mask in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_fpclass_ss_mask_introspect!();
    # [doc = " Test the lower single-precision (32-bit) floating-point element in a for special categories specified"] # [doc = " by imm8, and store the results in mask vector k using zeromask k1 (elements are zeroed out when the"] # [doc = " corresponding mask bit is not set)."] # [doc = " imm can be a combination of:"] # [doc = ""] # [doc = "     - 0x01 // QNaN"] # [doc = "     - 0x02 // Positive Zero"] # [doc = "     - 0x04 // Negative Zero"] # [doc = "     - 0x08 // Positive Infinity"] # [doc = "     - 0x10 // Negative Infinity"] # [doc = "     - 0x20 // Denormal"] # [doc = "     - 0x40 // Negative"] # [doc = "     - 0x80 // SNaN"] # [doc = ""] # [doc = " [Intel's Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_fpclass_ss_mask&ig_expand=3516)"] # [inline] # [target_feature (enable = "avx512dq")] # [cfg_attr (test , assert_instr (vfpclassss , IMM8 = 0))] # [rustc_legacy_const_generics (2)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_mask_fpclass_ss_mask < const IMM8 : i32 > (k1 : __mmask8 , a : __m128) -> __mmask8 { unsafe { static_assert_uimm_bits ! (IMM8 , 8) ; vfpclassss (a . as_f32x4 () , IMM8 , k1) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.avx512.sitofp.round.v2f64.v2i64"] fn vcvtqq2pd_128 (a : i64x2 , rounding : i32) -> f64x2 ; # [link_name = "llvm.x86.avx512.sitofp.round.v4f64.v4i64"] fn vcvtqq2pd_256 (a : i64x4 , rounding : i32) -> f64x4 ; # [link_name = "llvm.x86.avx512.sitofp.round.v8f64.v8i64"] fn vcvtqq2pd_512 (a : i64x8 , rounding : i32) -> f64x8 ; # [link_name = "llvm.x86.avx512.mask.cvtqq2ps.128"] fn vcvtqq2ps_128 (a : i64x2 , src : f32x4 , k : __mmask8) -> f32x4 ; # [link_name = "llvm.x86.avx512.sitofp.round.v4f32.v4i64"] fn vcvtqq2ps_256 (a : i64x4 , rounding : i32) -> f32x4 ; # [link_name = "llvm.x86.avx512.sitofp.round.v8f32.v8i64"] fn vcvtqq2ps_512 (a : i64x8 , rounding : i32) -> f32x8 ; # [link_name = "llvm.x86.avx512.uitofp.round.v2f64.v2i64"] fn vcvtuqq2pd_128 (a : u64x2 , rounding : i32) -> f64x2 ; # [link_name = "llvm.x86.avx512.uitofp.round.v4f64.v4i64"] fn vcvtuqq2pd_256 (a : u64x4 , rounding : i32) -> f64x4 ; # [link_name = "llvm.x86.avx512.uitofp.round.v8f64.v8i64"] fn vcvtuqq2pd_512 (a : u64x8 , rounding : i32) -> f64x8 ; # [link_name = "llvm.x86.avx512.mask.cvtuqq2ps.128"] fn vcvtuqq2ps_128 (a : u64x2 , src : f32x4 , k : __mmask8) -> f32x4 ; # [link_name = "llvm.x86.avx512.uitofp.round.v4f32.v4i64"] fn vcvtuqq2ps_256 (a : u64x4 , rounding : i32) -> f32x4 ; # [link_name = "llvm.x86.avx512.uitofp.round.v8f32.v8i64"] fn vcvtuqq2ps_512 (a : u64x8 , rounding : i32) -> f32x8 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2qq.128"] fn vcvtpd2qq_128 (a : f64x2 , src : i64x2 , k : __mmask8) -> i64x2 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2qq.256"] fn vcvtpd2qq_256 (a : f64x4 , src : i64x4 , k : __mmask8) -> i64x4 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2qq.512"] fn vcvtpd2qq_512 (a : f64x8 , src : i64x8 , k : __mmask8 , rounding : i32) -> i64x8 ; # [link_name = "llvm.x86.avx512.mask.cvtps2qq.128"] fn vcvtps2qq_128 (a : f32x4 , src : i64x2 , k : __mmask8) -> i64x2 ; # [link_name = "llvm.x86.avx512.mask.cvtps2qq.256"] fn vcvtps2qq_256 (a : f32x4 , src : i64x4 , k : __mmask8) -> i64x4 ; # [link_name = "llvm.x86.avx512.mask.cvtps2qq.512"] fn vcvtps2qq_512 (a : f32x8 , src : i64x8 , k : __mmask8 , rounding : i32) -> i64x8 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2uqq.128"] fn vcvtpd2uqq_128 (a : f64x2 , src : u64x2 , k : __mmask8) -> u64x2 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2uqq.256"] fn vcvtpd2uqq_256 (a : f64x4 , src : u64x4 , k : __mmask8) -> u64x4 ; # [link_name = "llvm.x86.avx512.mask.cvtpd2uqq.512"] fn vcvtpd2uqq_512 (a : f64x8 , src : u64x8 , k : __mmask8 , rounding : i32) -> u64x8 ; # [link_name = "llvm.x86.avx512.mask.cvtps2uqq.128"] fn vcvtps2uqq_128 (a : f32x4 , src : u64x2 , k : __mmask8) -> u64x2 ; # [link_name = "llvm.x86.avx512.mask.cvtps2uqq.256"] fn vcvtps2uqq_256 (a : f32x4 , src : u64x4 , k : __mmask8) -> u64x4 ; # [link_name = "llvm.x86.avx512.mask.cvtps2uqq.512"] fn vcvtps2uqq_512 (a : f32x8 , src : u64x8 , k : __mmask8 , rounding : i32) -> u64x8 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2qq.128"] fn vcvttpd2qq_128 (a : f64x2 , src : i64x2 , k : __mmask8) -> i64x2 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2qq.256"] fn vcvttpd2qq_256 (a : f64x4 , src : i64x4 , k : __mmask8) -> i64x4 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2qq.512"] fn vcvttpd2qq_512 (a : f64x8 , src : i64x8 , k : __mmask8 , sae : i32) -> i64x8 ; # [link_name = "llvm.x86.avx512.mask.cvttps2qq.128"] fn vcvttps2qq_128 (a : f32x4 , src : i64x2 , k : __mmask8) -> i64x2 ; # [link_name = "llvm.x86.avx512.mask.cvttps2qq.256"] fn vcvttps2qq_256 (a : f32x4 , src : i64x4 , k : __mmask8) -> i64x4 ; # [link_name = "llvm.x86.avx512.mask.cvttps2qq.512"] fn vcvttps2qq_512 (a : f32x8 , src : i64x8 , k : __mmask8 , sae : i32) -> i64x8 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2uqq.128"] fn vcvttpd2uqq_128 (a : f64x2 , src : u64x2 , k : __mmask8) -> u64x2 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2uqq.256"] fn vcvttpd2uqq_256 (a : f64x4 , src : u64x4 , k : __mmask8) -> u64x4 ; # [link_name = "llvm.x86.avx512.mask.cvttpd2uqq.512"] fn vcvttpd2uqq_512 (a : f64x8 , src : u64x8 , k : __mmask8 , sae : i32) -> u64x8 ; # [link_name = "llvm.x86.avx512.mask.cvttps2uqq.128"] fn vcvttps2uqq_128 (a : f32x4 , src : u64x2 , k : __mmask8) -> u64x2 ; # [link_name = "llvm.x86.avx512.mask.cvttps2uqq.256"] fn vcvttps2uqq_256 (a : f32x4 , src : u64x4 , k : __mmask8) -> u64x4 ; # [link_name = "llvm.x86.avx512.mask.cvttps2uqq.512"] fn vcvttps2uqq_512 (a : f32x8 , src : u64x8 , k : __mmask8 , sae : i32) -> u64x8 ; # [link_name = "llvm.x86.avx512.mask.range.pd.128"] fn vrangepd_128 (a : f64x2 , b : f64x2 , imm8 : i32 , src : f64x2 , k : __mmask8) -> f64x2 ; # [link_name = "llvm.x86.avx512.mask.range.pd.256"] fn vrangepd_256 (a : f64x4 , b : f64x4 , imm8 : i32 , src : f64x4 , k : __mmask8) -> f64x4 ; # [link_name = "llvm.x86.avx512.mask.range.pd.512"] fn vrangepd_512 (a : f64x8 , b : f64x8 , imm8 : i32 , src : f64x8 , k : __mmask8 , sae : i32) -> f64x8 ; # [link_name = "llvm.x86.avx512.mask.range.ps.128"] fn vrangeps_128 (a : f32x4 , b : f32x4 , imm8 : i32 , src : f32x4 , k : __mmask8) -> f32x4 ; # [link_name = "llvm.x86.avx512.mask.range.ps.256"] fn vrangeps_256 (a : f32x8 , b : f32x8 , imm8 : i32 , src : f32x8 , k : __mmask8) -> f32x8 ; # [link_name = "llvm.x86.avx512.mask.range.ps.512"] fn vrangeps_512 (a : f32x16 , b : f32x16 , imm8 : i32 , src : f32x16 , k : __mmask16 , sae : i32) -> f32x16 ; # [link_name = "llvm.x86.avx512.mask.range.sd"] fn vrangesd (a : f64x2 , b : f64x2 , src : f64x2 , k : __mmask8 , imm8 : i32 , sae : i32) -> f64x2 ; # [link_name = "llvm.x86.avx512.mask.range.ss"] fn vrangess (a : f32x4 , b : f32x4 , src : f32x4 , k : __mmask8 , imm8 : i32 , sae : i32) -> f32x4 ; # [link_name = "llvm.x86.avx512.mask.reduce.pd.128"] fn vreducepd_128 (a : f64x2 , imm8 : i32 , src : f64x2 , k : __mmask8) -> f64x2 ; # [link_name = "llvm.x86.avx512.mask.reduce.pd.256"] fn vreducepd_256 (a : f64x4 , imm8 : i32 , src : f64x4 , k : __mmask8) -> f64x4 ; # [link_name = "llvm.x86.avx512.mask.reduce.pd.512"] fn vreducepd_512 (a : f64x8 , imm8 : i32 , src : f64x8 , k : __mmask8 , sae : i32) -> f64x8 ; # [link_name = "llvm.x86.avx512.mask.reduce.ps.128"] fn vreduceps_128 (a : f32x4 , imm8 : i32 , src : f32x4 , k : __mmask8) -> f32x4 ; # [link_name = "llvm.x86.avx512.mask.reduce.ps.256"] fn vreduceps_256 (a : f32x8 , imm8 : i32 , src : f32x8 , k : __mmask8) -> f32x8 ; # [link_name = "llvm.x86.avx512.mask.reduce.ps.512"] fn vreduceps_512 (a : f32x16 , imm8 : i32 , src : f32x16 , k : __mmask16 , sae : i32) -> f32x16 ; # [link_name = "llvm.x86.avx512.mask.reduce.sd"] fn vreducesd (a : f64x2 , b : f64x2 , src : f64x2 , k : __mmask8 , imm8 : i32 , sae : i32) -> f64x2 ; # [link_name = "llvm.x86.avx512.mask.reduce.ss"] fn vreducess (a : f32x4 , b : f32x4 , src : f32x4 , k : __mmask8 , imm8 : i32 , sae : i32) -> f32x4 ; # [link_name = "llvm.x86.avx512.mask.fpclass.pd.128"] fn vfpclasspd_128 (a : f64x2 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.pd.256"] fn vfpclasspd_256 (a : f64x4 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.pd.512"] fn vfpclasspd_512 (a : f64x8 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.ps.128"] fn vfpclassps_128 (a : f32x4 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.ps.256"] fn vfpclassps_256 (a : f32x8 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.ps.512"] fn vfpclassps_512 (a : f32x16 , imm8 : i32 , k : __mmask16) -> __mmask16 ; # [link_name = "llvm.x86.avx512.mask.fpclass.sd"] fn vfpclasssd (a : f64x2 , imm8 : i32 , k : __mmask8) -> __mmask8 ; # [link_name = "llvm.x86.avx512.mask.fpclass.ss"] fn vfpclassss (a : f32x4 , imm8 : i32 , k : __mmask8) -> __mmask8 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use crate :: mem :: transmute ;}
mkitem!{const OPRND1_64 : f64 = unsafe { transmute (0x3333333333333333_u64) } ;}
mkitem!{const OPRND2_64 : f64 = unsafe { transmute (0x5555555555555555_u64) } ;}
mkitem!{const AND_64 : f64 = unsafe { transmute (0x1111111111111111_u64) } ;}
mkitem!{const ANDN_64 : f64 = unsafe { transmute (0x4444444444444444_u64) } ;}
mkitem!{const OR_64 : f64 = unsafe { transmute (0x7777777777777777_u64) } ;}
mkitem!{const XOR_64 : f64 = unsafe { transmute (0x6666666666666666_u64) } ;}
mkitem!{const OPRND1_32 : f32 = unsafe { transmute (0x33333333_u32) } ;}
mkitem!{const OPRND2_32 : f32 = unsafe { transmute (0x55555555_u32) } ;}
mkitem!{const AND_32 : f32 = unsafe { transmute (0x11111111_u32) } ;}
mkitem!{const ANDN_32 : f32 = unsafe { transmute (0x44444444_u32) } ;}
mkitem!{const OR_32 : f32 = unsafe { transmute (0x77777777_u32) } ;}
mkitem!{const XOR_32 : f32 = unsafe { transmute (0x66666666_u32) } ;}

macro_rules! test_mm_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_and_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_and_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let src = _mm_set_pd (1. , 2.) ; let r = _mm_mask_and_pd (src , 0b01 , a , b) ; let e = _mm_set_pd (1. , AND_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_and_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_and_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let r = _mm_maskz_and_pd (0b01 , a , b) ; let e = _mm_set_pd (0.0 , AND_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_and_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_and_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let src = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_mask_and_pd (src , 0b0101 , a , b) ; let e = _mm256_set_pd (1. , AND_64 , 3. , AND_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_and_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_and_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let r = _mm256_maskz_and_pd (0b0101 , a , b) ; let e = _mm256_set_pd (0.0 , AND_64 , 0.0 , AND_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_and_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_and_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_and_pd (a , b) ; let e = _mm512_set1_pd (AND_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_and_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_and_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let src = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_mask_and_pd (src , 0b01010101 , a , b) ; let e = _mm512_set_pd (1. , AND_64 , 3. , AND_64 , 5. , AND_64 , 7. , AND_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_and_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_and_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_and_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_and_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_maskz_and_pd (0b01010101 , a , b) ; let e = _mm512_set_pd (0.0 , AND_64 , 0.0 , AND_64 , 0.0 , AND_64 , 0.0 , AND_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_and_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_and_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let src = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_mask_and_ps (src , 0b0101 , a , b) ; let e = _mm_set_ps (1. , AND_32 , 3. , AND_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_and_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_and_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let r = _mm_maskz_and_ps (0b0101 , a , b) ; let e = _mm_set_ps (0.0 , AND_32 , 0.0 , AND_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_and_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_and_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let src = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm256_mask_and_ps (src , 0b01010101 , a , b) ; let e = _mm256_set_ps (1. , AND_32 , 3. , AND_32 , 5. , AND_32 , 7. , AND_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_and_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_and_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let r = _mm256_maskz_and_ps (0b01010101 , a , b) ; let e = _mm256_set_ps (0.0 , AND_32 , 0.0 , AND_32 , 0.0 , AND_32 , 0.0 , AND_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_and_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_and_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_and_ps (a , b) ; let e = _mm512_set1_ps (AND_32) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_and_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_and_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let src = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_mask_and_ps (src , 0b0101010101010101 , a , b) ; let e = _mm512_set_ps (1. , AND_32 , 3. , AND_32 , 5. , AND_32 , 7. , AND_32 , 9. , AND_32 , 11. , AND_32 , 13. , AND_32 , 15. , AND_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_and_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_and_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_and_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_and_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_maskz_and_ps (0b0101010101010101 , a , b) ; let e = _mm512_set_ps (0. , AND_32 , 0. , AND_32 , 0. , AND_32 , 0. , AND_32 , 0. , AND_32 , 0. , AND_32 , 0. , AND_32 , 0. , AND_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_andnot_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let src = _mm_set_pd (1. , 2.) ; let r = _mm_mask_andnot_pd (src , 0b01 , a , b) ; let e = _mm_set_pd (1. , ANDN_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_andnot_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let r = _mm_maskz_andnot_pd (0b01 , a , b) ; let e = _mm_set_pd (0.0 , ANDN_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_andnot_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let src = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_mask_andnot_pd (src , 0b0101 , a , b) ; let e = _mm256_set_pd (1. , ANDN_64 , 3. , ANDN_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_andnot_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let r = _mm256_maskz_andnot_pd (0b0101 , a , b) ; let e = _mm256_set_pd (0.0 , ANDN_64 , 0.0 , ANDN_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_andnot_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_andnot_pd (a , b) ; let e = _mm512_set1_pd (ANDN_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_andnot_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let src = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_mask_andnot_pd (src , 0b01010101 , a , b) ; let e = _mm512_set_pd (1. , ANDN_64 , 3. , ANDN_64 , 5. , ANDN_64 , 7. , ANDN_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_andnot_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_andnot_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_andnot_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_andnot_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_maskz_andnot_pd (0b01010101 , a , b) ; let e = _mm512_set_pd (0.0 , ANDN_64 , 0.0 , ANDN_64 , 0.0 , ANDN_64 , 0.0 , ANDN_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_andnot_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let src = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_mask_andnot_ps (src , 0b0101 , a , b) ; let e = _mm_set_ps (1. , ANDN_32 , 3. , ANDN_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_andnot_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let r = _mm_maskz_andnot_ps (0b0101 , a , b) ; let e = _mm_set_ps (0.0 , ANDN_32 , 0.0 , ANDN_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_andnot_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let src = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm256_mask_andnot_ps (src , 0b01010101 , a , b) ; let e = _mm256_set_ps (1. , ANDN_32 , 3. , ANDN_32 , 5. , ANDN_32 , 7. , ANDN_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_andnot_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let r = _mm256_maskz_andnot_ps (0b01010101 , a , b) ; let e = _mm256_set_ps (0.0 , ANDN_32 , 0.0 , ANDN_32 , 0.0 , ANDN_32 , 0.0 , ANDN_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_andnot_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_andnot_ps (a , b) ; let e = _mm512_set1_ps (ANDN_32) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_andnot_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let src = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_mask_andnot_ps (src , 0b0101010101010101 , a , b) ; let e = _mm512_set_ps (1. , ANDN_32 , 3. , ANDN_32 , 5. , ANDN_32 , 7. , ANDN_32 , 9. , ANDN_32 , 11. , ANDN_32 , 13. , ANDN_32 , 15. , ANDN_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_andnot_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_andnot_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_andnot_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_andnot_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_maskz_andnot_ps (0b0101010101010101 , a , b) ; let e = _mm512_set_ps (0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 , 0. , ANDN_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_or_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_or_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let src = _mm_set_pd (1. , 2.) ; let r = _mm_mask_or_pd (src , 0b01 , a , b) ; let e = _mm_set_pd (1. , OR_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_or_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_or_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let r = _mm_maskz_or_pd (0b01 , a , b) ; let e = _mm_set_pd (0.0 , OR_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_or_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_or_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let src = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_mask_or_pd (src , 0b0101 , a , b) ; let e = _mm256_set_pd (1. , OR_64 , 3. , OR_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_or_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_or_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let r = _mm256_maskz_or_pd (0b0101 , a , b) ; let e = _mm256_set_pd (0.0 , OR_64 , 0.0 , OR_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_or_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_or_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_or_pd (a , b) ; let e = _mm512_set1_pd (OR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_or_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_or_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let src = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_mask_or_pd (src , 0b01010101 , a , b) ; let e = _mm512_set_pd (1. , OR_64 , 3. , OR_64 , 5. , OR_64 , 7. , OR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_or_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_or_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_or_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_or_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_maskz_or_pd (0b01010101 , a , b) ; let e = _mm512_set_pd (0.0 , OR_64 , 0.0 , OR_64 , 0.0 , OR_64 , 0.0 , OR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_or_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_or_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let src = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_mask_or_ps (src , 0b0101 , a , b) ; let e = _mm_set_ps (1. , OR_32 , 3. , OR_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_or_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_or_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let r = _mm_maskz_or_ps (0b0101 , a , b) ; let e = _mm_set_ps (0.0 , OR_32 , 0.0 , OR_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_or_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_or_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let src = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm256_mask_or_ps (src , 0b01010101 , a , b) ; let e = _mm256_set_ps (1. , OR_32 , 3. , OR_32 , 5. , OR_32 , 7. , OR_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_or_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_or_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let r = _mm256_maskz_or_ps (0b01010101 , a , b) ; let e = _mm256_set_ps (0.0 , OR_32 , 0.0 , OR_32 , 0.0 , OR_32 , 0.0 , OR_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_or_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_or_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_or_ps (a , b) ; let e = _mm512_set1_ps (OR_32) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_or_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_or_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let src = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_mask_or_ps (src , 0b0101010101010101 , a , b) ; let e = _mm512_set_ps (1. , OR_32 , 3. , OR_32 , 5. , OR_32 , 7. , OR_32 , 9. , OR_32 , 11. , OR_32 , 13. , OR_32 , 15. , OR_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_or_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_or_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_or_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_or_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_maskz_or_ps (0b0101010101010101 , a , b) ; let e = _mm512_set_ps (0. , OR_32 , 0. , OR_32 , 0. , OR_32 , 0. , OR_32 , 0. , OR_32 , 0. , OR_32 , 0. , OR_32 , 0. , OR_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_xor_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let src = _mm_set_pd (1. , 2.) ; let r = _mm_mask_xor_pd (src , 0b01 , a , b) ; let e = _mm_set_pd (1. , XOR_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_xor_pd () { let a = _mm_set1_pd (OPRND1_64) ; let b = _mm_set1_pd (OPRND2_64) ; let r = _mm_maskz_xor_pd (0b01 , a , b) ; let e = _mm_set_pd (0.0 , XOR_64) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_xor_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let src = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_mask_xor_pd (src , 0b0101 , a , b) ; let e = _mm256_set_pd (1. , XOR_64 , 3. , XOR_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_xor_pd () { let a = _mm256_set1_pd (OPRND1_64) ; let b = _mm256_set1_pd (OPRND2_64) ; let r = _mm256_maskz_xor_pd (0b0101 , a , b) ; let e = _mm256_set_pd (0.0 , XOR_64 , 0.0 , XOR_64) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_xor_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_xor_pd (a , b) ; let e = _mm512_set1_pd (XOR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_xor_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let src = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_mask_xor_pd (src , 0b01010101 , a , b) ; let e = _mm512_set_pd (1. , XOR_64 , 3. , XOR_64 , 5. , XOR_64 , 7. , XOR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_xor_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_xor_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_xor_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_xor_pd () { let a = _mm512_set1_pd (OPRND1_64) ; let b = _mm512_set1_pd (OPRND2_64) ; let r = _mm512_maskz_xor_pd (0b01010101 , a , b) ; let e = _mm512_set_pd (0.0 , XOR_64 , 0.0 , XOR_64 , 0.0 , XOR_64 , 0.0 , XOR_64) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_xor_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let src = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_mask_xor_ps (src , 0b0101 , a , b) ; let e = _mm_set_ps (1. , XOR_32 , 3. , XOR_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_xor_ps () { let a = _mm_set1_ps (OPRND1_32) ; let b = _mm_set1_ps (OPRND2_32) ; let r = _mm_maskz_xor_ps (0b0101 , a , b) ; let e = _mm_set_ps (0.0 , XOR_32 , 0.0 , XOR_32) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_xor_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let src = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm256_mask_xor_ps (src , 0b01010101 , a , b) ; let e = _mm256_set_ps (1. , XOR_32 , 3. , XOR_32 , 5. , XOR_32 , 7. , XOR_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_xor_ps () { let a = _mm256_set1_ps (OPRND1_32) ; let b = _mm256_set1_ps (OPRND2_32) ; let r = _mm256_maskz_xor_ps (0b01010101 , a , b) ; let e = _mm256_set_ps (0.0 , XOR_32 , 0.0 , XOR_32 , 0.0 , XOR_32 , 0.0 , XOR_32) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_xor_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_xor_ps (a , b) ; let e = _mm512_set1_ps (XOR_32) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_xor_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let src = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_mask_xor_ps (src , 0b0101010101010101 , a , b) ; let e = _mm512_set_ps (1. , XOR_32 , 3. , XOR_32 , 5. , XOR_32 , 7. , XOR_32 , 9. , XOR_32 , 11. , XOR_32 , 13. , XOR_32 , 15. , XOR_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_xor_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_xor_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_xor_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_xor_ps () { let a = _mm512_set1_ps (OPRND1_32) ; let b = _mm512_set1_ps (OPRND2_32) ; let r = _mm512_maskz_xor_ps (0b0101010101010101 , a , b) ; let e = _mm512_set_ps (0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 , 0. , XOR_32 ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm256_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_broadcast_f32x2 (a) ; let e = _mm256_set_ps (3. , 4. , 3. , 4. , 3. , 4. , 3. , 4.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_mask_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm256_set_ps (5. , 6. , 7. , 8. , 9. , 10. , 11. , 12.) ; let r = _mm256_mask_broadcast_f32x2 (b , 0b01101001 , a) ; let e = _mm256_set_ps (5. , 4. , 3. , 8. , 3. , 10. , 11. , 4.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_broadcast_f32x2 (0b01101001 , a) ; let e = _mm256_set_ps (0. , 4. , 3. , 0. , 3. , 0. , 0. , 4.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm512_broadcast_f32x2 (a) ; let e = _mm512_set_ps (3. , 4. , 3. , 4. , 3. , 4. , 3. , 4. , 3. , 4. , 3. , 4. , 3. , 4. , 3. , 4. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm512_set_ps (5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. , 17. , 18. , 19. , 20. ,) ; let r = _mm512_mask_broadcast_f32x2 (b , 0b0110100100111100 , a) ; let e = _mm512_set_ps (5. , 4. , 3. , 8. , 3. , 10. , 11. , 4. , 13. , 14. , 3. , 4. , 3. , 4. , 19. , 20. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_f32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_f32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_f32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_f32x2 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm512_maskz_broadcast_f32x2 (0b0110100100111100 , a) ; let e = _mm512_set_ps (0. , 4. , 3. , 0. , 3. , 0. , 0. , 4. , 0. , 0. , 3. , 4. , 3. , 4. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_f32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_f32x8 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_broadcast_f32x8 (a) ; let e = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_f32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_f32x8 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. , 17. , 18. , 19. , 20. , 21. , 22. , 23. , 24. ,) ; let r = _mm512_mask_broadcast_f32x8 (b , 0b0110100100111100 , a) ; let e = _mm512_set_ps (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8. , 17. , 18. , 3. , 4. , 5. , 6. , 23. , 24. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_f32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_f32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_f32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_f32x8 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_broadcast_f32x8 (0b0110100100111100 , a) ; let e = _mm512_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8. , 0. , 0. , 3. , 4. , 5. , 6. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm256_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm256_broadcast_f64x2 (a) ; let e = _mm256_set_pd (1. , 2. , 1. , 2.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm256_set_pd (3. , 4. , 5. , 6.) ; let r = _mm256_mask_broadcast_f64x2 (b , 0b0110 , a) ; let e = _mm256_set_pd (3. , 2. , 1. , 6.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm256_maskz_broadcast_f64x2 (0b0110 , a) ; let e = _mm256_set_pd (0. , 2. , 1. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm512_broadcast_f64x2 (a) ; let e = _mm512_set_pd (1. , 2. , 1. , 2. , 1. , 2. , 1. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm512_set_pd (3. , 4. , 5. , 6. , 7. , 8. , 9. , 10.) ; let r = _mm512_mask_broadcast_f64x2 (b , 0b01101001 , a) ; let e = _mm512_set_pd (3. , 2. , 1. , 6. , 1. , 8. , 9. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_f64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_f64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_f64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_f64x2 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm512_maskz_broadcast_f64x2 (0b01101001 , a) ; let e = _mm512_set_pd (0. , 2. , 1. , 0. , 1. , 0. , 0. , 2.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_broadcast_i32x2 (a) ; let e = _mm_set_epi32 (3 , 4 , 3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let b = _mm_set_epi32 (5 , 6 , 7 , 8) ; let r = _mm_mask_broadcast_i32x2 (b , 0b0110 , a) ; let e = _mm_set_epi32 (5 , 4 , 3 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm_maskz_broadcast_i32x2 (0b0110 , a) ; let e = _mm_set_epi32 (0 , 4 , 3 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm256_broadcast_i32x2 (a) ; let e = _mm256_set_epi32 (3 , 4 , 3 , 4 , 3 , 4 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let b = _mm256_set_epi32 (5 , 6 , 7 , 8 , 9 , 10 , 11 , 12) ; let r = _mm256_mask_broadcast_i32x2 (b , 0b01101001 , a) ; let e = _mm256_set_epi32 (5 , 4 , 3 , 8 , 3 , 10 , 11 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm256_maskz_broadcast_i32x2 (0b01101001 , a) ; let e = _mm256_set_epi32 (0 , 4 , 3 , 0 , 3 , 0 , 0 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm512_broadcast_i32x2 (a) ; let e = _mm512_set_epi32 (3 , 4 , 3 , 4 , 3 , 4 , 3 , 4 , 3 , 4 , 3 , 4 , 3 , 4 , 3 , 4) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let b = _mm512_set_epi32 (5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20) ; let r = _mm512_mask_broadcast_i32x2 (b , 0b0110100100111100 , a) ; let e = _mm512_set_epi32 (5 , 4 , 3 , 8 , 3 , 10 , 11 , 4 , 13 , 14 , 3 , 4 , 3 , 4 , 19 , 20) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_i32x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_i32x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_i32x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_i32x2 () { let a = _mm_set_epi32 (1 , 2 , 3 , 4) ; let r = _mm512_maskz_broadcast_i32x2 (0b0110100100111100 , a) ; let e = _mm512_set_epi32 (0 , 4 , 3 , 0 , 3 , 0 , 0 , 4 , 0 , 0 , 3 , 4 , 3 , 4 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_i32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_i32x8 () { let a = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_broadcast_i32x8 (a) ; let e = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_i32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_i32x8 () { let a = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi32 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 ,) ; let r = _mm512_mask_broadcast_i32x8 (b , 0b0110100100111100 , a) ; let e = _mm512_set_epi32 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8 , 17 , 18 , 3 , 4 , 5 , 6 , 23 , 24) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_i32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_i32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_i32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_i32x8 () { let a = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_broadcast_i32x8 (0b0110100100111100 , a) ; let e = _mm512_set_epi32 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8 , 0 , 0 , 3 , 4 , 5 , 6 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm256_broadcast_i64x2 (a) ; let e = _mm256_set_epi64x (1 , 2 , 1 , 2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm256_set_epi64x (3 , 4 , 5 , 6) ; let r = _mm256_mask_broadcast_i64x2 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (3 , 2 , 1 , 6) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm256_maskz_broadcast_i64x2 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm512_broadcast_i64x2 (a) ; let e = _mm512_set_epi64 (1 , 2 , 1 , 2 , 1 , 2 , 1 , 2) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm512_set_epi64 (3 , 4 , 5 , 6 , 7 , 8 , 9 , 10) ; let r = _mm512_mask_broadcast_i64x2 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (3 , 2 , 1 , 6 , 1 , 8 , 9 , 2) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_broadcast_i64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_broadcast_i64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_broadcast_i64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_broadcast_i64x2 () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm512_maskz_broadcast_i64x2 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 1 , 0 , 1 , 0 , 0 , 2) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extractf32x8_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_extractf32x8_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_extractf32x8_ps :: < 1 > (a) ; let e = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extractf32x8_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_extractf32x8_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm256_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_mask_extractf32x8_ps :: < 1 > (b , 0b01101001 , a) ; let e = _mm256_set_ps (17. , 2. , 3. , 20. , 5. , 22. , 23. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_extractf32x8_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extractf32x8_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extractf32x8_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_extractf32x8_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let r = _mm512_maskz_extractf32x8_ps :: < 1 > (0b01101001 , a) ; let e = _mm256_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_extractf64x2_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_extractf64x2_pd :: < 1 > (a) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_mask_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_extractf64x2_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm_set_pd (5. , 6.) ; let r = _mm256_mask_extractf64x2_pd :: < 1 > (b , 0b01 , a) ; let e = _mm_set_pd (5. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_maskz_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_extractf64x2_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_extractf64x2_pd :: < 1 > (0b01 , a) ; let e = _mm_set_pd (0. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_extractf64x2_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_extractf64x2_pd :: < 2 > (a) ; let e = _mm_set_pd (3. , 4.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_mask_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_extractf64x2_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm_set_pd (9. , 10.) ; let r = _mm512_mask_extractf64x2_pd :: < 2 > (b , 0b01 , a) ; let e = _mm_set_pd (9. , 4.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_maskz_extractf64x2_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extractf64x2_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extractf64x2_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_extractf64x2_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_extractf64x2_pd :: < 2 > (0b01 , a) ; let e = _mm_set_pd (0. , 4.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm512_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extracti32x8_epi32_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_extracti32x8_epi32 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_extracti32x8_epi32 :: < 1 > (a) ; let e = _mm256_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mask_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extracti32x8_epi32_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_extracti32x8_epi32 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let b = _mm256_set_epi32 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_mask_extracti32x8_epi32 :: < 1 > (b , 0b01101001 , a) ; let e = _mm256_set_epi32 (17 , 2 , 3 , 20 , 5 , 22 , 23 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_maskz_extracti32x8_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extracti32x8_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extracti32x8_epi32_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_extracti32x8_epi32 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_maskz_extracti32x8_epi32 :: < 1 > (0b01101001 , a) ; let e = _mm256_set_epi32 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_extracti64x2_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_extracti64x2_epi64 :: < 1 > (a) ; let e = _mm_set_epi64x (1 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mask_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_extracti64x2_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm256_mask_extracti64x2_epi64 :: < 1 > (b , 0b01 , a) ; let e = _mm_set_epi64x (5 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_maskz_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_extracti64x2_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_extracti64x2_epi64 :: < 1 > (0b01 , a) ; let e = _mm_set_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_extracti64x2_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_extracti64x2_epi64 :: < 2 > (a) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_mask_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_extracti64x2_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_set_epi64x (9 , 10) ; let r = _mm512_mask_extracti64x2_epi64 :: < 2 > (b , 0b01 , a) ; let e = _mm_set_epi64x (9 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_maskz_extracti64x2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_extracti64x2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_extracti64x2_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_extracti64x2_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_extracti64x2_epi64 :: < 2 > (0b01 , a) ; let e = _mm_set_epi64x (0 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_insertf32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_insertf32x8 () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm256_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_insertf32x8 :: < 1 > (a , b) ; let e = _mm512_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_insertf32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_insertf32x8 () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm256_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let src = _mm512_set_ps (25. , 26. , 27. , 28. , 29. , 30. , 31. , 32. , 33. , 34. , 35. , 36. , 37. , 38. , 39. , 40. ,) ; let r = _mm512_mask_insertf32x8 :: < 1 > (src , 0b0110100100111100 , a , b) ; let e = _mm512_set_ps (25. , 18. , 19. , 28. , 21. , 30. , 31. , 24. , 33. , 34. , 11. , 12. , 13. , 14. , 39. , 40. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_insertf32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_insertf32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_insertf32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_insertf32x8 () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm256_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24.) ; let r = _mm512_maskz_insertf32x8 :: < 1 > (0b0110100100111100 , a , b) ; let e = _mm512_set_ps (0. , 18. , 19. , 0. , 21. , 0. , 0. , 24. , 0. , 0. , 11. , 12. , 13. , 14. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm256_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_insertf64x2 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm_set_pd (5. , 6.) ; let r = _mm256_insertf64x2 :: < 1 > (a , b) ; let e = _mm256_set_pd (5. , 6. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_insertf64x2 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm_set_pd (5. , 6.) ; let src = _mm256_set_pd (7. , 8. , 9. , 10.) ; let r = _mm256_mask_insertf64x2 :: < 1 > (src , 0b0110 , a , b) ; let e = _mm256_set_pd (7. , 6. , 3. , 10.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_insertf64x2 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm_set_pd (5. , 6.) ; let r = _mm256_maskz_insertf64x2 :: < 1 > (0b0110 , a , b) ; let e = _mm256_set_pd (0. , 6. , 3. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_insertf64x2 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm_set_pd (9. , 10.) ; let r = _mm512_insertf64x2 :: < 2 > (a , b) ; let e = _mm512_set_pd (1. , 2. , 9. , 10. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_insertf64x2 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm_set_pd (9. , 10.) ; let src = _mm512_set_pd (11. , 12. , 13. , 14. , 15. , 16. , 17. , 18.) ; let r = _mm512_mask_insertf64x2 :: < 2 > (src , 0b01101001 , a , b) ; let e = _mm512_set_pd (11. , 2. , 9. , 14. , 5. , 16. , 17. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_insertf64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_insertf64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_insertf64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_insertf64x2 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm_set_pd (9. , 10.) ; let r = _mm512_maskz_insertf64x2 :: < 2 > (0b01101001 , a , b) ; let e = _mm512_set_pd (0. , 2. , 9. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_inserti32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_inserti32x8 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let b = _mm256_set_epi32 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_inserti32x8 :: < 1 > (a , b) ; let e = _mm512_set_epi32 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_inserti32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_inserti32x8 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let b = _mm256_set_epi32 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let src = _mm512_set_epi32 (25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 ,) ; let r = _mm512_mask_inserti32x8 :: < 1 > (src , 0b0110100100111100 , a , b) ; let e = _mm512_set_epi32 (25 , 18 , 19 , 28 , 21 , 30 , 31 , 24 , 33 , 34 , 11 , 12 , 13 , 14 , 39 , 40 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_inserti32x8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_inserti32x8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_inserti32x8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_inserti32x8 () { let a = _mm512_set_epi32 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let b = _mm256_set_epi32 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_maskz_inserti32x8 :: < 1 > (0b0110100100111100 , a , b) ; let e = _mm512_set_epi32 (0 , 18 , 19 , 0 , 21 , 0 , 0 , 24 , 0 , 0 , 11 , 12 , 13 , 14 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_inserti64x2 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm256_inserti64x2 :: < 1 > (a , b) ; let e = _mm256_set_epi64x (5 , 6 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_inserti64x2 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_epi64x (5 , 6) ; let src = _mm256_set_epi64x (7 , 8 , 9 , 10) ; let r = _mm256_mask_inserti64x2 :: < 1 > (src , 0b0110 , a , b) ; let e = _mm256_set_epi64x (7 , 6 , 3 , 10) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_inserti64x2 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm256_maskz_inserti64x2 :: < 1 > (0b0110 , a , b) ; let e = _mm256_set_epi64x (0 , 6 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_inserti64x2 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_set_epi64x (9 , 10) ; let r = _mm512_inserti64x2 :: < 2 > (a , b) ; let e = _mm512_set_epi64 (1 , 2 , 9 , 10 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_inserti64x2 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_set_epi64x (9 , 10) ; let src = _mm512_set_epi64 (11 , 12 , 13 , 14 , 15 , 16 , 17 , 18) ; let r = _mm512_mask_inserti64x2 :: < 2 > (src , 0b01101001 , a , b) ; let e = _mm512_set_epi64 (11 , 2 , 9 , 14 , 5 , 16 , 17 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_inserti64x2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_inserti64x2 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_inserti64x2_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_inserti64x2 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm_set_epi64x (9 , 10) ; let r = _mm512_maskz_inserti64x2 :: < 2 > (0b01101001 , a , b) ; let e = _mm512_set_epi64 (0 , 2 , 9 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvt_roundepi64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvt_roundepi64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_pd (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvt_roundepi64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_pd (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtepi64_pd () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_cvtepi64_pd (a) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtepi64_pd () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_pd (3. , 4.) ; let r = _mm_mask_cvtepi64_pd (b , 0b01 , a) ; let e = _mm_set_pd (3. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtepi64_pd () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_maskz_cvtepi64_pd (0b01 , a) ; let e = _mm_set_pd (0. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtepi64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_cvtepi64_pd (a) ; let e = _mm256_set_pd (1. , 2. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_pd (5. , 6. , 7. , 8.) ; let r = _mm256_mask_cvtepi64_pd (b , 0b0110 , a) ; let e = _mm256_set_pd (5. , 2. , 3. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtepi64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_cvtepi64_pd (0b0110 , a) ; let e = _mm256_set_pd (0. , 2. , 3. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvtepi64_pd (a) ; let e = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvtepi64_pd (b , 0b01101001 , a) ; let e = _mm512_set_pd (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtepi64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvtepi64_pd (0b01101001 , a) ; let e = _mm512_set_pd (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvt_roundepi64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvt_roundepi64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm256_set_ps (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvt_roundepi64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm256_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtepi64_ps () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_cvtepi64_ps (a) ; let e = _mm_set_ps (0. , 0. , 1. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtepi64_ps () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_ps (3. , 4. , 5. , 6.) ; let r = _mm_mask_cvtepi64_ps (b , 0b01 , a) ; let e = _mm_set_ps (0. , 0. , 5. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtepi64_ps () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_maskz_cvtepi64_ps (0b01 , a) ; let e = _mm_set_ps (0. , 0. , 0. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtepi64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_cvtepi64_ps (a) ; let e = _mm_set_ps (1. , 2. , 3. , 4.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtepi64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_ps (5. , 6. , 7. , 8.) ; let r = _mm256_mask_cvtepi64_ps (b , 0b0110 , a) ; let e = _mm_set_ps (5. , 2. , 3. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtepi64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_cvtepi64_ps (0b0110 , a) ; let e = _mm_set_ps (0. , 2. , 3. , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm512_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvtepi64_ps (a) ; let e = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvtepi64_ps (b , 0b01101001 , a) ; let e = _mm256_set_ps (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepi64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepi64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepi64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtepi64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvtepi64_ps (0b01101001 , a) ; let e = _mm256_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvt_roundepu64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvt_roundepu64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_pd (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvt_roundepu64_pd :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_pd (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtepu64_pd () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_cvtepu64_pd (a) ; let e = _mm_set_pd (1. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtepu64_pd () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_pd (3. , 4.) ; let r = _mm_mask_cvtepu64_pd (b , 0b01 , a) ; let e = _mm_set_pd (3. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtepu64_pd () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_maskz_cvtepu64_pd (0b01 , a) ; let e = _mm_set_pd (0. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtepu64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_cvtepu64_pd (a) ; let e = _mm256_set_pd (1. , 2. , 3. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtepu64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_pd (5. , 6. , 7. , 8.) ; let r = _mm256_mask_cvtepu64_pd (b , 0b0110 , a) ; let e = _mm256_set_pd (5. , 2. , 3. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtepu64_pd () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_cvtepu64_pd (0b0110 , a) ; let e = _mm256_set_pd (0. , 2. , 3. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvtepu64_pd (a) ; let e = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvtepu64_pd (b , 0b01101001 , a) ; let e = _mm512_set_pd (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu64_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu64_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu64_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtepu64_pd () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvtepu64_pd (0b01101001 , a) ; let e = _mm512_set_pd (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvt_roundepu64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvt_roundepu64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm256_set_ps (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvt_roundepu64_ps :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm256_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtepu64_ps () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_cvtepu64_ps (a) ; let e = _mm_set_ps (0. , 0. , 1. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtepu64_ps () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_ps (3. , 4. , 5. , 6.) ; let r = _mm_mask_cvtepu64_ps (b , 0b01 , a) ; let e = _mm_set_ps (0. , 0. , 5. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtepu64_ps () { let a = _mm_set_epi64x (1 , 2) ; let r = _mm_maskz_cvtepu64_ps (0b01 , a) ; let e = _mm_set_ps (0. , 0. , 0. , 2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtepu64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_cvtepu64_ps (a) ; let e = _mm_set_ps (1. , 2. , 3. , 4.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtepu64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm_set_ps (5. , 6. , 7. , 8.) ; let r = _mm256_mask_cvtepu64_ps (b , 0b0110 , a) ; let e = _mm_set_ps (5. , 2. , 3. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtepu64_ps () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let r = _mm256_maskz_cvtepu64_ps (0b0110 , a) ; let e = _mm_set_ps (0. , 2. , 3. , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm512_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_cvtepu64_ps (a) ; let e = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_mask_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm256_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_cvtepu64_ps (b , 0b01101001 , a) ; let e = _mm256_set_ps (9. , 2. , 3. , 12. , 5. , 14. , 15. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtepu64_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtepu64_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtepu64_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtepu64_ps () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let r = _mm512_maskz_cvtepu64_ps (0b01101001 , a) ; let e = _mm256_set_ps (0. , 2. , 3. , 0. , 5. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvt_roundpd_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvt_roundpd_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvt_roundpd_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_cvtpd_epi64 (a) ; let e = _mm_set_epi64x (1 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_mask_cvtpd_epi64 (b , 0b01 , a) ; let e = _mm_set_epi64x (3 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_maskz_cvtpd_epi64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_cvtpd_epi64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvtpd_epi64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvtpd_epi64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtpd_epi64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtpd_epi64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtpd_epi64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvt_roundps_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvt_roundps_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvt_roundps_epi64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_cvtps_epi64 (a) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm_mask_cvtps_epi64 (b , 0b01 , a) ; let e = _mm_set_epi64x (5 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_maskz_cvtps_epi64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_cvtps_epi64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvtps_epi64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvtps_epi64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtps_epi64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtps_epi64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtps_epi64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvt_roundpd_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvt_roundpd_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvt_roundpd_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_cvtpd_epu64 (a) ; let e = _mm_set_epi64x (1 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_mask_cvtpd_epu64 (b , 0b01 , a) ; let e = _mm_set_epi64x (3 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_maskz_cvtpd_epu64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_cvtpd_epu64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvtpd_epu64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvtpd_epu64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtpd_epu64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtpd_epu64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtpd_epu64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvt_roundps_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvt_roundps_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (b , 0b01101001 , a ,) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvt_roundps_epu64 :: < { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC } > (0b01101001 , a ,) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_cvtps_epu64 (a) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm_mask_cvtps_epu64 (b , 0b01 , a) ; let e = _mm_set_epi64x (5 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_maskz_cvtps_epu64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_cvtps_epu64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvtps_epu64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvtps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvtps_epu64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtps_epu64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtps_epu64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtps_epu64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtt_roundpd_epi64 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtt_roundpd_epi64 :: < _MM_FROUND_NO_EXC > (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtt_roundpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtt_roundpd_epi64 :: < _MM_FROUND_NO_EXC > (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvttpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_cvttpd_epi64 (a) ; let e = _mm_set_epi64x (1 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvttpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_mask_cvttpd_epi64 (b , 0b01 , a) ; let e = _mm_set_epi64x (3 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvttpd_epi64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_maskz_cvttpd_epi64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvttpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_cvttpd_epi64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvttpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvttpd_epi64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvttpd_epi64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvttpd_epi64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvttpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvttpd_epi64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvttpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvttpd_epi64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttpd_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttpd_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttpd_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvttpd_epi64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvttpd_epi64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtt_roundps_epi64 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtt_roundps_epi64 :: < _MM_FROUND_NO_EXC > (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtt_roundps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtt_roundps_epi64 :: < _MM_FROUND_NO_EXC > (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_cvttps_epi64 (a) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm_mask_cvttps_epi64 (b , 0b01 , a) ; let e = _mm_set_epi64x (5 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_maskz_cvttps_epi64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_cvttps_epi64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvttps_epi64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvttps_epi64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvttps_epi64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvttps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvttps_epi64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvttps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvttps_epi64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttps_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttps_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttps_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvttps_epi64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvttps_epi64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtt_roundpd_epu64 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtt_roundpd_epu64 :: < _MM_FROUND_NO_EXC > (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtt_roundpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtt_roundpd_epu64 :: < _MM_FROUND_NO_EXC > (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvttpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_cvttpd_epu64 (a) ; let e = _mm_set_epi64x (1 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvttpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_mask_cvttpd_epu64 (b , 0b01 , a) ; let e = _mm_set_epi64x (3 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvttpd_epu64 () { let a = _mm_set_pd (1. , 2.) ; let r = _mm_maskz_cvttpd_epu64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvttpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_cvttpd_epu64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvttpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvttpd_epu64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvttpd_epu64 () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvttpd_epu64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvttpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvttpd_epu64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvttpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvttpd_epu64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttpd_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttpd_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttpd_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvttpd_epu64 () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvttpd_epu64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvtt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvtt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvtt_roundps_epu64 :: < _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvtt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvtt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvtt_roundps_epu64 :: < _MM_FROUND_NO_EXC > (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvtt_roundps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvtt_roundps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvtt_roundps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvtt_roundps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvtt_roundps_epu64 :: < _MM_FROUND_NO_EXC > (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_cvttps_epu64 (a) ; let e = _mm_set_epi64x (3 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_epi64x (5 , 6) ; let r = _mm_mask_cvttps_epu64 (b , 0b01 , a) ; let e = _mm_set_epi64x (5 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm_maskz_cvttps_epu64 (0b01 , a) ; let e = _mm_set_epi64x (0 , 4) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_cvttps_epu64 (a) ; let e = _mm256_set_epi64x (1 , 2 , 3 , 4) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mask_cvttps_epu64 (b , 0b0110 , a) ; let e = _mm256_set_epi64x (5 , 2 , 3 , 8) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_cvttps_epu64 () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let r = _mm256_maskz_cvttps_epu64 (0b0110 , a) ; let e = _mm256_set_epi64x (0 , 2 , 3 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_cvttps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_cvttps_epu64 (a) ; let e = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_cvttps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mask_cvttps_epu64 (b , 0b01101001 , a) ; let e = _mm512_set_epi64 (9 , 2 , 3 , 12 , 5 , 14 , 15 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_cvttps_epu64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_cvttps_epu64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_cvttps_epu64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_cvttps_epu64 () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r = _mm512_maskz_cvttps_epu64 (0b01101001 , a) ; let e = _mm512_set_epi64 (0 , 2 , 3 , 0 , 5 , 0 , 0 , 8) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mullo_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_mullo_epi64 (a , b) ; let e = _mm_set_epi64x (3 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_mullo_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (3 , 4) ; let c = _mm_set_epi64x (5 , 6) ; let r = _mm_mask_mullo_epi64 (c , 0b01 , a , b) ; let e = _mm_set_epi64x (5 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_mullo_epi64 () { let a = _mm_set_epi64x (1 , 2) ; let b = _mm_set_epi64x (3 , 4) ; let r = _mm_maskz_mullo_epi64 (0b01 , a , b) ; let e = _mm_set_epi64x (0 , 8) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mullo_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_mullo_epi64 (a , b) ; let e = _mm256_set_epi64x (5 , 12 , 21 , 32) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_mullo_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let c = _mm256_set_epi64x (9 , 10 , 11 , 12) ; let r = _mm256_mask_mullo_epi64 (c , 0b0110 , a , b) ; let e = _mm256_set_epi64x (9 , 12 , 21 , 12) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_mullo_epi64 () { let a = _mm256_set_epi64x (1 , 2 , 3 , 4) ; let b = _mm256_set_epi64x (5 , 6 , 7 , 8) ; let r = _mm256_maskz_mullo_epi64 (0b0110 , a , b) ; let e = _mm256_set_epi64x (0 , 12 , 21 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mullo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_mullo_epi64 (a , b) ; let e = _mm512_set_epi64 (9 , 20 , 33 , 48 , 65 , 84 , 105 , 128) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_mullo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let c = _mm512_set_epi64 (17 , 18 , 19 , 20 , 21 , 22 , 23 , 24) ; let r = _mm512_mask_mullo_epi64 (c , 0b01101001 , a , b) ; let e = _mm512_set_epi64 (17 , 20 , 33 , 20 , 65 , 22 , 23 , 128) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_mullo_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_mullo_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_mullo_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_mullo_epi64 () { let a = _mm512_set_epi64 (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8) ; let b = _mm512_set_epi64 (9 , 10 , 11 , 12 , 13 , 14 , 15 , 16) ; let r = _mm512_maskz_mullo_epi64 (0b01101001 , a , b) ; let e = _mm512_set_epi64 (0 , 20 , 33 , 0 , 65 , 0 , 0 , 128) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_cvtmask8_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_cvtmask8_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_cvtmask8_u32_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_cvtmask8_u32 () { let a : __mmask8 = 0b01101001 ; let r = _cvtmask8_u32 (a) ; let e : u32 = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_cvtu32_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_cvtu32_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_cvtu32_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_cvtu32_mask8 () { let a : u32 = 0b01101001 ; let r = _cvtu32_mask8 (a) ; let e : __mmask8 = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kadd_mask16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kadd_mask16 in module {}", module_path!());
    };
}

mkfn!{
    test_kadd_mask16_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kadd_mask16 () { let a : __mmask16 = 27549 ; let b : __mmask16 = 23434 ; let r = _kadd_mask16 (a , b) ; let e : __mmask16 = 50983 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kadd_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kadd_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kadd_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kadd_mask8 () { let a : __mmask8 = 98 ; let b : __mmask8 = 117 ; let r = _kadd_mask8 (a , b) ; let e : __mmask8 = 215 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kand_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kand_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kand_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kand_mask8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110011 ; let r = _kand_mask8 (a , b) ; let e : __mmask8 = 0b00100001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kandn_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kandn_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kandn_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kandn_mask8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110011 ; let r = _kandn_mask8 (a , b) ; let e : __mmask8 = 0b10010010 ; assert_eq ! (r , e) ; }
}

macro_rules! test_knot_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_knot_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_knot_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_knot_mask8 () { let a : __mmask8 = 0b01101001 ; let r = _knot_mask8 (a) ; let e : __mmask8 = 0b10010110 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kor_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kor_mask8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110011 ; let r = _kor_mask8 (a , b) ; let e : __mmask8 = 0b11111011 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kxnor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kxnor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kxnor_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kxnor_mask8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110011 ; let r = _kxnor_mask8 (a , b) ; let e : __mmask8 = 0b00100101 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kxor_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kxor_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kxor_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kxor_mask8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110011 ; let r = _kxor_mask8 (a , b) ; let e : __mmask8 = 0b11011010 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kortest_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kortest_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_kortest_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kortest_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110110 ; let mut all_ones : u8 = 0 ; let r = _kortest_mask8_u8 (a , b , & mut all_ones) ; assert_eq ! (r , 0) ; assert_eq ! (all_ones , 1) ; }
}

macro_rules! test_kortestc_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kortestc_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_kortestc_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kortestc_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110110 ; let r = _kortestc_mask8_u8 (a , b) ; assert_eq ! (r , 1) ; }
}

macro_rules! test_kortestz_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kortestz_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_kortestz_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kortestz_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10110110 ; let r = _kortestz_mask8_u8 (a , b) ; assert_eq ! (r , 0) ; }
}

macro_rules! test_kshiftli_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kshiftli_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kshiftli_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kshiftli_mask8 () { let a : __mmask8 = 0b01101001 ; let r = _kshiftli_mask8 :: < 3 > (a) ; let e : __mmask8 = 0b01001000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_kshiftri_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_kshiftri_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_kshiftri_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_kshiftri_mask8 () { let a : __mmask8 = 0b01101001 ; let r = _kshiftri_mask8 :: < 3 > (a) ; let e : __mmask8 = 0b00001101 ; assert_eq ! (r , e) ; }
}

macro_rules! test_ktest_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktest_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktest_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktest_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10010110 ; let mut and_not : u8 = 0 ; let r = _ktest_mask8_u8 (a , b , & mut and_not) ; assert_eq ! (r , 1) ; assert_eq ! (and_not , 0) ; }
}

macro_rules! test_ktestc_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktestc_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktestc_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktestc_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10010110 ; let r = _ktestc_mask8_u8 (a , b) ; assert_eq ! (r , 0) ; }
}

macro_rules! test_ktestz_mask8_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktestz_mask8_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktestz_mask8_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktestz_mask8_u8 () { let a : __mmask8 = 0b01101001 ; let b : __mmask8 = 0b10010110 ; let r = _ktestz_mask8_u8 (a , b) ; assert_eq ! (r , 1) ; }
}

macro_rules! test_ktest_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktest_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktest_mask16_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktest_mask16_u8 () { let a : __mmask16 = 0b0110100100111100 ; let b : __mmask16 = 0b1001011011000011 ; let mut and_not : u8 = 0 ; let r = _ktest_mask16_u8 (a , b , & mut and_not) ; assert_eq ! (r , 1) ; assert_eq ! (and_not , 0) ; }
}

macro_rules! test_ktestc_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktestc_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktestc_mask16_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktestc_mask16_u8 () { let a : __mmask16 = 0b0110100100111100 ; let b : __mmask16 = 0b1001011011000011 ; let r = _ktestc_mask16_u8 (a , b) ; assert_eq ! (r , 0) ; }
}

macro_rules! test_ktestz_mask16_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ktestz_mask16_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_ktestz_mask16_u8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_ktestz_mask16_u8 () { let a : __mmask16 = 0b0110100100111100 ; let b : __mmask16 = 0b1001011011000011 ; let r = _ktestz_mask16_u8 (a , b) ; assert_eq ! (r , 1) ; }
}

macro_rules! test_load_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_load_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_load_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_load_mask8 () { let a : __mmask8 = 0b01101001 ; let r = _load_mask8 (& a) ; let e : __mmask8 = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_store_mask8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_store_mask8 in module {}", module_path!());
    };
}

mkfn!{
    test_store_mask8_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_store_mask8 () { let a : __mmask8 = 0b01101001 ; let mut r = 0 ; _store_mask8 (& mut r , a) ; let e : __mmask8 = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movepi32_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_movepi32_mask () { let a = _mm_set_epi32 (0 , - 2 , - 3 , 4) ; let r = _mm_movepi32_mask (a) ; let e = 0b0110 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_movepi32_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_movepi32_mask () { let a = _mm256_set_epi32 (0 , - 2 , - 3 , 4 , - 5 , 6 , 7 , - 8) ; let r = _mm256_movepi32_mask (a) ; let e = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_movepi32_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_movepi32_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_movepi32_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_movepi32_mask () { let a = _mm512_set_epi32 (0 , - 2 , - 3 , 4 , - 5 , 6 , 7 , - 8 , 9 , 10 , - 11 , - 12 , - 13 , - 14 , 15 , 16 ,) ; let r = _mm512_movepi32_mask (a) ; let e = 0b0110100100111100 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movepi64_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_movepi64_mask () { let a = _mm_set_epi64x (0 , - 2) ; let r = _mm_movepi64_mask (a) ; let e = 0b01 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_movepi64_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_movepi64_mask () { let a = _mm256_set_epi64x (0 , - 2 , - 3 , 4) ; let r = _mm256_movepi64_mask (a) ; let e = 0b0110 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_movepi64_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_movepi64_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_movepi64_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_movepi64_mask () { let a = _mm512_set_epi64 (0 , - 2 , - 3 , 4 , - 5 , 6 , 7 , - 8) ; let r = _mm512_movepi64_mask (a) ; let e = 0b01101001 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movm_epi32_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_movm_epi32 () { let a = 0b0110 ; let r = _mm_movm_epi32 (a) ; let e = _mm_set_epi32 (0 , - 1 , - 1 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_movm_epi32_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_movm_epi32 () { let a = 0b01101001 ; let r = _mm256_movm_epi32 (a) ; let e = _mm256_set_epi32 (0 , - 1 , - 1 , 0 , - 1 , 0 , 0 , - 1) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_movm_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_movm_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_movm_epi32_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_movm_epi32 () { let a = 0b0110100100111100 ; let r = _mm512_movm_epi32 (a) ; let e = _mm512_set_epi32 (0 , - 1 , - 1 , 0 , - 1 , 0 , 0 , - 1 , 0 , 0 , - 1 , - 1 , - 1 , - 1 , 0 , 0) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_movm_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_movm_epi64 () { let a = 0b01 ; let r = _mm_movm_epi64 (a) ; let e = _mm_set_epi64x (0 , - 1) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_movm_epi64_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_movm_epi64 () { let a = 0b0110 ; let r = _mm256_movm_epi64 (a) ; let e = _mm256_set_epi64x (0 , - 1 , - 1 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm512_movm_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_movm_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_movm_epi64_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_movm_epi64 () { let a = 0b01101001 ; let r = _mm512_movm_epi64 (a) ; let e = _mm512_set_epi64 (0 , - 1 , - 1 , 0 , - 1 , 0 , 0 , - 1) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_range_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_range_round_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm512_range_round_pd :: < 0b0101 , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm512_set_pd (2. , 2. , 4. , 4. , 6. , 6. , 8. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_range_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_range_round_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let c = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_range_round_pd :: < 0b0101 , _MM_FROUND_NO_EXC > (c , 0b01101001 , a , b) ; let e = _mm512_set_pd (9. , 2. , 4. , 12. , 6. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_range_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_range_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_range_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_range_round_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm512_maskz_range_round_pd :: < 0b0101 , _MM_FROUND_NO_EXC > (0b01101001 , a , b) ; let e = _mm512_set_pd (0. , 2. , 4. , 0. , 6. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_range_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (2. , 1.) ; let r = _mm_range_pd :: < 0b0101 > (a , b) ; let e = _mm_set_pd (2. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_range_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (2. , 1.) ; let c = _mm_set_pd (3. , 4.) ; let r = _mm_mask_range_pd :: < 0b0101 > (c , 0b01 , a , b) ; let e = _mm_set_pd (3. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_range_pd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_pd (2. , 1.) ; let r = _mm_maskz_range_pd :: < 0b0101 > (0b01 , a , b) ; let e = _mm_set_pd (0. , 2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_range_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (2. , 1. , 4. , 3.) ; let r = _mm256_range_pd :: < 0b0101 > (a , b) ; let e = _mm256_set_pd (2. , 2. , 4. , 4.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_range_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (2. , 1. , 4. , 3.) ; let c = _mm256_set_pd (5. , 6. , 7. , 8.) ; let r = _mm256_mask_range_pd :: < 0b0101 > (c , 0b0110 , a , b) ; let e = _mm256_set_pd (5. , 2. , 4. , 8.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_range_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_range_pd () { let a = _mm256_set_pd (1. , 2. , 3. , 4.) ; let b = _mm256_set_pd (2. , 1. , 4. , 3.) ; let r = _mm256_maskz_range_pd :: < 0b0101 > (0b0110 , a , b) ; let e = _mm256_set_pd (0. , 2. , 4. , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_range_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_range_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm512_range_pd :: < 0b0101 > (a , b) ; let e = _mm512_set_pd (2. , 2. , 4. , 4. , 6. , 6. , 8. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_range_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_range_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let c = _mm512_set_pd (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm512_mask_range_pd :: < 0b0101 > (c , 0b01101001 , a , b) ; let e = _mm512_set_pd (9. , 2. , 4. , 12. , 6. , 14. , 15. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_range_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_range_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_range_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_range_pd () { let a = _mm512_set_pd (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm512_set_pd (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm512_maskz_range_pd :: < 0b0101 > (0b01101001 , a , b) ; let e = _mm512_set_pd (0. , 2. , 4. , 0. , 6. , 0. , 0. , 8.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_range_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_range_round_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let r = _mm512_range_round_ps :: < 0b0101 , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm512_set_ps (2. , 2. , 4. , 4. , 6. , 6. , 8. , 8. , 10. , 10. , 12. , 12. , 14. , 14. , 16. , 16. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_range_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_range_round_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let c = _mm512_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24. , 25. , 26. , 27. , 28. , 29. , 30. , 31. , 32. ,) ; let r = _mm512_mask_range_round_ps :: < 0b0101 , _MM_FROUND_NO_EXC > (c , 0b0110100100111100 , a , b) ; let e = _mm512_set_ps (17. , 2. , 4. , 20. , 6. , 22. , 23. , 8. , 25. , 26. , 12. , 12. , 14. , 14. , 31. , 32. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_range_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_range_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_range_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_range_round_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let r = _mm512_maskz_range_round_ps :: < 0b0101 , _MM_FROUND_NO_EXC > (0b0110100100111100 , a , b) ; let e = _mm512_set_ps (0. , 2. , 4. , 0. , 6. , 0. , 0. , 8. , 0. , 0. , 12. , 12. , 14. , 14. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_range_ps () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ps (2. , 1. , 4. , 3.) ; let r = _mm_range_ps :: < 0b0101 > (a , b) ; let e = _mm_set_ps (2. , 2. , 4. , 4.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_range_ps () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ps (2. , 1. , 4. , 3.) ; let c = _mm_set_ps (5. , 6. , 7. , 8.) ; let r = _mm_mask_range_ps :: < 0b0101 > (c , 0b0110 , a , b) ; let e = _mm_set_ps (5. , 2. , 4. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_range_ps () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ps (2. , 1. , 4. , 3.) ; let r = _mm_maskz_range_ps :: < 0b0101 > (0b0110 , a , b) ; let e = _mm_set_ps (0. , 2. , 4. , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_range_ps () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm256_range_ps :: < 0b0101 > (a , b) ; let e = _mm256_set_ps (2. , 2. , 4. , 4. , 6. , 6. , 8. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_range_ps () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let c = _mm256_set_ps (9. , 10. , 11. , 12. , 13. , 14. , 15. , 16.) ; let r = _mm256_mask_range_ps :: < 0b0101 > (c , 0b01101001 , a , b) ; let e = _mm256_set_ps (9. , 2. , 4. , 12. , 6. , 14. , 15. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_range_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_range_ps () { let a = _mm256_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let b = _mm256_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7.) ; let r = _mm256_maskz_range_ps :: < 0b0101 > (0b01101001 , a , b) ; let e = _mm256_set_ps (0. , 2. , 4. , 0. , 6. , 0. , 0. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_range_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_range_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let r = _mm512_range_ps :: < 0b0101 > (a , b) ; let e = _mm512_set_ps (2. , 2. , 4. , 4. , 6. , 6. , 8. , 8. , 10. , 10. , 12. , 12. , 14. , 14. , 16. , 16. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_range_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_range_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let c = _mm512_set_ps (17. , 18. , 19. , 20. , 21. , 22. , 23. , 24. , 25. , 26. , 27. , 28. , 29. , 30. , 31. , 32. ,) ; let r = _mm512_mask_range_ps :: < 0b0101 > (c , 0b0110100100111100 , a , b) ; let e = _mm512_set_ps (17. , 2. , 4. , 20. , 6. , 22. , 23. , 8. , 25. , 26. , 12. , 12. , 14. , 14. , 31. , 32. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_range_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_range_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_range_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_range_ps () { let a = _mm512_set_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. ,) ; let b = _mm512_set_ps (2. , 1. , 4. , 3. , 6. , 5. , 8. , 7. , 10. , 9. , 12. , 11. , 14. , 13. , 16. , 15. ,) ; let r = _mm512_maskz_range_ps :: < 0b0101 > (0b0110100100111100 , a , b) ; let e = _mm512_set_ps (0. , 2. , 4. , 0. , 6. , 0. , 0. , 8. , 0. , 0. , 12. , 12. , 14. , 14. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_range_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_range_round_sd () { let a = _mm_set_sd (1.) ; let b = _mm_set_sd (2.) ; let r = _mm_range_round_sd :: < 0b0101 , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm_set_sd (2.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_range_round_sd () { let a = _mm_set_sd (1.) ; let b = _mm_set_sd (2.) ; let c = _mm_set_sd (3.) ; let r = _mm_mask_range_round_sd :: < 0b0101 , _MM_FROUND_NO_EXC > (c , 0b0 , a , b) ; let e = _mm_set_sd (3.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_range_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_range_round_sd () { let a = _mm_set_sd (1.) ; let b = _mm_set_sd (2.) ; let r = _mm_maskz_range_round_sd :: < 0b0101 , _MM_FROUND_NO_EXC > (0b0 , a , b) ; let e = _mm_set_sd (0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_range_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_range_sd () { let a = _mm_set_sd (1.) ; let b = _mm_set_sd (2.) ; let c = _mm_set_sd (3.) ; let r = _mm_mask_range_sd :: < 0b0101 > (c , 0b0 , a , b) ; let e = _mm_set_sd (3.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_range_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_range_sd () { let a = _mm_set_sd (1.) ; let b = _mm_set_sd (2.) ; let r = _mm_maskz_range_sd :: < 0b0101 > (0b0 , a , b) ; let e = _mm_set_sd (0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_range_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_range_round_ss () { let a = _mm_set_ss (1.) ; let b = _mm_set_ss (2.) ; let r = _mm_range_round_ss :: < 0b0101 , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm_set_ss (2.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_range_round_ss () { let a = _mm_set_ss (1.) ; let b = _mm_set_ss (2.) ; let c = _mm_set_ss (3.) ; let r = _mm_mask_range_round_ss :: < 0b0101 , _MM_FROUND_NO_EXC > (c , 0b0 , a , b) ; let e = _mm_set_ss (3.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_range_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_range_round_ss () { let a = _mm_set_ss (1.) ; let b = _mm_set_ss (2.) ; let r = _mm_maskz_range_round_ss :: < 0b0101 , _MM_FROUND_NO_EXC > (0b0 , a , b) ; let e = _mm_set_ss (0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_range_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_range_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_range_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_range_ss () { let a = _mm_set_ss (1.) ; let b = _mm_set_ss (2.) ; let c = _mm_set_ss (3.) ; let r = _mm_mask_range_ss :: < 0b0101 > (c , 0b0 , a , b) ; let e = _mm_set_ss (3.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_range_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_range_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_range_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_range_ss () { let a = _mm_set_ss (1.) ; let b = _mm_set_ss (2.) ; let r = _mm_maskz_range_ss :: < 0b0101 > (0b0 , a , b) ; let e = _mm_set_ss (0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm512_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_reduce_round_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm512_reduce_round_pd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_pd (0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_reduce_round_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let src = _mm512_set_pd (3. , 4. , 5. , 6. , 7. , 8. , 9. , 10.) ; let r = _mm512_mask_reduce_round_pd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (src , 0b01101001 , a ,) ; let e = _mm512_set_pd (3. , 0. , 0.25 , 6. , 0.25 , 8. , 9. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_reduce_round_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_reduce_round_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_reduce_round_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_reduce_round_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm512_maskz_reduce_round_pd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (0b01101001 , a ,) ; let e = _mm512_set_pd (0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_reduce_pd () { let a = _mm_set_pd (0.25 , 0.50) ; let r = _mm_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm_set_pd (0.25 , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_reduce_pd () { let a = _mm_set_pd (0.25 , 0.50) ; let src = _mm_set_pd (3. , 4.) ; let r = _mm_mask_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b01 , a) ; let e = _mm_set_pd (3. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_reduce_pd () { let a = _mm_set_pd (0.25 , 0.50) ; let r = _mm_maskz_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (0b01 , a) ; let e = _mm_set_pd (0. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm256_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_reduce_pd () { let a = _mm256_set_pd (0.25 , 0.50 , 0.75 , 1.0) ; let r = _mm256_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm256_set_pd (0.25 , 0. , 0.25 , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_reduce_pd () { let a = _mm256_set_pd (0.25 , 0.50 , 0.75 , 1.0) ; let src = _mm256_set_pd (3. , 4. , 5. , 6.) ; let r = _mm256_mask_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b0110 , a) ; let e = _mm256_set_pd (3. , 0. , 0.25 , 6.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm256_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_reduce_pd () { let a = _mm256_set_pd (0.25 , 0.50 , 0.75 , 1.0) ; let r = _mm256_maskz_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (0b0110 , a) ; let e = _mm256_set_pd (0. , 0. , 0.25 , 0.) ; assert_eq_m256d (r , e) ; }
}

macro_rules! test_mm512_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_reduce_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm512_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm512_set_pd (0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_mask_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_reduce_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let src = _mm512_set_pd (3. , 4. , 5. , 6. , 7. , 8. , 9. , 10.) ; let r = _mm512_mask_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b01101001 , a) ; let e = _mm512_set_pd (3. , 0. , 0.25 , 6. , 0.25 , 8. , 9. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_maskz_reduce_pd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_reduce_pd in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_reduce_pd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_reduce_pd () { let a = _mm512_set_pd (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm512_maskz_reduce_pd :: < { 16 | _MM_FROUND_TO_ZERO } > (0b01101001 , a) ; let e = _mm512_set_pd (0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0.) ; assert_eq_m512d (r , e) ; }
}

macro_rules! test_mm512_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_reduce_round_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let r = _mm512_reduce_round_ps :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (a) ; let e = _mm512_set_ps (0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_reduce_round_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let src = _mm512_set_ps (5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. , 17. , 18. , 19. , 20. ,) ; let r = _mm512_mask_reduce_round_ps :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (src , 0b0110100100111100 , a ,) ; let e = _mm512_set_ps (5. , 0. , 0.25 , 8. , 0.25 , 10. , 11. , 0. , 13. , 14. , 0.25 , 0. , 0.25 , 0. , 19. , 20. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_reduce_round_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_reduce_round_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_reduce_round_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_reduce_round_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let r = _mm512_maskz_reduce_round_ps :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (0b0110100100111100 , a ,) ; let e = _mm512_set_ps (0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0. , 0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_reduce_ps () { let a = _mm_set_ps (0.25 , 0.50 , 0.75 , 1.0) ; let r = _mm_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm_set_ps (0.25 , 0. , 0.25 , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_reduce_ps () { let a = _mm_set_ps (0.25 , 0.50 , 0.75 , 1.0) ; let src = _mm_set_ps (2. , 3. , 4. , 5.) ; let r = _mm_mask_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b0110 , a) ; let e = _mm_set_ps (2. , 0. , 0.25 , 5.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_maskz_reduce_ps () { let a = _mm_set_ps (0.25 , 0.50 , 0.75 , 1.0) ; let r = _mm_maskz_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (0b0110 , a) ; let e = _mm_set_ps (0. , 0. , 0.25 , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_reduce_ps () { let a = _mm256_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm256_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm256_set_ps (0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_reduce_ps () { let a = _mm256_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let src = _mm256_set_ps (3. , 4. , 5. , 6. , 7. , 8. , 9. , 10.) ; let r = _mm256_mask_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b01101001 , a) ; let e = _mm256_set_ps (3. , 0. , 0.25 , 6. , 0.25 , 8. , 9. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm256_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_maskz_reduce_ps () { let a = _mm256_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0) ; let r = _mm256_maskz_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (0b01101001 , a) ; let e = _mm256_set_ps (0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm512_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_reduce_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let r = _mm512_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (a) ; let e = _mm512_set_ps (0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. , 0.25 , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_mask_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_reduce_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let src = _mm512_set_ps (5. , 6. , 7. , 8. , 9. , 10. , 11. , 12. , 13. , 14. , 15. , 16. , 17. , 18. , 19. , 20. ,) ; let r = _mm512_mask_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (src , 0b0110100100111100 , a) ; let e = _mm512_set_ps (5. , 0. , 0.25 , 8. , 0.25 , 10. , 11. , 0. , 13. , 14. , 0.25 , 0. , 0.25 , 0. , 19. , 20. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm512_maskz_reduce_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_reduce_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_reduce_ps_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_maskz_reduce_ps () { let a = _mm512_set_ps (0.25 , 0.50 , 0.75 , 1.0 , 1.25 , 1.50 , 1.75 , 2.0 , 2.25 , 2.50 , 2.75 , 3.0 , 3.25 , 3.50 , 3.75 , 4.0 ,) ; let r = _mm512_maskz_reduce_ps :: < { 16 | _MM_FROUND_TO_ZERO } > (0b0110100100111100 , a) ; let e = _mm512_set_ps (0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0. , 0. , 0. , 0.25 , 0. , 0.25 , 0. , 0. , 0. ,) ; assert_eq_m512 (r , e) ; }
}

macro_rules! test_mm_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_reduce_round_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let r = _mm_reduce_round_sd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm_set_pd (1. , 0.25) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_reduce_round_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let c = _mm_set_pd (3. , 4.) ; let r = _mm_mask_reduce_round_sd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (c , 0b0 , a , b ,) ; let e = _mm_set_pd (1. , 4.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_round_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_round_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_round_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_reduce_round_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let r = _mm_maskz_reduce_round_sd :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (0b0 , a , b) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_reduce_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let r = _mm_reduce_sd :: < { 16 | _MM_FROUND_TO_ZERO } > (a , b) ; let e = _mm_set_pd (1. , 0.25) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_mask_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_reduce_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let c = _mm_set_pd (3. , 4.) ; let r = _mm_mask_reduce_sd :: < { 16 | _MM_FROUND_TO_ZERO } > (c , 0b0 , a , b) ; let e = _mm_set_pd (1. , 4.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_sd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_sd in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_sd_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_reduce_sd () { let a = _mm_set_pd (1. , 2.) ; let b = _mm_set_sd (0.25) ; let r = _mm_maskz_reduce_sd :: < { 16 | _MM_FROUND_TO_ZERO } > (0b0 , a , b) ; let e = _mm_set_pd (1. , 0.) ; assert_eq_m128d (r , e) ; }
}

macro_rules! test_mm_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_reduce_round_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let r = _mm_reduce_round_ss :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (a , b) ; let e = _mm_set_ps (1. , 2. , 3. , 0.25) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_reduce_round_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let c = _mm_set_ps (5. , 6. , 7. , 8.) ; let r = _mm_mask_reduce_round_ss :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (c , 0b0 , a , b ,) ; let e = _mm_set_ps (1. , 2. , 3. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_round_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_round_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_round_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_reduce_round_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let r = _mm_maskz_reduce_round_ss :: < { 16 | _MM_FROUND_TO_ZERO } , _MM_FROUND_NO_EXC > (0b0 , a , b) ; let e = _mm_set_ps (1. , 2. , 3. , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_reduce_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_reduce_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let r = _mm_reduce_ss :: < { 16 | _MM_FROUND_TO_ZERO } > (a , b) ; let e = _mm_set_ps (1. , 2. , 3. , 0.25) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_mask_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_reduce_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_reduce_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let c = _mm_set_ps (5. , 6. , 7. , 8.) ; let r = _mm_mask_reduce_ss :: < { 16 | _MM_FROUND_TO_ZERO } > (c , 0b0 , a , b) ; let e = _mm_set_ps (1. , 2. , 3. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_maskz_reduce_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_reduce_ss in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_reduce_ss_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_maskz_reduce_ss () { let a = _mm_set_ps (1. , 2. , 3. , 4.) ; let b = _mm_set_ss (0.25) ; let r = _mm_maskz_reduce_ss :: < { 16 | _MM_FROUND_TO_ZERO } > (0b0 , a , b) ; let e = _mm_set_ps (1. , 2. , 3. , 0.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_fpclass_pd_mask () { let a = _mm_set_pd (1. , f64 :: INFINITY) ; let r = _mm_fpclass_pd_mask :: < 0x18 > (a) ; let e = 0b01 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_fpclass_pd_mask () { let a = _mm_set_pd (1. , f64 :: INFINITY) ; let r = _mm_mask_fpclass_pd_mask :: < 0x18 > (0b10 , a) ; let e = 0b00 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_fpclass_pd_mask () { let a = _mm256_set_pd (1. , f64 :: INFINITY , f64 :: NEG_INFINITY , 0.0) ; let r = _mm256_fpclass_pd_mask :: < 0x18 > (a) ; let e = 0b0110 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_fpclass_pd_mask () { let a = _mm256_set_pd (1. , f64 :: INFINITY , f64 :: NEG_INFINITY , 0.0) ; let r = _mm256_mask_fpclass_pd_mask :: < 0x18 > (0b1010 , a) ; let e = 0b0010 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_fpclass_pd_mask () { let a = _mm512_set_pd (1. , f64 :: INFINITY , f64 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f64 :: NAN , 1.0e-308 ,) ; let r = _mm512_fpclass_pd_mask :: < 0x18 > (a) ; let e = 0b01100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_mask_fpclass_pd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fpclass_pd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fpclass_pd_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_fpclass_pd_mask () { let a = _mm512_set_pd (1. , f64 :: INFINITY , f64 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f64 :: NAN , 1.0e-308 ,) ; let r = _mm512_mask_fpclass_pd_mask :: < 0x18 > (0b10101010 , a) ; let e = 0b00100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_fpclass_ps_mask () { let a = _mm_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0) ; let r = _mm_fpclass_ps_mask :: < 0x18 > (a) ; let e = 0b0110 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm_mask_fpclass_ps_mask () { let a = _mm_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0) ; let r = _mm_mask_fpclass_ps_mask :: < 0x18 > (0b1010 , a) ; let e = 0b0010 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_fpclass_ps_mask () { let a = _mm256_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f32 :: NAN , 1.0e-38 ,) ; let r = _mm256_fpclass_ps_mask :: < 0x18 > (a) ; let e = 0b01100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq,avx512vl")] unsafe fn test_mm256_mask_fpclass_ps_mask () { let a = _mm256_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f32 :: NAN , 1.0e-38 ,) ; let r = _mm256_mask_fpclass_ps_mask :: < 0x18 > (0b10101010 , a) ; let e = 0b00100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_fpclass_ps_mask () { let a = _mm512_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f32 :: NAN , 1.0e-38 , - 1. , f32 :: NEG_INFINITY , f32 :: INFINITY , - 0.0 , 0.0 , 2.0 , f32 :: NAN , - 1.0e-38 ,) ; let r = _mm512_fpclass_ps_mask :: < 0x18 > (a) ; let e = 0b0110000001100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm512_mask_fpclass_ps_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_fpclass_ps_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_fpclass_ps_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm512_mask_fpclass_ps_mask () { let a = _mm512_set_ps (1. , f32 :: INFINITY , f32 :: NEG_INFINITY , 0.0 , - 0.0 , - 2.0 , f32 :: NAN , 1.0e-38 , - 1. , f32 :: NEG_INFINITY , f32 :: INFINITY , - 0.0 , 0.0 , 2.0 , f32 :: NAN , - 1.0e-38 ,) ; let r = _mm512_mask_fpclass_ps_mask :: < 0x18 > (0b1010101010101010 , a) ; let e = 0b0010000000100000 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_fpclass_sd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fpclass_sd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fpclass_sd_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_fpclass_sd_mask () { let a = _mm_set_pd (1. , f64 :: INFINITY) ; let r = _mm_fpclass_sd_mask :: < 0x18 > (a) ; let e = 0b1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_fpclass_sd_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fpclass_sd_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fpclass_sd_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_fpclass_sd_mask () { let a = _mm_set_sd (f64 :: INFINITY) ; let r = _mm_mask_fpclass_sd_mask :: < 0x18 > (0b0 , a) ; let e = 0b0 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_fpclass_ss_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_fpclass_ss_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_fpclass_ss_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_fpclass_ss_mask () { let a = _mm_set_ss (f32 :: INFINITY) ; let r = _mm_fpclass_ss_mask :: < 0x18 > (a) ; let e = 0b1 ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm_mask_fpclass_ss_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_fpclass_ss_mask in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_fpclass_ss_mask_introspect!();
    # [simd_test (enable = "avx512dq")] unsafe fn test_mm_mask_fpclass_ss_mask () { let a = _mm_set_ss (f32 :: INFINITY) ; let r = _mm_mask_fpclass_ss_mask :: < 0x18 > (0b0 , a) ; let e = 0b0 ; assert_eq ! (r , e) ; }
} 
            }}