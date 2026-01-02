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
mkitem!{#[allow (improper_ctypes)] unsafe extern "C" { #[link_name = "llvm.x86.sha1msg1"] fn sha1msg1 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.sha1msg2"] fn sha1msg2 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.sha1nexte"] fn sha1nexte (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.sha1rnds4"] fn sha1rnds4 (a : i32x4 , b : i32x4 , c : i8) -> i32x4 ; #[link_name = "llvm.x86.sha256msg1"] fn sha256msg1 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.sha256msg2"] fn sha256msg2 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.sha256rnds2"] fn sha256rnds2 (a : i32x4 , b : i32x4 , k : i32x4) -> i32x4 ; #[link_name = "llvm.x86.vsha512msg1"] fn vsha512msg1 (a : i64x4 , b : i64x2) -> i64x4 ; #[link_name = "llvm.x86.vsha512msg2"] fn vsha512msg2 (a : i64x4 , b : i64x4) -> i64x4 ; #[link_name = "llvm.x86.vsha512rnds2"] fn vsha512rnds2 (a : i64x4 , b : i64x4 , k : i64x2) -> i64x4 ; #[link_name = "llvm.x86.vsm3msg1"] fn vsm3msg1 (a : i32x4 , b : i32x4 , c : i32x4) -> i32x4 ; #[link_name = "llvm.x86.vsm3msg2"] fn vsm3msg2 (a : i32x4 , b : i32x4 , c : i32x4) -> i32x4 ; #[link_name = "llvm.x86.vsm3rnds2"] fn vsm3rnds2 (a : i32x4 , b : i32x4 , c : i32x4 , d : i32) -> i32x4 ; #[link_name = "llvm.x86.vsm4key4128"] fn vsm4key4128 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.vsm4key4256"] fn vsm4key4256 (a : i32x8 , b : i32x8) -> i32x8 ; #[link_name = "llvm.x86.vsm4rnds4128"] fn vsm4rnds4128 (a : i32x4 , b : i32x4) -> i32x4 ; #[link_name = "llvm.x86.vsm4rnds4256"] fn vsm4rnds4256 (a : i32x8 , b : i32x8) -> i32x8 ; }}
mkuse!{#[cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! _mm_sha1msg1_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha1msg1_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha1msg1_epu32_introspect!();
    #[doc = " Performs an intermediate calculation for the next four SHA1 message values"] #[doc = " (unsigned 32-bit integers) using previous message values from `a` and `b`,"] #[doc = " and returning the result."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha1msg1_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha1msg1))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha1msg1_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (sha1msg1 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sha1msg2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha1msg2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha1msg2_epu32_introspect!();
    #[doc = " Performs the final calculation for the next four SHA1 message values"] #[doc = " (unsigned 32-bit integers) using the intermediate result in `a` and the"] #[doc = " previous message values in `b`, and returns the result."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha1msg2_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha1msg2))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha1msg2_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (sha1msg2 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sha1nexte_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha1nexte_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha1nexte_epu32_introspect!();
    #[doc = " Calculate SHA1 state variable E after four rounds of operation from the"] #[doc = " current SHA1 state variable `a`, add that value to the scheduled values"] #[doc = " (unsigned 32-bit integers) in `b`, and returns the result."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha1nexte_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha1nexte))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha1nexte_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (sha1nexte (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sha1rnds4_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha1rnds4_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha1rnds4_epu32_introspect!();
    #[doc = " Performs four rounds of SHA1 operation using an initial SHA1 state (A,B,C,D)"] #[doc = " from `a` and some pre-computed sum of the next 4 round message values"] #[doc = " (unsigned 32-bit integers), and state variable E from `b`, and return the"] #[doc = " updated SHA1 state (A,B,C,D). `FUNC` contains the logic functions and round"] #[doc = " constants."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha1rnds4_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha1rnds4 , FUNC = 0))] #[rustc_legacy_const_generics (2)] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha1rnds4_epu32 < const FUNC : i32 > (a : __m128i , b : __m128i) -> __m128i { static_assert_uimm_bits ! (FUNC , 2) ; unsafe { transmute (sha1rnds4 (a . as_i32x4 () , b . as_i32x4 () , FUNC as i8)) } }
}

macro_rules! _mm_sha256msg1_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha256msg1_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha256msg1_epu32_introspect!();
    #[doc = " Performs an intermediate calculation for the next four SHA256 message values"] #[doc = " (unsigned 32-bit integers) using previous message values from `a` and `b`,"] #[doc = " and return the result."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha256msg1_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha256msg1))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha256msg1_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (sha256msg1 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sha256msg2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha256msg2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha256msg2_epu32_introspect!();
    #[doc = " Performs the final calculation for the next four SHA256 message values"] #[doc = " (unsigned 32-bit integers) using previous message values from `a` and `b`,"] #[doc = " and return the result."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha256msg2_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha256msg2))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha256msg2_epu32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (sha256msg2 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm_sha256rnds2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sha256rnds2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sha256rnds2_epu32_introspect!();
    #[doc = " Performs 2 rounds of SHA256 operation using an initial SHA256 state"] #[doc = " (C,D,G,H) from `a`, an initial SHA256 state (A,B,E,F) from `b`, and a"] #[doc = " pre-computed sum of the next 2 round message values (unsigned 32-bit"] #[doc = " integers) and the corresponding round constants from `k`, and store the"] #[doc = " updated SHA256 state (A,B,E,F) in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sha256rnds2_epu32)"] #[inline] #[target_feature (enable = "sha")] #[cfg_attr (test , assert_instr (sha256rnds2))] #[stable (feature = "simd_x86" , since = "1.27.0")] pub fn _mm_sha256rnds2_epu32 (a : __m128i , b : __m128i , k : __m128i) -> __m128i { unsafe { transmute (sha256rnds2 (a . as_i32x4 () , b . as_i32x4 () , k . as_i32x4 ())) } }
}

