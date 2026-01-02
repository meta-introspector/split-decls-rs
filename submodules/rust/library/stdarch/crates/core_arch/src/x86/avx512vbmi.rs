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

macro_rules! _mm512_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_permutex2var_epi8&expand=4262)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm512_permutex2var_epi8 (a : __m512i , idx : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpermi2b (a . as_i8x64 () , idx . as_i8x64 () , b . as_i8x64 ())) } }
}

macro_rules! _mm512_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_permutex2var_epi8&expand=4259)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermt2b))] pub fn _mm512_mask_permutex2var_epi8 (a : __m512i , k : __mmask64 , idx : __m512i , b : __m512i ,) -> __m512i { unsafe { let permute = _mm512_permutex2var_epi8 (a , idx , b) . as_i8x64 () ; transmute (simd_select_bitmask (k , permute , a . as_i8x64 ())) } }
}

macro_rules! _mm512_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_permutex2var_epi8&expand=4261)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm512_maskz_permutex2var_epi8 (k : __mmask64 , a : __m512i , idx : __m512i , b : __m512i ,) -> __m512i { unsafe { let permute = _mm512_permutex2var_epi8 (a , idx , b) . as_i8x64 () ; transmute (simd_select_bitmask (k , permute , i8x64 :: ZERO)) } }
}

macro_rules! _mm512_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask2_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask2_permutex2var_epi8&expand=4260)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermi2b))] pub fn _mm512_mask2_permutex2var_epi8 (a : __m512i , idx : __m512i , k : __mmask64 , b : __m512i ,) -> __m512i { unsafe { let permute = _mm512_permutex2var_epi8 (a , idx , b) . as_i8x64 () ; transmute (simd_select_bitmask (k , permute , idx . as_i8x64 ())) } }
}

macro_rules! _mm256_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_permutex2var_epi8&expand=4258)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm256_permutex2var_epi8 (a : __m256i , idx : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpermi2b256 (a . as_i8x32 () , idx . as_i8x32 () , b . as_i8x32 ())) } }
}

macro_rules! _mm256_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_permutex2var_epi8&expand=4255)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermt2b))] pub fn _mm256_mask_permutex2var_epi8 (a : __m256i , k : __mmask32 , idx : __m256i , b : __m256i ,) -> __m256i { unsafe { let permute = _mm256_permutex2var_epi8 (a , idx , b) . as_i8x32 () ; transmute (simd_select_bitmask (k , permute , a . as_i8x32 ())) } }
}

macro_rules! _mm256_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_permutex2var_epi8&expand=4257)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm256_maskz_permutex2var_epi8 (k : __mmask32 , a : __m256i , idx : __m256i , b : __m256i ,) -> __m256i { unsafe { let permute = _mm256_permutex2var_epi8 (a , idx , b) . as_i8x32 () ; transmute (simd_select_bitmask (k , permute , i8x32 :: ZERO)) } }
}

macro_rules! _mm256_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask2_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask2_permutex2var_epi8&expand=4256)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermi2b))] pub fn _mm256_mask2_permutex2var_epi8 (a : __m256i , idx : __m256i , k : __mmask32 , b : __m256i ,) -> __m256i { unsafe { let permute = _mm256_permutex2var_epi8 (a , idx , b) . as_i8x32 () ; transmute (simd_select_bitmask (k , permute , idx . as_i8x32 ())) } }
}

macro_rules! _mm_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_permutex2var_epi8&expand=4254)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm_permutex2var_epi8 (a : __m128i , idx : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpermi2b128 (a . as_i8x16 () , idx . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_permutex2var_epi8&expand=4251)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermt2b))] pub fn _mm_mask_permutex2var_epi8 (a : __m128i , k : __mmask16 , idx : __m128i , b : __m128i) -> __m128i { unsafe { let permute = _mm_permutex2var_epi8 (a , idx , b) . as_i8x16 () ; transmute (simd_select_bitmask (k , permute , a . as_i8x16 ())) } }
}

macro_rules! _mm_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_permutex2var_epi8&expand=4253)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vperm))] pub fn _mm_maskz_permutex2var_epi8 (k : __mmask16 , a : __m128i , idx : __m128i , b : __m128i) -> __m128i { unsafe { let permute = _mm_permutex2var_epi8 (a , idx , b) . as_i8x16 () ; transmute (simd_select_bitmask (k , permute , i8x16 :: ZERO)) } }
}

