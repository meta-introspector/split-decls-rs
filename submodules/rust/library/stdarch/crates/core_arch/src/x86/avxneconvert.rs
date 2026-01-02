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
mkuse!{use crate :: arch :: asm ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{#[cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_bcstnebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_bcstnebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_bcstnebf16_ps_introspect!();
    #[doc = " Convert scalar BF16 (16-bit) floating point element stored at memory locations starting at location"] #[doc = " a to single precision (32-bit) floating-point, broadcast it to packed single precision (32-bit)"] #[doc = " floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_bcstnebf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vbcstnebf162ps))] #[unstable (feature = "stdarch_x86_avx512_bf16" , issue = "127356")] pub unsafe fn _mm_bcstnebf16_ps (a : * const bf16) -> __m128 { bcstnebf162ps_128 (a) }
}

macro_rules! _mm256_bcstnebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_bcstnebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_bcstnebf16_ps_introspect!();
    #[doc = " Convert scalar BF16 (16-bit) floating point element stored at memory locations starting at location"] #[doc = " a to single precision (32-bit) floating-point, broadcast it to packed single precision (32-bit) floating-point"] #[doc = " elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_bcstnebf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vbcstnebf162ps))] #[unstable (feature = "stdarch_x86_avx512_bf16" , issue = "127356")] pub unsafe fn _mm256_bcstnebf16_ps (a : * const bf16) -> __m256 { bcstnebf162ps_256 (a) }
}

macro_rules! _mm_bcstnesh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_bcstnesh_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_bcstnesh_ps_introspect!();
    #[doc = " Convert scalar half-precision (16-bit) floating-point element stored at memory locations starting"] #[doc = " at location a to a single-precision (32-bit) floating-point, broadcast it to packed single-precision"] #[doc = " (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_bcstnesh_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vbcstnesh2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm_bcstnesh_ps (a : * const f16) -> __m128 { bcstnesh2ps_128 (a) }
}

macro_rules! _mm256_bcstnesh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_bcstnesh_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_bcstnesh_ps_introspect!();
    #[doc = " Convert scalar half-precision (16-bit) floating-point element stored at memory locations starting"] #[doc = " at location a to a single-precision (32-bit) floating-point, broadcast it to packed single-precision"] #[doc = " (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_bcstnesh_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vbcstnesh2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm256_bcstnesh_ps (a : * const f16) -> __m256 { bcstnesh2ps_256 (a) }
}

macro_rules! _mm_cvtneebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtneebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtneebf16_ps_introspect!();
    #[doc = " Convert packed BF16 (16-bit) floating-point even-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtneebf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneebf162ps))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _mm_cvtneebf16_ps (a : * const __m128bh) -> __m128 { transmute (cvtneebf162ps_128 (a)) }
}

macro_rules! _mm256_cvtneebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtneebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtneebf16_ps_introspect!();
    #[doc = " Convert packed BF16 (16-bit) floating-point even-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtneebf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneebf162ps))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _mm256_cvtneebf16_ps (a : * const __m256bh) -> __m256 { transmute (cvtneebf162ps_256 (a)) }
}

macro_rules! _mm_cvtneeph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtneeph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtneeph_ps_introspect!();
    #[doc = " Convert packed half-precision (16-bit) floating-point even-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtneeph_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneeph2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm_cvtneeph_ps (a : * const __m128h) -> __m128 { transmute (cvtneeph2ps_128 (a)) }
}

macro_rules! _mm256_cvtneeph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtneeph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtneeph_ps_introspect!();
    #[doc = " Convert packed half-precision (16-bit) floating-point even-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtneeph_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneeph2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm256_cvtneeph_ps (a : * const __m256h) -> __m256 { transmute (cvtneeph2ps_256 (a)) }
}

macro_rules! _mm_cvtneobf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtneobf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtneobf16_ps_introspect!();
    #[doc = " Convert packed BF16 (16-bit) floating-point odd-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtneobf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneobf162ps))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _mm_cvtneobf16_ps (a : * const __m128bh) -> __m128 { transmute (cvtneobf162ps_128 (a)) }
}

macro_rules! _mm256_cvtneobf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtneobf16_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtneobf16_ps_introspect!();
    #[doc = " Convert packed BF16 (16-bit) floating-point odd-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtneobf16_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneobf162ps))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub unsafe fn _mm256_cvtneobf16_ps (a : * const __m256bh) -> __m256 { transmute (cvtneobf162ps_256 (a)) }
}

macro_rules! _mm_cvtneoph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtneoph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtneoph_ps_introspect!();
    #[doc = " Convert packed half-precision (16-bit) floating-point odd-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtneoph_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneoph2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm_cvtneoph_ps (a : * const __m128h) -> __m128 { transmute (cvtneoph2ps_128 (a)) }
}