macro_rules! _mm256_sha512msg1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_sha512msg1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_sha512msg1_epi64_introspect!();
    #[doc = " This intrinsic is one of the two SHA512 message scheduling instructions."] #[doc = " The intrinsic performs an intermediate calculation for the next four SHA512"] #[doc = " message qwords. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_sha512msg1_epi64)"] #[inline] #[target_feature (enable = "sha512,avx")] #[cfg_attr (test , assert_instr (vsha512msg1))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm256_sha512msg1_epi64 (a : __m256i , b : __m128i) -> __m256i { unsafe { transmute (vsha512msg1 (a . as_i64x4 () , b . as_i64x2 ())) } }
}

macro_rules! _mm256_sha512msg2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_sha512msg2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_sha512msg2_epi64_introspect!();
    #[doc = " This intrinsic is one of the two SHA512 message scheduling instructions."] #[doc = " The intrinsic performs the final calculation for the next four SHA512 message"] #[doc = " qwords. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_sha512msg2_epi64)"] #[inline] #[target_feature (enable = "sha512,avx")] #[cfg_attr (test , assert_instr (vsha512msg2))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm256_sha512msg2_epi64 (a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vsha512msg2 (a . as_i64x4 () , b . as_i64x4 ())) } }
}

macro_rules! _mm256_sha512rnds2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_sha512rnds2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_sha512rnds2_epi64_introspect!();
    #[doc = " This intrinsic performs two rounds of SHA512 operation using initial SHA512 state"] #[doc = " `(C,D,G,H)` from `a`, an initial SHA512 state `(A,B,E,F)` from `b`, and a"] #[doc = " pre-computed sum of the next two round message qwords and the corresponding"] #[doc = " round constants from `c` (only the two lower qwords of the third operand). The"] #[doc = " updated SHA512 state `(A,B,E,F)` is written to dst, and dst can be used as the"] #[doc = " updated state `(C,D,G,H)` in later rounds."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_sha512rnds2_epi64)"] #[inline] #[target_feature (enable = "sha512,avx")] #[cfg_attr (test , assert_instr (vsha512rnds2))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm256_sha512rnds2_epi64 (a : __m256i , b : __m256i , k : __m128i) -> __m256i { unsafe { transmute (vsha512rnds2 (a . as_i64x4 () , b . as_i64x4 () , k . as_i64x2 ())) } }
}

macro_rules! _mm_sm3msg1_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sm3msg1_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sm3msg1_epi32_introspect!();
    #[doc = " This is one of the two SM3 message scheduling intrinsics. The intrinsic performs"] #[doc = " an initial calculation for the next four SM3 message words. The calculated results"] #[doc = " are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sm3msg1_epi32)"] #[inline] #[target_feature (enable = "sm3,avx")] #[cfg_attr (test , assert_instr (vsm3msg1))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm_sm3msg1_epi32 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { transmute (vsm3msg1 (a . as_i32x4 () , b . as_i32x4 () , c . as_i32x4 ())) } }
}