macro_rules! _mm_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask2_permutex2var_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a and b across lanes using the corresponding selector and index in idx, and store the results in dst using writemask k (elements are copied from a when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask2_permutex2var_epi8&expand=4252)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermi2b))] pub fn _mm_mask2_permutex2var_epi8 (a : __m128i , idx : __m128i , k : __mmask16 , b : __m128i) -> __m128i { unsafe { let permute = _mm_permutex2var_epi8 (a , idx , b) . as_i8x16 () ; transmute (simd_select_bitmask (k , permute , idx . as_i8x16 ())) } }
}

macro_rules! _mm512_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_permutexvar_epi8&expand=4316)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm512_permutexvar_epi8 (idx : __m512i , a : __m512i) -> __m512i { unsafe { transmute (vpermb (a . as_i8x64 () , idx . as_i8x64 ())) } }
}

macro_rules! _mm512_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_permutexvar_epi8&expand=4314)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm512_mask_permutexvar_epi8 (src : __m512i , k : __mmask64 , idx : __m512i , a : __m512i ,) -> __m512i { unsafe { let permute = _mm512_permutexvar_epi8 (idx , a) . as_i8x64 () ; transmute (simd_select_bitmask (k , permute , src . as_i8x64 ())) } }
}

macro_rules! _mm512_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_permutexvar_epi8&expand=4315)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm512_maskz_permutexvar_epi8 (k : __mmask64 , idx : __m512i , a : __m512i) -> __m512i { unsafe { let permute = _mm512_permutexvar_epi8 (idx , a) . as_i8x64 () ; transmute (simd_select_bitmask (k , permute , i8x64 :: ZERO)) } }
}

macro_rules! _mm256_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_permutexvar_epi8&expand=4313)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm256_permutexvar_epi8 (idx : __m256i , a : __m256i) -> __m256i { unsafe { transmute (vpermb256 (a . as_i8x32 () , idx . as_i8x32 ())) } }
}

macro_rules! _mm256_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_permutexvar_epi8&expand=4311)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm256_mask_permutexvar_epi8 (src : __m256i , k : __mmask32 , idx : __m256i , a : __m256i ,) -> __m256i { unsafe { let permute = _mm256_permutexvar_epi8 (idx , a) . as_i8x32 () ; transmute (simd_select_bitmask (k , permute , src . as_i8x32 ())) } }
}

macro_rules! _mm256_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_permutexvar_epi8&expand=4312)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm256_maskz_permutexvar_epi8 (k : __mmask32 , idx : __m256i , a : __m256i) -> __m256i { unsafe { let permute = _mm256_permutexvar_epi8 (idx , a) . as_i8x32 () ; transmute (simd_select_bitmask (k , permute , i8x32 :: ZERO)) } }
}

macro_rules! _mm_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_permutexvar_epi8&expand=4310)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm_permutexvar_epi8 (idx : __m128i , a : __m128i) -> __m128i { unsafe { transmute (vpermb128 (a . as_i8x16 () , idx . as_i8x16 ())) } }
}

macro_rules! _mm_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_permutexvar_epi8&expand=4308)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm_mask_permutexvar_epi8 (src : __m128i , k : __mmask16 , idx : __m128i , a : __m128i) -> __m128i { unsafe { let permute = _mm_permutexvar_epi8 (idx , a) . as_i8x16 () ; transmute (simd_select_bitmask (k , permute , src . as_i8x16 ())) } }
}

macro_rules! _mm_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_permutexvar_epi8_introspect!();
    #[doc = " Shuffle 8-bit integers in a across lanes using the corresponding index in idx, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_permutexvar_epi8&expand=4309)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpermb))] pub fn _mm_maskz_permutexvar_epi8 (k : __mmask16 , idx : __m128i , a : __m128i) -> __m128i { unsafe { let permute = _mm_permutexvar_epi8 (idx , a) . as_i8x16 () ; transmute (simd_select_bitmask (k , permute , i8x16 :: ZERO)) } }
}

macro_rules! _mm512_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_multishift_epi64_epi8&expand=4026)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm512_multishift_epi64_epi8 (a : __m512i , b : __m512i) -> __m512i { unsafe { transmute (vpmultishiftqb (a . as_i8x64 () , b . as_i8x64 ())) } }
}