macro_rules! _mm256_cvtneoph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtneoph_ps in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtneoph_ps_introspect!();
    #[doc = " Convert packed half-precision (16-bit) floating-point odd-indexed elements stored at memory locations starting at"] #[doc = " location a to single precision (32-bit) floating-point elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtneoph_ps)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneoph2ps))] #[unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub unsafe fn _mm256_cvtneoph_ps (a : * const __m256h) -> __m256 { transmute (cvtneoph2ps_256 (a)) }
}

macro_rules! _mm_cvtneps_avx_pbh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_cvtneps_avx_pbh in module {}", module_path!());
    };
}

mkfn!{
    _mm_cvtneps_avx_pbh_introspect!();
    #[doc = " Convert packed single precision (32-bit) floating-point elements in a to packed BF16 (16-bit) floating-point"] #[doc = " elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtneps_avx_pbh)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneps2bf16))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm_cvtneps_avx_pbh (a : __m128) -> __m128bh { unsafe { let mut dst : __m128bh ; asm ! ("{{vex}}vcvtneps2bf16 {dst},{src}" , dst = lateout (xmm_reg) dst , src = in (xmm_reg) a , options (pure , nomem , nostack , preserves_flags)) ; dst } }
}

macro_rules! _mm256_cvtneps_avx_pbh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_cvtneps_avx_pbh in module {}", module_path!());
    };
}