macro_rules! _mm_sm3msg2_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sm3msg2_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sm3msg2_epi32_introspect!();
    #[doc = " This is one of the two SM3 message scheduling intrinsics. The intrinsic performs"] #[doc = " the final calculation for the next four SM3 message words. The calculated results"] #[doc = " are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sm3msg2_epi32)"] #[inline] #[target_feature (enable = "sm3,avx")] #[cfg_attr (test , assert_instr (vsm3msg2))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm_sm3msg2_epi32 (a : __m128i , b : __m128i , c : __m128i) -> __m128i { unsafe { transmute (vsm3msg2 (a . as_i32x4 () , b . as_i32x4 () , c . as_i32x4 ())) } }
}

macro_rules! _mm_sm3rnds2_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sm3rnds2_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sm3rnds2_epi32_introspect!();
    #[doc = " The intrinsic performs two rounds of SM3 operation using initial SM3 state `(C, D, G, H)`"] #[doc = " from `a`, an initial SM3 states `(A, B, E, F)` from `b` and a pre-computed words from the"] #[doc = " `c`. `a` with initial SM3 state of `(C, D, G, H)` assumes input of non-rotated left variables"] #[doc = " from previous state. The updated SM3 state `(A, B, E, F)` is written to `a`. The `imm8`"] #[doc = " should contain the even round number for the first of the two rounds computed by this instruction."] #[doc = " The computation masks the `imm8` value by ANDing it with `0x3E` so that only even round numbers"] #[doc = " from 0 through 62 are used for this operation. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sm3rnds2_epi32)"] #[inline] #[target_feature (enable = "sm3,avx")] #[cfg_attr (test , assert_instr (vsm3rnds2 , IMM8 = 0))] #[rustc_legacy_const_generics (3)] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm_sm3rnds2_epi32 < const IMM8 : i32 > (a : __m128i , b : __m128i , c : __m128i) -> __m128i { static_assert ! (IMM8 == (IMM8 & 0x3e) , "IMM8 must be an even number in the range `0..=62`") ; unsafe { transmute (vsm3rnds2 (a . as_i32x4 () , b . as_i32x4 () , c . as_i32x4 () , IMM8)) } }
}

macro_rules! _mm_sm4key4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sm4key4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sm4key4_epi32_introspect!();
    #[doc = " This intrinsic performs four rounds of SM4 key expansion. The intrinsic operates on independent"] #[doc = " 128-bit lanes. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sm4key4_epi32)"] #[inline] #[target_feature (enable = "sm4,avx")] #[cfg_attr (test , assert_instr (vsm4key4))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm_sm4key4_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vsm4key4128 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_sm4key4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_sm4key4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_sm4key4_epi32_introspect!();
    #[doc = " This intrinsic performs four rounds of SM4 key expansion. The intrinsic operates on independent"] #[doc = " 128-bit lanes. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_sm4key4_epi32)"] #[inline] #[target_feature (enable = "sm4,avx")] #[cfg_attr (test , assert_instr (vsm4key4))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm256_sm4key4_epi32 (a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vsm4key4256 (a . as_i32x8 () , b . as_i32x8 ())) } }
}

macro_rules! _mm_sm4rnds4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm_sm4rnds4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm_sm4rnds4_epi32_introspect!();
    #[doc = " This intrinsic performs four rounds of SM4 encryption. The intrinsic operates on independent"] #[doc = " 128-bit lanes. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_sm4rnds4_epi32)"] #[inline] #[target_feature (enable = "sm4,avx")] #[cfg_attr (test , assert_instr (vsm4rnds4))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm_sm4rnds4_epi32 (a : __m128i , b : __m128i) -> __m128i { unsafe { transmute (vsm4rnds4128 (a . as_i32x4 () , b . as_i32x4 ())) } }
}