macro_rules! _mm512_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_multishift_epi64_epi8&expand=4024)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm512_mask_multishift_epi64_epi8 (src : __m512i , k : __mmask64 , a : __m512i , b : __m512i ,) -> __m512i { unsafe { let multishift = _mm512_multishift_epi64_epi8 (a , b) . as_i8x64 () ; transmute (simd_select_bitmask (k , multishift , src . as_i8x64 ())) } }
}

macro_rules! _mm512_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_multishift_epi64_epi8&expand=4025)"] #[inline] #[target_feature (enable = "avx512vbmi")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm512_maskz_multishift_epi64_epi8 (k : __mmask64 , a : __m512i , b : __m512i) -> __m512i { unsafe { let multishift = _mm512_multishift_epi64_epi8 (a , b) . as_i8x64 () ; transmute (simd_select_bitmask (k , multishift , i8x64 :: ZERO)) } }
}

macro_rules! _mm256_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_multishift_epi64_epi8&expand=4023)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm256_multishift_epi64_epi8 (a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vpmultishiftqb256 (a . as_i8x32 () , b . as_i8x32 ())) } }
}

macro_rules! _mm256_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_multishift_epi64_epi8&expand=4021)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm256_mask_multishift_epi64_epi8 (src : __m256i , k : __mmask32 , a : __m256i , b : __m256i ,) -> __m256i { unsafe { let multishift = _mm256_multishift_epi64_epi8 (a , b) . as_i8x32 () ; transmute (simd_select_bitmask (k , multishift , src . as_i8x32 ())) } }
}

macro_rules! _mm256_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_multishift_epi64_epi8&expand=4022)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm256_maskz_multishift_epi64_epi8 (k : __mmask32 , a : __m256i , b : __m256i) -> __m256i { unsafe { let multishift = _mm256_multishift_epi64_epi8 (a , b) . as_i8x32 () ; transmute (simd_select_bitmask (k , multishift , i8x32 :: ZERO)) } }
}

macro_rules! _mm_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/IntrinsicsGuide/#text=_mm_multishift_epi64_epi8&expand=4020)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm_multishift_epi64_epi8 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vpmultishiftqb128 (a . as_i8x16 () , b . as_i8x16 ())) } }
}

macro_rules! _mm_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_multishift_epi64_epi8&expand=4018)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm_mask_multishift_epi64_epi8 (src : __m128i , k : __mmask16 , a : __m128i , b : __m128i ,) -> __m128i { unsafe { let multishift = _mm_multishift_epi64_epi8 (a , b) . as_i8x16 () ; transmute (simd_select_bitmask (k , multishift , src . as_i8x16 ())) } }
}

