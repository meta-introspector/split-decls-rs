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
mkuse!{use crate :: core_arch :: { simd :: * , x86 :: * } ;}
mkuse!{use crate :: intrinsics :: simd :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm512_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcastmw_epi32_introspect!();
    # [doc = " Broadcast the low 16-bits from input mask k to all 32-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcastmw_epi32&expand=553)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm512_broadcastmw_epi32 (k : __mmask16) -> __m512i { _mm512_set1_epi32 (k as i32) }
}

macro_rules! _mm256_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcastmw_epi32_introspect!();
    # [doc = " Broadcast the low 16-bits from input mask k to all 32-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcastmw_epi32&expand=552)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm256_broadcastmw_epi32 (k : __mmask16) -> __m256i { _mm256_set1_epi32 (k as i32) }
}

macro_rules! _mm_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_broadcastmw_epi32_introspect!();
    # [doc = " Broadcast the low 16-bits from input mask k to all 32-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_broadcastmw_epi32&expand=551)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm_broadcastmw_epi32 (k : __mmask16) -> __m128i { _mm_set1_epi32 (k as i32) }
}

macro_rules! _mm512_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_broadcastmb_epi64_introspect!();
    # [doc = " Broadcast the low 8-bits from input mask k to all 64-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_broadcastmb_epi64&expand=550)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm512_broadcastmb_epi64 (k : __mmask8) -> __m512i { _mm512_set1_epi64 (k as i64) }
}

macro_rules! _mm256_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_broadcastmb_epi64_introspect!();
    # [doc = " Broadcast the low 8-bits from input mask k to all 64-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_broadcastmb_epi64&expand=549)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm256_broadcastmb_epi64 (k : __mmask8) -> __m256i { _mm256_set1_epi64x (k as i64) }
}

macro_rules! _mm_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_broadcastmb_epi64_introspect!();
    # [doc = " Broadcast the low 8-bits from input mask k to all 64-bit elements of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_broadcastmb_epi64&expand=548)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpbroadcast))] pub fn _mm_broadcastmb_epi64 (k : __mmask8) -> __m128i { _mm_set1_epi64x (k as i64) }
}

macro_rules! _mm512_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_conflict_epi32&expand=1248)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm512_conflict_epi32 (a : __m512i) -> __m512i { unsafe { transmute (vpconflictd (a . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_conflict_epi32&expand=1249)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm512_mask_conflict_epi32 (src : __m512i , k : __mmask16 , a : __m512i) -> __m512i { unsafe { let conflict = _mm512_conflict_epi32 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , conflict , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_conflict_epi32&expand=1250)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm512_maskz_conflict_epi32 (k : __mmask16 , a : __m512i) -> __m512i { unsafe { let conflict = _mm512_conflict_epi32 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , conflict , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_conflict_epi32&expand=1245)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm256_conflict_epi32 (a : __m256i) -> __m256i { unsafe { transmute (vpconflictd256 (a . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_conflict_epi32&expand=1246)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm256_mask_conflict_epi32 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { let conflict = _mm256_conflict_epi32 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , conflict , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_conflict_epi32&expand=1247)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm256_maskz_conflict_epi32 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { let conflict = _mm256_conflict_epi32 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , conflict , i32x8 :: ZERO)) } }
}

macro_rules! _mm_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_conflict_epi32&expand=1242)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm_conflict_epi32 (a : __m128i) -> __m128i { unsafe { transmute (vpconflictd128 (a . as_i32x4 ())) } }
}

macro_rules! _mm_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_conflict_epi32&expand=1243)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm_mask_conflict_epi32 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { let conflict = _mm_conflict_epi32 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , conflict , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_conflict_epi32_introspect!();
    # [doc = " Test each 32-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_conflict_epi32&expand=1244)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictd))] pub fn _mm_maskz_conflict_epi32 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { let conflict = _mm_conflict_epi32 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , conflict , i32x4 :: ZERO)) } }
}

macro_rules! _mm512_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_conflict_epi64&expand=1257)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm512_conflict_epi64 (a : __m512i) -> __m512i { unsafe { transmute (vpconflictq (a . as_i64x8 ())) } }
}