macro_rules! _mm256_sm4rnds4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _mm256_sm4rnds4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    _mm256_sm4rnds4_epi32_introspect!();
    #[doc = " This intrinsic performs four rounds of SM4 encryption. The intrinsic operates on independent"] #[doc = " 128-bit lanes. The calculated results are stored in dst."] #[doc = ""] #[doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_sm4rnds4_epi32)"] #[inline] #[target_feature (enable = "sm4,avx")] #[cfg_attr (test , assert_instr (vsm4rnds4))] #[stable (feature = "sha512_sm_x86" , since = "1.89.0")] pub fn _mm256_sm4rnds4_epi32 (a : __m256i , b : __m256i) -> __m256i { unsafe { transmute (vsm4rnds4256 (a . as_i32x8 () , b . as_i32x8 ())) } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: { core_arch :: { simd :: * , x86 :: * } , hint :: black_box , } ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! test_mm_sha1msg1_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha1msg1_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha1msg1_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha1msg1_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0x98829f34f74ad457 , 0xda2b1a44d0b5ad3c) ; let r = _mm_sha1msg1_epu32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha1msg2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha1msg2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha1msg2_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha1msg2_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0xf714b202d863d47d , 0x90c30d946b3d3b35) ; let r = _mm_sha1msg2_epu32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha1nexte_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha1nexte_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha1nexte_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha1nexte_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0x2589d5be923f82a4 , 0x59f111f13956c25b) ; let r = _mm_sha1nexte_epu32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha1rnds4_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha1rnds4_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha1rnds4_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha1rnds4_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0x32b13cd8322f5268 , 0xc54420862bd9246f) ; let r = _mm_sha1rnds4_epu32 :: < 0 > (a , b) ; assert_eq_m128i (r , expected) ; let expected = _mm_set_epi64x (0x6d4c43e56a3c25d9 , 0xa7e00fb775cbd3fe) ; let r = _mm_sha1rnds4_epu32 :: < 1 > (a , b) ; assert_eq_m128i (r , expected) ; let expected = _mm_set_epi64x (0xb304e383c01222f4 , 0x66f6b3b1f89d8001) ; let r = _mm_sha1rnds4_epu32 :: < 2 > (a , b) ; assert_eq_m128i (r , expected) ; let expected = _mm_set_epi64x (0x8189b758bfabfa79 , 0xdb08f6e78cae098b) ; let r = _mm_sha1rnds4_epu32 :: < 3 > (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha256msg1_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha256msg1_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha256msg1_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha256msg1_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0xeb84973fd5cda67d , 0x2857b88f406b09ee) ; let r = _mm_sha256msg1_epu32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha256msg2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha256msg2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha256msg2_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha256msg2_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let expected = _mm_set_epi64x (0xb58777ce887fd851 , 0x15d1ec8b73ac8450) ; let r = _mm_sha256msg2_epu32 (a , b) ; assert_eq_m128i (r , expected) ; }
}

macro_rules! test_mm_sha256rnds2_epu32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sha256rnds2_epu32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sha256rnds2_epu32_introspect!();
    #[simd_test (enable = "sha")] #[allow (overflowing_literals)] unsafe fn test_mm_sha256rnds2_epu32 () { let a = _mm_set_epi64x (0xe9b5dba5b5c0fbcf , 0x71374491428a2f98) ; let b = _mm_set_epi64x (0xab1c5ed5923f82a4 , 0x59f111f13956c25b) ; let k = _mm_set_epi64x (0 , 0x12835b01d807aa98) ; let expected = _mm_set_epi64x (0xd3063037effb15ea , 0x187ee3db0d6d1d19) ; let r = _mm_sha256rnds2_epu32 (a , b , k) ; assert_eq_m128i (r , expected) ; }
}
mkitem!{static DATA_64 : [u64 ; 10] = [0x0011223344556677 , 0x8899aabbccddeeff , 0xffeeddccbbaa9988 , 0x7766554433221100 , 0x0123456789abcdef , 0xfedcba9876543210 , 0x02468ace13579bdf , 0xfdb97531eca86420 , 0x048c159d26ae37bf , 0xfb73ea62d951c840 ,] ;}