macro_rules! _mm_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_multishift_epi64_epi8_introspect!();
    #[doc = " For each 64-bit element in b, select 8 unaligned bytes using a byte-granular shift control within the corresponding 64-bit element of a, and store the 8 assembled bytes to the corresponding 64-bit element of dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_multishift_epi64_epi8&expand=4019)"] #[inline] #[target_feature (enable = "avx512vbmi,avx512vl")] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] #[cfg_attr (test , assert_instr (vpmultishiftqb))] pub fn _mm_maskz_multishift_epi64_epi8 (k : __mmask16 , a : __m128i , b : __m128i) -> __m128i { unsafe { let multishift = _mm_multishift_epi64_epi8 (a , b) . as_i8x16 () ; transmute (simd_select_bitmask (k , multishift , i8x16 :: ZERO)) } }
}
mkitem!{#[allow (improper_ctypes)] unsafe extern "C" { #[link_name = "llvm.x86.avx512.vpermi2var.qi.512"] fn vpermi2b (a : i8x64 , idx : i8x64 , b : i8x64) -> i8x64 ; #[link_name = "llvm.x86.avx512.vpermi2var.qi.256"] fn vpermi2b256 (a : i8x32 , idx : i8x32 , b : i8x32) -> i8x32 ; #[link_name = "llvm.x86.avx512.vpermi2var.qi.128"] fn vpermi2b128 (a : i8x16 , idx : i8x16 , b : i8x16) -> i8x16 ; #[link_name = "llvm.x86.avx512.permvar.qi.512"] fn vpermb (a : i8x64 , idx : i8x64) -> i8x64 ; #[link_name = "llvm.x86.avx512.permvar.qi.256"] fn vpermb256 (a : i8x32 , idx : i8x32) -> i8x32 ; #[link_name = "llvm.x86.avx512.permvar.qi.128"] fn vpermb128 (a : i8x16 , idx : i8x16) -> i8x16 ; #[link_name = "llvm.x86.avx512.pmultishift.qb.512"] fn vpmultishiftqb (a : i8x64 , b : i8x64) -> i8x64 ; #[link_name = "llvm.x86.avx512.pmultishift.qb.256"] fn vpmultishiftqb256 (a : i8x32 , b : i8x32) -> i8x32 ; #[link_name = "llvm.x86.avx512.pmultishift.qb.128"] fn vpmultishiftqb128 (a : i8x16 , b : i8x16) -> i8x16 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_mm512_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; #[rustfmt :: skip] let idx = _mm512_set_epi8 (1 , 1 << 6 , 2 , 1 << 6 , 3 , 1 << 6 , 4 , 1 << 6 , 5 , 1 << 6 , 6 , 1 << 6 , 7 , 1 << 6 , 8 , 1 << 6 , 9 , 1 << 6 , 10 , 1 << 6 , 11 , 1 << 6 , 12 , 1 << 6 , 13 , 1 << 6 , 14 , 1 << 6 , 15 , 1 << 6 , 16 , 1 << 6 , 17 , 1 << 6 , 18 , 1 << 6 , 19 , 1 << 6 , 20 , 1 << 6 , 21 , 1 << 6 , 22 , 1 << 6 , 23 , 1 << 6 , 24 , 1 << 6 , 25 , 1 << 6 , 26 , 1 << 6 , 27 , 1 << 6 , 28 , 1 << 6 , 29 , 1 << 6 , 30 , 1 << 6 , 31 , 1 << 6 , 32 , 1 << 6) ; let b = _mm512_set1_epi8 (100) ; let r = _mm512_permutex2var_epi8 (a , idx , b) ; #[rustfmt :: skip] let e = _mm512_set_epi8 (62 , 100 , 61 , 100 , 60 , 100 , 59 , 100 , 58 , 100 , 57 , 100 , 56 , 100 , 55 , 100 , 54 , 100 , 53 , 100 , 52 , 100 , 51 , 100 , 50 , 100 , 49 , 100 , 48 , 100 , 47 , 100 , 46 , 100 , 45 , 100 , 44 , 100 , 43 , 100 , 42 , 100 , 41 , 100 , 40 , 100 , 39 , 100 , 38 , 100 , 37 , 100 , 36 , 100 , 35 , 100 , 34 , 100 , 33 , 100 , 32 , 100 , 31 , 100 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_mask_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; #[rustfmt :: skip] let idx = _mm512_set_epi8 (1 , 1 << 6 , 2 , 1 << 6 , 3 , 1 << 6 , 4 , 1 << 6 , 5 , 1 << 6 , 6 , 1 << 6 , 7 , 1 << 6 , 8 , 1 << 6 , 9 , 1 << 6 , 10 , 1 << 6 , 11 , 1 << 6 , 12 , 1 << 6 , 13 , 1 << 6 , 14 , 1 << 6 , 15 , 1 << 6 , 16 , 1 << 6 , 17 , 1 << 6 , 18 , 1 << 6 , 19 , 1 << 6 , 20 , 1 << 6 , 21 , 1 << 6 , 22 , 1 << 6 , 23 , 1 << 6 , 24 , 1 << 6 , 25 , 1 << 6 , 26 , 1 << 6 , 27 , 1 << 6 , 28 , 1 << 6 , 29 , 1 << 6 , 30 , 1 << 6 , 31 , 1 << 6 , 32 , 1 << 6) ; let b = _mm512_set1_epi8 (100) ; let r = _mm512_mask_permutex2var_epi8 (a , 0 , idx , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_permutex2var_epi8 (a , 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , idx , b ,) ; #[rustfmt :: skip] let e = _mm512_set_epi8 (62 , 100 , 61 , 100 , 60 , 100 , 59 , 100 , 58 , 100 , 57 , 100 , 56 , 100 , 55 , 100 , 54 , 100 , 53 , 100 , 52 , 100 , 51 , 100 , 50 , 100 , 49 , 100 , 48 , 100 , 47 , 100 , 46 , 100 , 45 , 100 , 44 , 100 , 43 , 100 , 42 , 100 , 41 , 100 , 40 , 100 , 39 , 100 , 38 , 100 , 37 , 100 , 36 , 100 , 35 , 100 , 34 , 100 , 33 , 100 , 32 , 100 , 31 , 100 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_maskz_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; #[rustfmt :: skip] let idx = _mm512_set_epi8 (1 , 1 << 6 , 2 , 1 << 6 , 3 , 1 << 6 , 4 , 1 << 6 , 5 , 1 << 6 , 6 , 1 << 6 , 7 , 1 << 6 , 8 , 1 << 6 , 9 , 1 << 6 , 10 , 1 << 6 , 11 , 1 << 6 , 12 , 1 << 6 , 13 , 1 << 6 , 14 , 1 << 6 , 15 , 1 << 6 , 16 , 1 << 6 , 17 , 1 << 6 , 18 , 1 << 6 , 19 , 1 << 6 , 20 , 1 << 6 , 21 , 1 << 6 , 22 , 1 << 6 , 23 , 1 << 6 , 24 , 1 << 6 , 25 , 1 << 6 , 26 , 1 << 6 , 27 , 1 << 6 , 28 , 1 << 6 , 29 , 1 << 6 , 30 , 1 << 6 , 31 , 1 << 6 , 32 , 1 << 6) ; let b = _mm512_set1_epi8 (100) ; let r = _mm512_maskz_permutex2var_epi8 (0 , a , idx , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_permutex2var_epi8 (0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , a , idx , b ,) ; #[rustfmt :: skip] let e = _mm512_set_epi8 (62 , 100 , 61 , 100 , 60 , 100 , 59 , 100 , 58 , 100 , 57 , 100 , 56 , 100 , 55 , 100 , 54 , 100 , 53 , 100 , 52 , 100 , 51 , 100 , 50 , 100 , 49 , 100 , 48 , 100 , 47 , 100 , 46 , 100 , 45 , 100 , 44 , 100 , 43 , 100 , 42 , 100 , 41 , 100 , 40 , 100 , 39 , 100 , 38 , 100 , 37 , 100 , 36 , 100 , 35 , 100 , 34 , 100 , 33 , 100 , 32 , 100 , 31 , 100 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask2_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_mask2_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; #[rustfmt :: skip] let idx = _mm512_set_epi8 (1 , 1 << 6 , 2 , 1 << 6 , 3 , 1 << 6 , 4 , 1 << 6 , 5 , 1 << 6 , 6 , 1 << 6 , 7 , 1 << 6 , 8 , 1 << 6 , 9 , 1 << 6 , 10 , 1 << 6 , 11 , 1 << 6 , 12 , 1 << 6 , 13 , 1 << 6 , 14 , 1 << 6 , 15 , 1 << 6 , 16 , 1 << 6 , 17 , 1 << 6 , 18 , 1 << 6 , 19 , 1 << 6 , 20 , 1 << 6 , 21 , 1 << 6 , 22 , 1 << 6 , 23 , 1 << 6 , 24 , 1 << 6 , 25 , 1 << 6 , 26 , 1 << 6 , 27 , 1 << 6 , 28 , 1 << 6 , 29 , 1 << 6 , 30 , 1 << 6 , 31 , 1 << 6 , 32 , 1 << 6) ; let b = _mm512_set1_epi8 (100) ; let r = _mm512_mask2_permutex2var_epi8 (a , idx , 0 , b) ; assert_eq_m512i (r , idx) ; let r = _mm512_mask2_permutex2var_epi8 (a , idx , 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , b ,) ; #[rustfmt :: skip] let e = _mm512_set_epi8 (62 , 100 , 61 , 100 , 60 , 100 , 59 , 100 , 58 , 100 , 57 , 100 , 56 , 100 , 55 , 100 , 54 , 100 , 53 , 100 , 52 , 100 , 51 , 100 , 50 , 100 , 49 , 100 , 48 , 100 , 47 , 100 , 46 , 100 , 45 , 100 , 44 , 100 , 43 , 100 , 42 , 100 , 41 , 100 , 40 , 100 , 39 , 100 , 38 , 100 , 37 , 100 , 36 , 100 , 35 , 100 , 34 , 100 , 33 , 100 , 32 , 100 , 31 , 100 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; #[rustfmt :: skip] let idx = _mm256_set_epi8 (1 , 1 << 5 , 2 , 1 << 5 , 3 , 1 << 5 , 4 , 1 << 5 , 5 , 1 << 5 , 6 , 1 << 5 , 7 , 1 << 5 , 8 , 1 << 5 , 9 , 1 << 5 , 10 , 1 << 5 , 11 , 1 << 5 , 12 , 1 << 5 , 13 , 1 << 5 , 14 , 1 << 5 , 15 , 1 << 5 , 16 , 1 << 5) ; let b = _mm256_set1_epi8 (100) ; let r = _mm256_permutex2var_epi8 (a , idx , b) ; #[rustfmt :: skip] let e = _mm256_set_epi8 (30 , 100 , 29 , 100 , 28 , 100 , 27 , 100 , 26 , 100 , 25 , 100 , 24 , 100 , 23 , 100 , 22 , 100 , 21 , 100 , 20 , 100 , 19 , 100 , 18 , 100 , 17 , 100 , 16 , 100 , 15 , 100 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_mask_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; #[rustfmt :: skip] let idx = _mm256_set_epi8 (1 , 1 << 5 , 2 , 1 << 5 , 3 , 1 << 5 , 4 , 1 << 5 , 5 , 1 << 5 , 6 , 1 << 5 , 7 , 1 << 5 , 8 , 1 << 5 , 9 , 1 << 5 , 10 , 1 << 5 , 11 , 1 << 5 , 12 , 1 << 5 , 13 , 1 << 5 , 14 , 1 << 5 , 15 , 1 << 5 , 16 , 1 << 5) ; let b = _mm256_set1_epi8 (100) ; let r = _mm256_mask_permutex2var_epi8 (a , 0 , idx , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_permutex2var_epi8 (a , 0b11111111_11111111_11111111_11111111 , idx , b) ; #[rustfmt :: skip] let e = _mm256_set_epi8 (30 , 100 , 29 , 100 , 28 , 100 , 27 , 100 , 26 , 100 , 25 , 100 , 24 , 100 , 23 , 100 , 22 , 100 , 21 , 100 , 20 , 100 , 19 , 100 , 18 , 100 , 17 , 100 , 16 , 100 , 15 , 100 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_maskz_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; #[rustfmt :: skip] let idx = _mm256_set_epi8 (1 , 1 << 5 , 2 , 1 << 5 , 3 , 1 << 5 , 4 , 1 << 5 , 5 , 1 << 5 , 6 , 1 << 5 , 7 , 1 << 5 , 8 , 1 << 5 , 9 , 1 << 5 , 10 , 1 << 5 , 11 , 1 << 5 , 12 , 1 << 5 , 13 , 1 << 5 , 14 , 1 << 5 , 15 , 1 << 5 , 16 , 1 << 5) ; let b = _mm256_set1_epi8 (100) ; let r = _mm256_maskz_permutex2var_epi8 (0 , a , idx , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_permutex2var_epi8 (0b11111111_11111111_11111111_11111111 , a , idx , b) ; #[rustfmt :: skip] let e = _mm256_set_epi8 (30 , 100 , 29 , 100 , 28 , 100 , 27 , 100 , 26 , 100 , 25 , 100 , 24 , 100 , 23 , 100 , 22 , 100 , 21 , 100 , 20 , 100 , 19 , 100 , 18 , 100 , 17 , 100 , 16 , 100 , 15 , 100 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask2_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_mask2_permutex2var_epi8 () { #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; #[rustfmt :: skip] let idx = _mm256_set_epi8 (1 , 1 << 5 , 2 , 1 << 5 , 3 , 1 << 5 , 4 , 1 << 5 , 5 , 1 << 5 , 6 , 1 << 5 , 7 , 1 << 5 , 8 , 1 << 5 , 9 , 1 << 5 , 10 , 1 << 5 , 11 , 1 << 5 , 12 , 1 << 5 , 13 , 1 << 5 , 14 , 1 << 5 , 15 , 1 << 5 , 16 , 1 << 5) ; let b = _mm256_set1_epi8 (100) ; let r = _mm256_mask2_permutex2var_epi8 (a , idx , 0 , b) ; assert_eq_m256i (r , idx) ; let r = _mm256_mask2_permutex2var_epi8 (a , idx , 0b11111111_11111111_11111111_11111111 , b) ; #[rustfmt :: skip] let e = _mm256_set_epi8 (30 , 100 , 29 , 100 , 28 , 100 , 27 , 100 , 26 , 100 , 25 , 100 , 24 , 100 , 23 , 100 , 22 , 100 , 21 , 100 , 20 , 100 , 19 , 100 , 18 , 100 , 17 , 100 , 16 , 100 , 15 , 100 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_permutex2var_epi8 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; #[rustfmt :: skip] let idx = _mm_set_epi8 (1 , 1 << 4 , 2 , 1 << 4 , 3 , 1 << 4 , 4 , 1 << 4 , 5 , 1 << 4 , 6 , 1 << 4 , 7 , 1 << 4 , 8 , 1 << 4) ; let b = _mm_set1_epi8 (100) ; let r = _mm_permutex2var_epi8 (a , idx , b) ; let e = _mm_set_epi8 (14 , 100 , 13 , 100 , 12 , 100 , 11 , 100 , 10 , 100 , 9 , 100 , 8 , 100 , 7 , 100 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_mask_permutex2var_epi8 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; #[rustfmt :: skip] let idx = _mm_set_epi8 (1 , 1 << 4 , 2 , 1 << 4 , 3 , 1 << 4 , 4 , 1 << 4 , 5 , 1 << 4 , 6 , 1 << 4 , 7 , 1 << 4 , 8 , 1 << 4) ; let b = _mm_set1_epi8 (100) ; let r = _mm_mask_permutex2var_epi8 (a , 0 , idx , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_permutex2var_epi8 (a , 0b11111111_11111111 , idx , b) ; let e = _mm_set_epi8 (14 , 100 , 13 , 100 , 12 , 100 , 11 , 100 , 10 , 100 , 9 , 100 , 8 , 100 , 7 , 100 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_maskz_permutex2var_epi8 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; #[rustfmt :: skip] let idx = _mm_set_epi8 (1 , 1 << 4 , 2 , 1 << 4 , 3 , 1 << 4 , 4 , 1 << 4 , 5 , 1 << 4 , 6 , 1 << 4 , 7 , 1 << 4 , 8 , 1 << 4) ; let b = _mm_set1_epi8 (100) ; let r = _mm_maskz_permutex2var_epi8 (0 , a , idx , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_permutex2var_epi8 (0b11111111_11111111 , a , idx , b) ; let e = _mm_set_epi8 (14 , 100 , 13 , 100 , 12 , 100 , 11 , 100 , 10 , 100 , 9 , 100 , 8 , 100 , 7 , 100 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask2_permutex2var_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask2_permutex2var_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask2_permutex2var_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_mask2_permutex2var_epi8 () { let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; #[rustfmt :: skip] let idx = _mm_set_epi8 (1 , 1 << 4 , 2 , 1 << 4 , 3 , 1 << 4 , 4 , 1 << 4 , 5 , 1 << 4 , 6 , 1 << 4 , 7 , 1 << 4 , 8 , 1 << 4) ; let b = _mm_set1_epi8 (100) ; let r = _mm_mask2_permutex2var_epi8 (a , idx , 0 , b) ; assert_eq_m128i (r , idx) ; let r = _mm_mask2_permutex2var_epi8 (a , idx , 0b11111111_11111111 , b) ; let e = _mm_set_epi8 (14 , 100 , 13 , 100 , 12 , 100 , 11 , 100 , 10 , 100 , 9 , 100 , 8 , 100 , 7 , 100 ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_permutexvar_epi8 () { let idx = _mm512_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; let r = _mm512_permutexvar_epi8 (idx , a) ; let e = _mm512_set1_epi8 (62) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_mask_permutexvar_epi8 () { let idx = _mm512_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; let r = _mm512_mask_permutexvar_epi8 (a , 0 , idx , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_permutexvar_epi8 (a , 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , idx , a ,) ; let e = _mm512_set1_epi8 (62) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_maskz_permutexvar_epi8 () { let idx = _mm512_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm512_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 , 32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 , 40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 , 48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 , 56 , 57 , 58 , 59 , 60 , 61 , 62 , 63) ; let r = _mm512_maskz_permutexvar_epi8 (0 , idx , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_permutexvar_epi8 (0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , idx , a ,) ; let e = _mm512_set1_epi8 (62) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_permutexvar_epi8 () { let idx = _mm256_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; let r = _mm256_permutexvar_epi8 (idx , a) ; let e = _mm256_set1_epi8 (30) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_mask_permutexvar_epi8 () { let idx = _mm256_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; let r = _mm256_mask_permutexvar_epi8 (a , 0 , idx , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_permutexvar_epi8 (a , 0b11111111_11111111_11111111_11111111 , idx , a) ; let e = _mm256_set1_epi8 (30) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_maskz_permutexvar_epi8 () { let idx = _mm256_set1_epi8 (1) ; #[rustfmt :: skip] let a = _mm256_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31) ; let r = _mm256_maskz_permutexvar_epi8 (0 , idx , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_permutexvar_epi8 (0b11111111_11111111_11111111_11111111 , idx , a) ; let e = _mm256_set1_epi8 (30) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_permutexvar_epi8 () { let idx = _mm_set1_epi8 (1) ; let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_permutexvar_epi8 (idx , a) ; let e = _mm_set1_epi8 (14) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_mask_permutexvar_epi8 () { let idx = _mm_set1_epi8 (1) ; let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_mask_permutexvar_epi8 (a , 0 , idx , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_permutexvar_epi8 (a , 0b11111111_11111111 , idx , a) ; let e = _mm_set1_epi8 (14) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_permutexvar_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_permutexvar_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_permutexvar_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_maskz_permutexvar_epi8 () { let idx = _mm_set1_epi8 (1) ; let a = _mm_set_epi8 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) ; let r = _mm_maskz_permutexvar_epi8 (0 , idx , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_permutexvar_epi8 (0b11111111_11111111 , idx , a) ; let e = _mm_set1_epi8 (14) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_multishift_epi64_epi8 () { let a = _mm512_set1_epi8 (1) ; let b = _mm512_set1_epi8 (1) ; let r = _mm512_multishift_epi64_epi8 (a , b) ; let e = _mm512_set1_epi8 (1 << 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_mask_multishift_epi64_epi8 () { let a = _mm512_set1_epi8 (1) ; let b = _mm512_set1_epi8 (1) ; let r = _mm512_mask_multishift_epi64_epi8 (a , 0 , a , b) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_multishift_epi64_epi8 (a , 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , a , b ,) ; let e = _mm512_set1_epi8 (1 << 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi")] unsafe fn test_mm512_maskz_multishift_epi64_epi8 () { let a = _mm512_set1_epi8 (1) ; let b = _mm512_set1_epi8 (1) ; let r = _mm512_maskz_multishift_epi64_epi8 (0 , a , b) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_multishift_epi64_epi8 (0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111 , a , b ,) ; let e = _mm512_set1_epi8 (1 << 7) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_multishift_epi64_epi8 () { let a = _mm256_set1_epi8 (1) ; let b = _mm256_set1_epi8 (1) ; let r = _mm256_multishift_epi64_epi8 (a , b) ; let e = _mm256_set1_epi8 (1 << 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_mask_multishift_epi64_epi8 () { let a = _mm256_set1_epi8 (1) ; let b = _mm256_set1_epi8 (1) ; let r = _mm256_mask_multishift_epi64_epi8 (a , 0 , a , b) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_multishift_epi64_epi8 (a , 0b11111111_11111111_11111111_11111111 , a , b) ; let e = _mm256_set1_epi8 (1 << 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm256_maskz_multishift_epi64_epi8 () { let a = _mm256_set1_epi8 (1) ; let b = _mm256_set1_epi8 (1) ; let r = _mm256_maskz_multishift_epi64_epi8 (0 , a , b) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_multishift_epi64_epi8 (0b11111111_11111111_11111111_11111111 , a , b) ; let e = _mm256_set1_epi8 (1 << 7) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_multishift_epi64_epi8 () { let a = _mm_set1_epi8 (1) ; let b = _mm_set1_epi8 (1) ; let r = _mm_multishift_epi64_epi8 (a , b) ; let e = _mm_set1_epi8 (1 << 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_mask_multishift_epi64_epi8 () { let a = _mm_set1_epi8 (1) ; let b = _mm_set1_epi8 (1) ; let r = _mm_mask_multishift_epi64_epi8 (a , 0 , a , b) ; assert_eq_m128i (r , a) ; let r = _mm_mask_multishift_epi64_epi8 (a , 0b11111111_11111111 , a , b) ; let e = _mm_set1_epi8 (1 << 7) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_multishift_epi64_epi8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_multishift_epi64_epi8 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_multishift_epi64_epi8_introspect!();
    #[simd_test (enable = "avx512vbmi,avx512vl")] unsafe fn test_mm_maskz_multishift_epi64_epi8 () { let a = _mm_set1_epi8 (1) ; let b = _mm_set1_epi8 (1) ; let r = _mm_maskz_multishift_epi64_epi8 (0 , a , b) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_multishift_epi64_epi8 (0b11111111_11111111 , a , b) ; let e = _mm_set1_epi8 (1 << 7) ; assert_eq_m128i (r , e) ; }
} 
            }}