macro_rules! _mm512_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_conflict_epi64&expand=1258)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm512_mask_conflict_epi64 (src : __m512i , k : __mmask8 , a : __m512i) -> __m512i { unsafe { let conflict = _mm512_conflict_epi64 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , conflict , src . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_conflict_epi64&expand=1259)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm512_maskz_conflict_epi64 (k : __mmask8 , a : __m512i) -> __m512i { unsafe { let conflict = _mm512_conflict_epi64 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , conflict , i64x8 :: ZERO)) } }
}

macro_rules! _mm256_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_conflict_epi64&expand=1254)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm256_conflict_epi64 (a : __m256i) -> __m256i { unsafe { transmute (vpconflictq256 (a . as_i64x4 ())) } }
}

macro_rules! _mm256_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_conflict_epi64&expand=1255)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm256_mask_conflict_epi64 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { let conflict = _mm256_conflict_epi64 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , conflict , src . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_conflict_epi64&expand=1256)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm256_maskz_conflict_epi64 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { let conflict = _mm256_conflict_epi64 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , conflict , i64x4 :: ZERO)) } }
}

macro_rules! _mm_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit. Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_conflict_epi64&expand=1251)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm_conflict_epi64 (a : __m128i) -> __m128i { unsafe { transmute (vpconflictq128 (a . as_i64x2 ())) } }
}

macro_rules! _mm_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using writemask k (elements are copied from src when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_conflict_epi64&expand=1252)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm_mask_conflict_epi64 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { let conflict = _mm_conflict_epi64 (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , conflict , src . as_i64x2 ())) } }
}

macro_rules! _mm_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_conflict_epi64_introspect!();
    # [doc = " Test each 64-bit element of a for equality with all other elements in a closer to the least significant bit using zeromask k (elements are zeroed out when the corresponding mask bit is not set). Each element's comparison forms a zero extended bit vector in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_conflict_epi64&expand=1253)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vpconflictq))] pub fn _mm_maskz_conflict_epi64 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { let conflict = _mm_conflict_epi64 (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , conflict , i64x2 :: ZERO)) } }
}

macro_rules! _mm512_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_lzcnt_epi32&expand=3491)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm512_lzcnt_epi32 (a : __m512i) -> __m512i { unsafe { transmute (simd_ctlz (a . as_i32x16 ())) } }
}

macro_rules! _mm512_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_lzcnt_epi32&expand=3492)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm512_mask_lzcnt_epi32 (src : __m512i , k : __mmask16 , a : __m512i) -> __m512i { unsafe { let zerocount = _mm512_lzcnt_epi32 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i32x16 ())) } }
}

macro_rules! _mm512_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_lzcnt_epi32&expand=3493)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm512_maskz_lzcnt_epi32 (k : __mmask16 , a : __m512i) -> __m512i { unsafe { let zerocount = _mm512_lzcnt_epi32 (a) . as_i32x16 () ; transmute (simd_select_bitmask (k , zerocount , i32x16 :: ZERO)) } }
}

macro_rules! _mm256_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_lzcnt_epi32&expand=3488)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm256_lzcnt_epi32 (a : __m256i) -> __m256i { unsafe { transmute (simd_ctlz (a . as_i32x8 ())) } }
}

macro_rules! _mm256_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_lzcnt_epi32&expand=3489)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm256_mask_lzcnt_epi32 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { let zerocount = _mm256_lzcnt_epi32 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i32x8 ())) } }
}

macro_rules! _mm256_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_lzcnt_epi32&expand=3490)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm256_maskz_lzcnt_epi32 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { let zerocount = _mm256_lzcnt_epi32 (a) . as_i32x8 () ; transmute (simd_select_bitmask (k , zerocount , i32x8 :: ZERO)) } }
}

macro_rules! _mm_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_lzcnt_epi32&expand=3485)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm_lzcnt_epi32 (a : __m128i) -> __m128i { unsafe { transmute (simd_ctlz (a . as_i32x4 ())) } }
}

macro_rules! _mm_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_lzcnt_epi32&expand=3486)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm_mask_lzcnt_epi32 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { let zerocount = _mm_lzcnt_epi32 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i32x4 ())) } }
}

macro_rules! _mm_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_lzcnt_epi32_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 32-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_lzcnt_epi32&expand=3487)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntd))] pub fn _mm_maskz_lzcnt_epi32 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { let zerocount = _mm_lzcnt_epi32 (a) . as_i32x4 () ; transmute (simd_select_bitmask (k , zerocount , i32x4 :: ZERO)) } }
}