macro_rules! test_mm256_sha512msg1_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sha512msg1_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sha512msg1_epi64_introspect!();
    #[simd_test (enable = "sha512,avx")] unsafe fn test_mm256_sha512msg1_epi64 () { fn s0 (word : u64) -> u64 { word . rotate_right (1) ^ word . rotate_right (8) ^ (word >> 7) } let A = & DATA_64 [0 .. 4] ; let B = & DATA_64 [4 .. 6] ; let a = _mm256_loadu_si256 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let r = _mm256_sha512msg1_epi64 (a , b) ; let e = _mm256_setr_epi64x (A [0] . wrapping_add (s0 (A [1])) as _ , A [1] . wrapping_add (s0 (A [2])) as _ , A [2] . wrapping_add (s0 (A [3])) as _ , A [3] . wrapping_add (s0 (B [0])) as _ ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_sha512msg2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sha512msg2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sha512msg2_epi64_introspect!();
    #[simd_test (enable = "sha512,avx")] unsafe fn test_mm256_sha512msg2_epi64 () { fn s1 (word : u64) -> u64 { word . rotate_right (19) ^ word . rotate_right (61) ^ (word >> 6) } let A = & DATA_64 [0 .. 4] ; let B = & DATA_64 [4 .. 8] ; let a = _mm256_loadu_si256 (A . as_ptr () . cast ()) ; let b = _mm256_loadu_si256 (B . as_ptr () . cast ()) ; let r = _mm256_sha512msg2_epi64 (a , b) ; let e0 = A [0] . wrapping_add (s1 (B [2])) ; let e1 = A [1] . wrapping_add (s1 (B [3])) ; let e = _mm256_setr_epi64x (e0 as _ , e1 as _ , A [2] . wrapping_add (s1 (e0)) as _ , A [3] . wrapping_add (s1 (e1)) as _ ,) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm256_sha512rnds2_epi64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sha512rnds2_epi64 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sha512rnds2_epi64_introspect!();
    #[simd_test (enable = "sha512,avx")] unsafe fn test_mm256_sha512rnds2_epi64 () { fn cap_sigma0 (word : u64) -> u64 { word . rotate_right (28) ^ word . rotate_right (34) ^ word . rotate_right (39) } fn cap_sigma1 (word : u64) -> u64 { word . rotate_right (14) ^ word . rotate_right (18) ^ word . rotate_right (41) } fn maj (a : u64 , b : u64 , c : u64) -> u64 { (a & b) ^ (a & c) ^ (b & c) } fn ch (e : u64 , f : u64 , g : u64) -> u64 { (e & f) ^ (g & ! e) } let A = & DATA_64 [0 .. 4] ; let B = & DATA_64 [4 .. 8] ; let K = & DATA_64 [8 .. 10] ; let a = _mm256_loadu_si256 (A . as_ptr () . cast ()) ; let b = _mm256_loadu_si256 (B . as_ptr () . cast ()) ; let k = _mm_loadu_si128 (K . as_ptr () . cast ()) ; let r = _mm256_sha512rnds2_epi64 (a , b , k) ; let mut array = [B [3] , B [2] , A [3] , A [2] , B [1] , B [0] , A [1] , A [0]] ; for i in 0 .. 2 { let new_d = ch (array [4] , array [5] , array [6]) . wrapping_add (cap_sigma1 (array [4])) . wrapping_add (K [i]) . wrapping_add (array [7]) ; array [7] = new_d . wrapping_add (maj (array [0] , array [1] , array [2])) . wrapping_add (cap_sigma0 (array [0])) ; array [3] = new_d . wrapping_add (array [3]) ; array . rotate_right (1) ; } let e = _mm256_setr_epi64x (array [5] as _ , array [4] as _ , array [1] as _ , array [0] as _) ; assert_eq_m256i (r , e) ; }
}
mkitem!{static DATA_32 : [u32 ; 16] = [0x00112233 , 0x44556677 , 0x8899aabb , 0xccddeeff , 0xffeeddcc , 0xbbaa9988 , 0x77665544 , 0x33221100 , 0x01234567 , 0x89abcdef , 0xfedcba98 , 0x76543210 , 0x02468ace , 0x13579bdf , 0xfdb97531 , 0xeca86420 ,] ;}