mkfn!{
    _mm256_cvtneps_avx_pbh_introspect!();
    #[doc = " Convert packed single precision (32-bit) floating-point elements in a to packed BF16 (16-bit) floating-point"] #[doc = " elements, and store the results in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cvtneps_avx_pbh)"] #[inline] #[target_feature (enable = "avxneconvert")] #[cfg_attr (test , assert_instr (vcvtneps2bf16))] #[stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub fn _mm256_cvtneps_avx_pbh (a : __m256) -> __m128bh { unsafe { let mut dst : __m128bh ; asm ! ("{{vex}}vcvtneps2bf16 {dst},{src}" , dst = lateout (xmm_reg) dst , src = in (ymm_reg) a , options (pure , nomem , nostack , preserves_flags)) ; dst } }
}
mkitem!{#[allow (improper_ctypes)] unsafe extern "C" { #[link_name = "llvm.x86.vbcstnebf162ps128"] fn bcstnebf162ps_128 (a : * const bf16) -> __m128 ; #[link_name = "llvm.x86.vbcstnebf162ps256"] fn bcstnebf162ps_256 (a : * const bf16) -> __m256 ; #[link_name = "llvm.x86.vbcstnesh2ps128"] fn bcstnesh2ps_128 (a : * const f16) -> __m128 ; #[link_name = "llvm.x86.vbcstnesh2ps256"] fn bcstnesh2ps_256 (a : * const f16) -> __m256 ; #[link_name = "llvm.x86.vcvtneebf162ps128"] fn cvtneebf162ps_128 (a : * const __m128bh) -> __m128 ; #[link_name = "llvm.x86.vcvtneebf162ps256"] fn cvtneebf162ps_256 (a : * const __m256bh) -> __m256 ; #[link_name = "llvm.x86.vcvtneeph2ps128"] fn cvtneeph2ps_128 (a : * const __m128h) -> __m128 ; #[link_name = "llvm.x86.vcvtneeph2ps256"] fn cvtneeph2ps_256 (a : * const __m256h) -> __m256 ; #[link_name = "llvm.x86.vcvtneobf162ps128"] fn cvtneobf162ps_128 (a : * const __m128bh) -> __m128 ; #[link_name = "llvm.x86.vcvtneobf162ps256"] fn cvtneobf162ps_256 (a : * const __m256bh) -> __m256 ; #[link_name = "llvm.x86.vcvtneoph2ps128"] fn cvtneoph2ps_128 (a : * const __m128h) -> __m128 ; #[link_name = "llvm.x86.vcvtneoph2ps256"] fn cvtneoph2ps_256 (a : * const __m256h) -> __m256 ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: simd :: { u16x4 , u16x8 } ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use crate :: mem :: transmute_copy ;}
mkuse!{use std :: ptr :: addr_of ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{const BF16_ONE : u16 = 0b0_01111111_0000000 ;}
mkitem!{const BF16_TWO : u16 = 0b0_10000000_0000000 ;}
mkitem!{const BF16_THREE : u16 = 0b0_10000000_1000000 ;}
mkitem!{const BF16_FOUR : u16 = 0b0_10000001_0000000 ;}
mkitem!{const BF16_FIVE : u16 = 0b0_10000001_0100000 ;}
mkitem!{const BF16_SIX : u16 = 0b0_10000001_1000000 ;}
mkitem!{const BF16_SEVEN : u16 = 0b0_10000001_1100000 ;}
mkitem!{const BF16_EIGHT : u16 = 0b0_10000010_0000000 ;}

macro_rules! test_mm_bcstnebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_bcstnebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_bcstnebf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_bcstnebf16_ps () { let a = bf16 :: from_bits (BF16_ONE) ; let r = _mm_bcstnebf16_ps (addr_of ! (a)) ; let e = _mm_set_ps (1. , 1. , 1. , 1.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_bcstnebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_bcstnebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_bcstnebf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_bcstnebf16_ps () { let a = bf16 :: from_bits (BF16_ONE) ; let r = _mm256_bcstnebf16_ps (addr_of ! (a)) ; let e = _mm256_set_ps (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_bcstnesh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_bcstnesh_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_bcstnesh_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_bcstnesh_ps () { let a = 1.0_f16 ; let r = _mm_bcstnesh_ps (addr_of ! (a)) ; let e = _mm_set_ps (1. , 1. , 1. , 1.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_bcstnesh_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_bcstnesh_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_bcstnesh_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_bcstnesh_ps () { let a = 1.0_f16 ; let r = _mm256_bcstnesh_ps (addr_of ! (a)) ; let e = _mm256_set_ps (1. , 1. , 1. , 1. , 1. , 1. , 1. , 1.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtneebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtneebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtneebf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_cvtneebf16_ps () { let a = __m128bh ([BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT ,]) ; let r = _mm_cvtneebf16_ps (addr_of ! (a)) ; let e = _mm_setr_ps (1. , 3. , 5. , 7.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtneebf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtneebf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtneebf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_cvtneebf16_ps () { let a = __m256bh ([BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT , BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT ,]) ; let r = _mm256_cvtneebf16_ps (addr_of ! (a)) ; let e = _mm256_setr_ps (1. , 3. , 5. , 7. , 1. , 3. , 5. , 7.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtneeph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtneeph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtneeph_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_cvtneeph_ps () { let a = __m128h ([1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0]) ; let r = _mm_cvtneeph_ps (addr_of ! (a)) ; let e = _mm_setr_ps (1. , 3. , 5. , 7.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtneeph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtneeph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtneeph_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_cvtneeph_ps () { let a = __m256h ([1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0 , 9.0 , 10.0 , 11.0 , 12.0 , 13.0 , 14.0 , 15.0 , 16.0 ,]) ; let r = _mm256_cvtneeph_ps (addr_of ! (a)) ; let e = _mm256_setr_ps (1. , 3. , 5. , 7. , 9. , 11. , 13. , 15.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtneobf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtneobf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtneobf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_cvtneobf16_ps () { let a = __m128bh ([BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT ,]) ; let r = _mm_cvtneobf16_ps (addr_of ! (a)) ; let e = _mm_setr_ps (2. , 4. , 6. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtneobf16_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtneobf16_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtneobf16_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_cvtneobf16_ps () { let a = __m256bh ([BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT , BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT ,]) ; let r = _mm256_cvtneobf16_ps (addr_of ! (a)) ; let e = _mm256_setr_ps (2. , 4. , 6. , 8. , 2. , 4. , 6. , 8.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtneoph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtneoph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtneoph_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_cvtneoph_ps () { let a = __m128h ([1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0]) ; let r = _mm_cvtneoph_ps (addr_of ! (a)) ; let e = _mm_setr_ps (2. , 4. , 6. , 8.) ; assert_eq_m128 (r , e) ; }
}

macro_rules! test_mm256_cvtneoph_ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtneoph_ps in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtneoph_ps_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_cvtneoph_ps () { let a = __m256h ([1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0 , 9.0 , 10.0 , 11.0 , 12.0 , 13.0 , 14.0 , 15.0 , 16.0 ,]) ; let r = _mm256_cvtneoph_ps (addr_of ! (a)) ; let e = _mm256_setr_ps (2. , 4. , 6. , 8. , 10. , 12. , 14. , 16.) ; assert_eq_m256 (r , e) ; }
}

macro_rules! test_mm_cvtneps_avx_pbh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_cvtneps_avx_pbh in module {}", module_path!());
    };
}

mkfn!{
    test_mm_cvtneps_avx_pbh_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm_cvtneps_avx_pbh () { let a = _mm_setr_ps (1. , 2. , 3. , 4.) ; let r : u16x4 = transmute_copy (& _mm_cvtneps_avx_pbh (a)) ; let e = u16x4 :: new (BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR) ; assert_eq ! (r , e) ; }
}

macro_rules! test_mm256_cvtneps_avx_pbh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_cvtneps_avx_pbh in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_cvtneps_avx_pbh_introspect!();
    #[simd_test (enable = "avxneconvert")] unsafe fn test_mm256_cvtneps_avx_pbh () { let a = _mm256_setr_ps (1. , 2. , 3. , 4. , 5. , 6. , 7. , 8.) ; let r : u16x8 = transmute (_mm256_cvtneps_avx_pbh (a)) ; let e = u16x8 :: new (BF16_ONE , BF16_TWO , BF16_THREE , BF16_FOUR , BF16_FIVE , BF16_SIX , BF16_SEVEN , BF16_EIGHT ,) ; assert_eq ! (r , e) ; }
} 
            }}