macro_rules! _mm512_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_lzcnt_epi64&expand=3500)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm512_lzcnt_epi64 (a : __m512i) -> __m512i { unsafe { transmute (simd_ctlz (a . as_i64x8 ())) } }
}

macro_rules! _mm512_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_mask_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_mask_lzcnt_epi64&expand=3501)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm512_mask_lzcnt_epi64 (src : __m512i , k : __mmask8 , a : __m512i) -> __m512i { unsafe { let zerocount = _mm512_lzcnt_epi64 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i64x8 ())) } }
}

macro_rules! _mm512_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm512_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm512_maskz_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_maskz_lzcnt_epi64&expand=3502)"] # [inline] # [target_feature (enable = "avx512cd")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm512_maskz_lzcnt_epi64 (k : __mmask8 , a : __m512i) -> __m512i { unsafe { let zerocount = _mm512_lzcnt_epi64 (a) . as_i64x8 () ; transmute (simd_select_bitmask (k , zerocount , i64x8 :: ZERO)) } }
}

macro_rules! _mm256_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_lzcnt_epi64&expand=3497)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm256_lzcnt_epi64 (a : __m256i) -> __m256i { unsafe { transmute (simd_ctlz (a . as_i64x4 ())) } }
}

macro_rules! _mm256_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_mask_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mask_lzcnt_epi64&expand=3498)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm256_mask_lzcnt_epi64 (src : __m256i , k : __mmask8 , a : __m256i) -> __m256i { unsafe { let zerocount = _mm256_lzcnt_epi64 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i64x4 ())) } }
}

macro_rules! _mm256_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_maskz_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_maskz_lzcnt_epi64&expand=3499)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm256_maskz_lzcnt_epi64 (k : __mmask8 , a : __m256i) -> __m256i { unsafe { let zerocount = _mm256_lzcnt_epi64 (a) . as_i64x4 () ; transmute (simd_select_bitmask (k , zerocount , i64x4 :: ZERO)) } }
}

macro_rules! _mm_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_lzcnt_epi64&expand=3494)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm_lzcnt_epi64 (a : __m128i) -> __m128i { unsafe { transmute (simd_ctlz (a . as_i64x2 ())) } }
}

macro_rules! _mm_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_mask_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mask_lzcnt_epi64&expand=3495)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm_mask_lzcnt_epi64 (src : __m128i , k : __mmask8 , a : __m128i) -> __m128i { unsafe { let zerocount = _mm_lzcnt_epi64 (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , zerocount , src . as_i64x2 ())) } }
}