macro_rules! test_mm_sm3msg1_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sm3msg1_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sm3msg1_epi32_introspect!();
    #[simd_test (enable = "sm3,avx")] unsafe fn test_mm_sm3msg1_epi32 () { fn p1 (x : u32) -> u32 { x ^ x . rotate_left (15) ^ x . rotate_left (23) } let A = & DATA_32 [0 .. 4] ; let B = & DATA_32 [4 .. 8] ; let C = & DATA_32 [8 .. 12] ; let a = _mm_loadu_si128 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let c = _mm_loadu_si128 (C . as_ptr () . cast ()) ; let r = _mm_sm3msg1_epi32 (a , b , c) ; let e = _mm_setr_epi32 (p1 (A [0] ^ C [0] ^ B [0] . rotate_left (15)) as _ , p1 (A [1] ^ C [1] ^ B [1] . rotate_left (15)) as _ , p1 (A [2] ^ C [2] ^ B [2] . rotate_left (15)) as _ , p1 (A [3] ^ C [3]) as _ ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_sm3msg2_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sm3msg2_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sm3msg2_epi32_introspect!();
    #[simd_test (enable = "sm3,avx")] unsafe fn test_mm_sm3msg2_epi32 () { let A = & DATA_32 [0 .. 4] ; let B = & DATA_32 [4 .. 8] ; let C = & DATA_32 [8 .. 12] ; let a = _mm_loadu_si128 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let c = _mm_loadu_si128 (C . as_ptr () . cast ()) ; let r = _mm_sm3msg2_epi32 (a , b , c) ; let e0 = B [0] . rotate_left (7) ^ C [0] ^ A [0] ; let e = _mm_setr_epi32 (e0 as _ , (B [1] . rotate_left (7) ^ C [1] ^ A [1]) as _ , (B [2] . rotate_left (7) ^ C [2] ^ A [2]) as _ , (B [3] . rotate_left (7) ^ C [3] ^ A [3] ^ e0 . rotate_left (6) ^ e0 . rotate_left (15) ^ e0 . rotate_left (30)) as _ ,) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm_sm3rnds2_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sm3rnds2_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sm3rnds2_epi32_introspect!();
    #[simd_test (enable = "sm3,avx")] unsafe fn test_mm_sm3rnds2_epi32 () { fn p0 (x : u32) -> u32 { x ^ x . rotate_left (9) ^ x . rotate_left (17) } fn ff (x : u32 , y : u32 , z : u32 , round : u32) -> u32 { if round < 16 { x ^ y ^ z } else { (x & y) | (x & z) | (y & z) } } fn gg (x : u32 , y : u32 , z : u32 , round : u32) -> u32 { if round < 16 { x ^ y ^ z } else { (x & y) | (! x & z) } } const ROUND : u32 = 30 ; let A = & DATA_32 [0 .. 4] ; let B = & DATA_32 [4 .. 8] ; let C = & DATA_32 [8 .. 12] ; let a = _mm_loadu_si128 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let c = _mm_loadu_si128 (C . as_ptr () . cast ()) ; let r = _mm_sm3rnds2_epi32 :: < { ROUND as i32 } > (a , b , c) ; let CONST : u32 = if ROUND < 16 { 0x79cc4519 } else { 0x7a879d8a } ; let mut array = [B [3] , B [2] , A [3] . rotate_left (9) , A [2] . rotate_left (9) , B [1] , B [0] , A [1] . rotate_left (19) , A [0] . rotate_left (19) ,] ; for i in 0 .. 2 { let s1 = array [0] . rotate_left (12) . wrapping_add (array [4]) . wrapping_add (CONST . rotate_left (ROUND as u32 + i as u32)) . rotate_left (7) ; let s2 = s1 ^ array [0] . rotate_left (12) ; let t1 = ff (array [0] , array [1] , array [2] , ROUND) . wrapping_add (array [3]) . wrapping_add (s2) . wrapping_add (C [i] ^ C [i + 2]) ; let t2 = gg (array [4] , array [5] , array [6] , ROUND) . wrapping_add (array [7]) . wrapping_add (s1) . wrapping_add (C [i]) ; array [3] = array [2] ; array [2] = array [1] . rotate_left (9) ; array [1] = array [0] ; array [0] = t1 ; array [7] = array [6] ; array [6] = array [5] . rotate_left (19) ; array [5] = array [4] ; array [4] = p0 (t2) ; } let e = _mm_setr_epi32 (array [5] as _ , array [4] as _ , array [1] as _ , array [0] as _) ; assert_eq_m128i (r , e) ; }
}

macro_rules! lower_t_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lower_t in module {}", module_path!());
    };
}

mkfn!{
    lower_t_introspect!();
    fn lower_t (x : u32) -> u32 { static SBOX : [u8 ; 256] = [0xD6 , 0x90 , 0xE9 , 0xFE , 0xCC , 0xE1 , 0x3D , 0xB7 , 0x16 , 0xB6 , 0x14 , 0xC2 , 0x28 , 0xFB , 0x2C , 0x05 , 0x2B , 0x67 , 0x9A , 0x76 , 0x2A , 0xBE , 0x04 , 0xC3 , 0xAA , 0x44 , 0x13 , 0x26 , 0x49 , 0x86 , 0x06 , 0x99 , 0x9C , 0x42 , 0x50 , 0xF4 , 0x91 , 0xEF , 0x98 , 0x7A , 0x33 , 0x54 , 0x0B , 0x43 , 0xED , 0xCF , 0xAC , 0x62 , 0xE4 , 0xB3 , 0x1C , 0xA9 , 0xC9 , 0x08 , 0xE8 , 0x95 , 0x80 , 0xDF , 0x94 , 0xFA , 0x75 , 0x8F , 0x3F , 0xA6 , 0x47 , 0x07 , 0xA7 , 0xFC , 0xF3 , 0x73 , 0x17 , 0xBA , 0x83 , 0x59 , 0x3C , 0x19 , 0xE6 , 0x85 , 0x4F , 0xA8 , 0x68 , 0x6B , 0x81 , 0xB2 , 0x71 , 0x64 , 0xDA , 0x8B , 0xF8 , 0xEB , 0x0F , 0x4B , 0x70 , 0x56 , 0x9D , 0x35 , 0x1E , 0x24 , 0x0E , 0x5E , 0x63 , 0x58 , 0xD1 , 0xA2 , 0x25 , 0x22 , 0x7C , 0x3B , 0x01 , 0x21 , 0x78 , 0x87 , 0xD4 , 0x00 , 0x46 , 0x57 , 0x9F , 0xD3 , 0x27 , 0x52 , 0x4C , 0x36 , 0x02 , 0xE7 , 0xA0 , 0xC4 , 0xC8 , 0x9E , 0xEA , 0xBF , 0x8A , 0xD2 , 0x40 , 0xC7 , 0x38 , 0xB5 , 0xA3 , 0xF7 , 0xF2 , 0xCE , 0xF9 , 0x61 , 0x15 , 0xA1 , 0xE0 , 0xAE , 0x5D , 0xA4 , 0x9B , 0x34 , 0x1A , 0x55 , 0xAD , 0x93 , 0x32 , 0x30 , 0xF5 , 0x8C , 0xB1 , 0xE3 , 0x1D , 0xF6 , 0xE2 , 0x2E , 0x82 , 0x66 , 0xCA , 0x60 , 0xC0 , 0x29 , 0x23 , 0xAB , 0x0D , 0x53 , 0x4E , 0x6F , 0xD5 , 0xDB , 0x37 , 0x45 , 0xDE , 0xFD , 0x8E , 0x2F , 0x03 , 0xFF , 0x6A , 0x72 , 0x6D , 0x6C , 0x5B , 0x51 , 0x8D , 0x1B , 0xAF , 0x92 , 0xBB , 0xDD , 0xBC , 0x7F , 0x11 , 0xD9 , 0x5C , 0x41 , 0x1F , 0x10 , 0x5A , 0xD8 , 0x0A , 0xC1 , 0x31 , 0x88 , 0xA5 , 0xCD , 0x7B , 0xBD , 0x2D , 0x74 , 0xD0 , 0x12 , 0xB8 , 0xE5 , 0xB4 , 0xB0 , 0x89 , 0x69 , 0x97 , 0x4A , 0x0C , 0x96 , 0x77 , 0x7E , 0x65 , 0xB9 , 0xF1 , 0x09 , 0xC5 , 0x6E , 0xC6 , 0x84 , 0x18 , 0xF0 , 0x7D , 0xEC , 0x3A , 0xDC , 0x4D , 0x20 , 0x79 , 0xEE , 0x5F , 0x3E , 0xD7 , 0xCB , 0x39 , 0x48 ,] ; ((SBOX [(x >> 24) as usize] as u32) << 24) | ((SBOX [((x >> 16) & 0xff) as usize] as u32) << 16) | ((SBOX [((x >> 8) & 0xff) as usize] as u32) << 8) | (SBOX [(x & 0xff) as usize] as u32) }
}