macro_rules! _mm_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm_maskz_lzcnt_epi64_introspect!();
    # [doc = " Counts the number of leading zero bits in each packed 64-bit integer in a, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_maskz_lzcnt_epi64&expand=3496)"] # [inline] # [target_feature (enable = "avx512cd,avx512vl")] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [cfg_attr (test , assert_instr (vplzcntq))] pub fn _mm_maskz_lzcnt_epi64 (k : __mmask8 , a : __m128i) -> __m128i { unsafe { let zerocount = _mm_lzcnt_epi64 (a) . as_i64x2 () ; transmute (simd_select_bitmask (k , zerocount , i64x2 :: ZERO)) } }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.avx512.conflict.d.512"] fn vpconflictd (a : i32x16) -> i32x16 ; # [link_name = "llvm.x86.avx512.conflict.d.256"] fn vpconflictd256 (a : i32x8) -> i32x8 ; # [link_name = "llvm.x86.avx512.conflict.d.128"] fn vpconflictd128 (a : i32x4) -> i32x4 ; # [link_name = "llvm.x86.avx512.conflict.q.512"] fn vpconflictq (a : i64x8) -> i64x8 ; # [link_name = "llvm.x86.avx512.conflict.q.256"] fn vpconflictq256 (a : i64x4) -> i64x4 ; # [link_name = "llvm.x86.avx512.conflict.q.128"] fn vpconflictq128 (a : i64x2) -> i64x2 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm512_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcastmw_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_broadcastmw_epi32 () { let a : __mmask16 = 2 ; let r = _mm512_broadcastmw_epi32 (a) ; let e = _mm512_set1_epi32 (2) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcastmw_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_broadcastmw_epi32 () { let a : __mmask16 = 2 ; let r = _mm256_broadcastmw_epi32 (a) ; let e = _mm256_set1_epi32 (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_broadcastmw_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_broadcastmw_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_broadcastmw_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_broadcastmw_epi32 () { let a : __mmask16 = 2 ; let r = _mm_broadcastmw_epi32 (a) ; let e = _mm_set1_epi32 (2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_broadcastmb_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_broadcastmb_epi64 () { let a : __mmask8 = 2 ; let r = _mm512_broadcastmb_epi64 (a) ; let e = _mm512_set1_epi64 (2) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_broadcastmb_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_broadcastmb_epi64 () { let a : __mmask8 = 2 ; let r = _mm256_broadcastmb_epi64 (a) ; let e = _mm256_set1_epi64x (2) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_broadcastmb_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_broadcastmb_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_broadcastmb_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_broadcastmb_epi64 () { let a : __mmask8 = 2 ; let r = _mm_broadcastmb_epi64 (a) ; let e = _mm_set1_epi64x (2) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_conflict_epi32 () { let a = _mm512_set1_epi32 (1) ; let r = _mm512_conflict_epi32 (a) ; let e = _mm512_set_epi32 (1 << 14 | 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_mask_conflict_epi32 () { let a = _mm512_set1_epi32 (1) ; let r = _mm512_mask_conflict_epi32 (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_conflict_epi32 (a , 0b11111111_11111111 , a) ; let e = _mm512_set_epi32 (1 << 14 | 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_maskz_conflict_epi32 () { let a = _mm512_set1_epi32 (1) ; let r = _mm512_maskz_conflict_epi32 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_conflict_epi32 (0b11111111_11111111 , a) ; let e = _mm512_set_epi32 (1 << 14 | 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 12 | 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 11 | 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 10 | 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 8 | 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 7 | 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_conflict_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_conflict_epi32 (a) ; let e = _mm256_set_epi32 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_mask_conflict_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_mask_conflict_epi32 (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_conflict_epi32 (a , 0b11111111 , a) ; let e = _mm256_set_epi32 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_maskz_conflict_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_maskz_conflict_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_conflict_epi32 (0b11111111 , a) ; let e = _mm256_set_epi32 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_conflict_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_conflict_epi32 (a) ; let e = _mm_set_epi32 (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_mask_conflict_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_mask_conflict_epi32 (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_conflict_epi32 (a , 0b00001111 , a) ; let e = _mm_set_epi32 (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_conflict_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_conflict_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_conflict_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_maskz_conflict_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_maskz_conflict_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_conflict_epi32 (0b00001111 , a) ; let e = _mm_set_epi32 (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_conflict_epi64 () { let a = _mm512_set1_epi64 (1) ; let r = _mm512_conflict_epi64 (a) ; let e = _mm512_set_epi64 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_mask_conflict_epi64 () { let a = _mm512_set1_epi64 (1) ; let r = _mm512_mask_conflict_epi64 (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_conflict_epi64 (a , 0b11111111 , a) ; let e = _mm512_set_epi64 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_maskz_conflict_epi64 () { let a = _mm512_set1_epi64 (1) ; let r = _mm512_maskz_conflict_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_conflict_epi64 (0b11111111 , a) ; let e = _mm512_set_epi64 (1 << 6 | 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 5 | 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 4 | 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 3 | 1 << 2 | 1 << 1 | 1 << 0 , 1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0 ,) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_conflict_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_conflict_epi64 (a) ; let e = _mm256_set_epi64x (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_mask_conflict_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_mask_conflict_epi64 (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_conflict_epi64 (a , 0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_maskz_conflict_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_conflict_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_conflict_epi64 (0b00001111 , a) ; let e = _mm256_set_epi64x (1 << 2 | 1 << 1 | 1 << 0 , 1 << 1 | 1 << 0 , 1 << 0 , 0) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_conflict_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_conflict_epi64 (a) ; let e = _mm_set_epi64x (1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_mask_conflict_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_mask_conflict_epi64 (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_conflict_epi64 (a , 0b00000011 , a) ; let e = _mm_set_epi64x (1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_conflict_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_conflict_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_conflict_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_maskz_conflict_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_maskz_conflict_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_conflict_epi64 (0b00000011 , a) ; let e = _mm_set_epi64x (1 << 0 , 0) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_lzcnt_epi32 () { let a = _mm512_set1_epi32 (1) ; let r = _mm512_lzcnt_epi32 (a) ; let e = _mm512_set1_epi32 (31) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_mask_lzcnt_epi32 () { let a = _mm512_set1_epi32 (1) ; let r = _mm512_mask_lzcnt_epi32 (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_lzcnt_epi32 (a , 0b11111111_11111111 , a) ; let e = _mm512_set1_epi32 (31) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_maskz_lzcnt_epi32 () { let a = _mm512_set1_epi32 (2) ; let r = _mm512_maskz_lzcnt_epi32 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_lzcnt_epi32 (0b11111111_11111111 , a) ; let e = _mm512_set1_epi32 (30) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_lzcnt_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_lzcnt_epi32 (a) ; let e = _mm256_set1_epi32 (31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_mask_lzcnt_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_mask_lzcnt_epi32 (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_lzcnt_epi32 (a , 0b11111111 , a) ; let e = _mm256_set1_epi32 (31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_maskz_lzcnt_epi32 () { let a = _mm256_set1_epi32 (1) ; let r = _mm256_maskz_lzcnt_epi32 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_lzcnt_epi32 (0b11111111 , a) ; let e = _mm256_set1_epi32 (31) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_lzcnt_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_lzcnt_epi32 (a) ; let e = _mm_set1_epi32 (31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_mask_lzcnt_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_mask_lzcnt_epi32 (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_lzcnt_epi32 (a , 0b00001111 , a) ; let e = _mm_set1_epi32 (31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_lzcnt_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_lzcnt_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_lzcnt_epi32_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_maskz_lzcnt_epi32 () { let a = _mm_set1_epi32 (1) ; let r = _mm_maskz_lzcnt_epi32 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_lzcnt_epi32 (0b00001111 , a) ; let e = _mm_set1_epi32 (31) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm512_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_lzcnt_epi64 () { let a = _mm512_set1_epi64 (1) ; let r = _mm512_lzcnt_epi64 (a) ; let e = _mm512_set1_epi64 (63) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_mask_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_mask_lzcnt_epi64 () { let a = _mm512_set1_epi64 (1) ; let r = _mm512_mask_lzcnt_epi64 (a , 0 , a) ; assert_eq_m512i (r , a) ; let r = _mm512_mask_lzcnt_epi64 (a , 0b11111111 , a) ; let e = _mm512_set1_epi64 (63) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm512_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm512_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm512_maskz_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd")] unsafe fn test_mm512_maskz_lzcnt_epi64 () { let a = _mm512_set1_epi64 (2) ; let r = _mm512_maskz_lzcnt_epi64 (0 , a) ; assert_eq_m512i (r , _mm512_setzero_si512 ()) ; let r = _mm512_maskz_lzcnt_epi64 (0b11111111 , a) ; let e = _mm512_set1_epi64 (62) ; assert_eq_m512i (r , e) ; }
}

macro_rules! test_mm256_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_lzcnt_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_lzcnt_epi64 (a) ; let e = _mm256_set1_epi64x (63) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_mask_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_mask_lzcnt_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_mask_lzcnt_epi64 (a , 0 , a) ; assert_eq_m256i (r , a) ; let r = _mm256_mask_lzcnt_epi64 (a , 0b00001111 , a) ; let e = _mm256_set1_epi64x (63) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_maskz_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm256_maskz_lzcnt_epi64 () { let a = _mm256_set1_epi64x (1) ; let r = _mm256_maskz_lzcnt_epi64 (0 , a) ; assert_eq_m256i (r , _mm256_setzero_si256 ()) ; let r = _mm256_maskz_lzcnt_epi64 (0b00001111 , a) ; let e = _mm256_set1_epi64x (63) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_lzcnt_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_lzcnt_epi64 (a) ; let e = _mm_set1_epi64x (63) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_mask_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_mask_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_mask_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_mask_lzcnt_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_mask_lzcnt_epi64 (a , 0 , a) ; assert_eq_m128i (r , a) ; let r = _mm_mask_lzcnt_epi64 (a , 0b00001111 , a) ; let e = _mm_set1_epi64x (63) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_maskz_lzcnt_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_maskz_lzcnt_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_maskz_lzcnt_epi64_introspect!();
    # [simd_test (enable = "avx512cd,avx512vl")] unsafe fn test_mm_maskz_lzcnt_epi64 () { let a = _mm_set1_epi64x (1) ; let r = _mm_maskz_lzcnt_epi64 (0 , a) ; assert_eq_m128i (r , _mm_setzero_si128 ()) ; let r = _mm_maskz_lzcnt_epi64 (0b00001111 , a) ; let e = _mm_set1_epi64x (63) ; assert_eq_m128i (r , e) ; }
} 
            }}