macro_rules! test_mm_sm4key4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sm4key4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sm4key4_epi32_introspect!();
    #[simd_test (enable = "sm4,avx")] unsafe fn test_mm_sm4key4_epi32 () { fn l_key (x : u32) -> u32 { x ^ x . rotate_left (13) ^ x . rotate_left (23) } fn f_key (x0 : u32 , x1 : u32 , x2 : u32 , x3 : u32 , rk : u32) -> u32 { x0 ^ l_key (lower_t (x1 ^ x2 ^ x3 ^ rk)) } let A = & DATA_32 [0 .. 4] ; let B = & DATA_32 [4 .. 8] ; let a = _mm_loadu_si128 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let r = _mm_sm4key4_epi32 (a , b) ; let e0 = f_key (A [0] , A [1] , A [2] , A [3] , B [0]) ; let e1 = f_key (A [1] , A [2] , A [3] , e0 , B [1]) ; let e2 = f_key (A [2] , A [3] , e0 , e1 , B [2]) ; let e3 = f_key (A [3] , e0 , e1 , e2 , B [3]) ; let e = _mm_setr_epi32 (e0 as _ , e1 as _ , e2 as _ , e3 as _) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_sm4key4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sm4key4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sm4key4_epi32_introspect!();
    #[simd_test (enable = "sm4,avx")] unsafe fn test_mm256_sm4key4_epi32 () { let a_low = _mm_loadu_si128 (DATA_32 . as_ptr () . cast ()) ; let a_high = _mm_loadu_si128 (DATA_32 [4 ..] . as_ptr () . cast ()) ; let b_low = _mm_loadu_si128 (DATA_32 [8 ..] . as_ptr () . cast ()) ; let b_high = _mm_loadu_si128 (DATA_32 [12 ..] . as_ptr () . cast ()) ; let a = _mm256_set_m128i (a_high , a_low) ; let b = _mm256_set_m128i (b_high , b_low) ; let r = _mm256_sm4key4_epi32 (a , b) ; let e_low = _mm_sm4key4_epi32 (a_low , b_low) ; let e_high = _mm_sm4key4_epi32 (a_high , b_high) ; let e = _mm256_set_m128i (e_high , e_low) ; assert_eq_m256i (r , e) ; }
}

macro_rules! test_mm_sm4rnds4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm_sm4rnds4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm_sm4rnds4_epi32_introspect!();
    #[simd_test (enable = "sm4,avx")] unsafe fn test_mm_sm4rnds4_epi32 () { fn l_rnd (x : u32) -> u32 { x ^ x . rotate_left (2) ^ x . rotate_left (10) ^ x . rotate_left (18) ^ x . rotate_left (24) } fn f_rnd (x0 : u32 , x1 : u32 , x2 : u32 , x3 : u32 , rk : u32) -> u32 { x0 ^ l_rnd (lower_t (x1 ^ x2 ^ x3 ^ rk)) } let A = & DATA_32 [0 .. 4] ; let B = & DATA_32 [4 .. 8] ; let a = _mm_loadu_si128 (A . as_ptr () . cast ()) ; let b = _mm_loadu_si128 (B . as_ptr () . cast ()) ; let r = _mm_sm4rnds4_epi32 (a , b) ; let e0 = f_rnd (A [0] , A [1] , A [2] , A [3] , B [0]) ; let e1 = f_rnd (A [1] , A [2] , A [3] , e0 , B [1]) ; let e2 = f_rnd (A [2] , A [3] , e0 , e1 , B [2]) ; let e3 = f_rnd (A [3] , e0 , e1 , e2 , B [3]) ; let e = _mm_setr_epi32 (e0 as _ , e1 as _ , e2 as _ , e3 as _) ; assert_eq_m128i (r , e) ; }
}

macro_rules! test_mm256_sm4rnds4_epi32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_mm256_sm4rnds4_epi32 in module {}", module_path!());
    };
}

mkfn!{
    test_mm256_sm4rnds4_epi32_introspect!();
    #[simd_test (enable = "sm4,avx")] unsafe fn test_mm256_sm4rnds4_epi32 () { let a_low = _mm_loadu_si128 (DATA_32 . as_ptr () . cast ()) ; let a_high = _mm_loadu_si128 (DATA_32 [4 ..] . as_ptr () . cast ()) ; let b_low = _mm_loadu_si128 (DATA_32 [8 ..] . as_ptr () . cast ()) ; let b_high = _mm_loadu_si128 (DATA_32 [12 ..] . as_ptr () . cast ()) ; let a = _mm256_set_m128i (a_high , a_low) ; let b = _mm256_set_m128i (b_high , b_low) ; let r = _mm256_sm4rnds4_epi32 (a , b) ; let e_low = _mm_sm4rnds4_epi32 (a_low , b_low) ; let e_high = _mm_sm4rnds4_epi32 (a_high , b_high) ; let e = _mm256_set_m128i (e_high , e_low) ; assert_eq_m256i (r , e) ; }
} 
            }}