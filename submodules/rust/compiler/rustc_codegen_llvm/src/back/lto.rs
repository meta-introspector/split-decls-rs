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
mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use std :: ffi :: { CStr , CString } ;}
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: ptr :: NonNull ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: { io , iter , slice } ;}
mkuse!{use object :: read :: archive :: ArchiveFile ;}
mkuse!{use object :: { Object , ObjectSection } ;}
mkuse!{use rustc_codegen_ssa :: back :: lto :: { SerializedModule , ThinModule , ThinShared } ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: { CodegenContext , FatLtoInput } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_codegen_ssa :: { ModuleCodegen , ModuleKind , looks_like_rust_object_file } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_hir :: attrs :: SanitizerSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: dep_graph :: WorkProduct ;}
mkuse!{use rustc_session :: config :: { self , Lto } ;}
mkuse!{use tracing :: { debug , info } ;}
mkuse!{use crate :: back :: write :: { self , CodegenDiagnosticsStage , DiagnosticHandlers , bitcode_section_name , save_temp_bitcode , } ;}
mkuse!{use crate :: errors :: { LlvmError , LtoBitcodeFromRlib } ;}
mkuse!{use crate :: llvm :: { self , build_string } ;}
mkuse!{use crate :: { LlvmCodegenBackend , ModuleLlvm , SimpleCx } ;}
mkitem!{#[doc = " We keep track of the computed LTO cache keys from the previous"] #[doc = " session to determine which CGUs we can reuse."] const THIN_LTO_KEYS_INCR_COMP_FILE_NAME : & str = "thin-lto-past-keys.bin" ;}

macro_rules! prepare_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_lto in module {}", module_path!());
    };
}

mkfn!{
    prepare_lto_introspect!();
    fn prepare_lto (cgcx : & CodegenContext < LlvmCodegenBackend > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , dcx : DiagCtxtHandle < '_ > ,) -> (Vec < CString > , Vec < (SerializedModule < ModuleBuffer > , CString) >) { let mut symbols_below_threshold = exported_symbols_for_lto . iter () . map (| symbol | CString :: new (symbol . to_owned ()) . unwrap ()) . collect :: < Vec < CString > > () ; if cgcx . module_config . instrument_coverage || cgcx . module_config . pgo_gen . enabled () { const PROFILER_WEAK_SYMBOLS : [& CStr ; 2] = [c"__llvm_profile_raw_version" , c"__llvm_profile_filename"] ; symbols_below_threshold . extend (PROFILER_WEAK_SYMBOLS . iter () . map (| & sym | sym . to_owned ())) ; } if cgcx . module_config . sanitizer . contains (SanitizerSet :: MEMORY) { let mut msan_weak_symbols = Vec :: new () ; if cgcx . module_config . sanitizer_recover . contains (SanitizerSet :: MEMORY) { msan_weak_symbols . push (c"__msan_keep_going") ; } if cgcx . module_config . sanitizer_memory_track_origins != 0 { msan_weak_symbols . push (c"__msan_track_origins") ; } symbols_below_threshold . extend (msan_weak_symbols . into_iter () . map (| sym | sym . to_owned ())) ; } symbols_below_threshold . push (c"___asan_globals_registered" . to_owned ()) ; symbols_below_threshold . push (c"__llvm_profile_counter_bias" . to_owned ()) ; let mut upstream_modules = Vec :: new () ; if cgcx . lto != Lto :: ThinLocal { for path in each_linked_rlib_for_lto { let archive_data = unsafe { Mmap :: map (std :: fs :: File :: open (& path) . expect ("couldn't open rlib")) . expect ("couldn't map rlib") } ; let archive = ArchiveFile :: parse (& * archive_data) . expect ("wanted an rlib") ; let obj_files = archive . members () . filter_map (| child | { child . ok () . and_then (| c | { std :: str :: from_utf8 (c . name ()) . ok () . map (| name | (name . trim () , c)) }) }) . filter (| & (name , _) | looks_like_rust_object_file (name)) ; for (name , child) in obj_files { info ! ("adding bitcode from {}" , name) ; match get_bitcode_slice_from_object_data (child . data (& * archive_data) . expect ("corrupt rlib") , cgcx ,) { Ok (data) => { let module = SerializedModule :: FromRlib (data . to_vec ()) ; upstream_modules . push ((module , CString :: new (name) . unwrap ())) ; } Err (e) => dcx . emit_fatal (e) , } } } } (symbols_below_threshold , upstream_modules) }
}

macro_rules! get_bitcode_slice_from_object_data_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_bitcode_slice_from_object_data in module {}", module_path!());
    };
}

mkfn!{
    get_bitcode_slice_from_object_data_introspect!();
    fn get_bitcode_slice_from_object_data < 'a > (obj : & 'a [u8] , cgcx : & CodegenContext < LlvmCodegenBackend > ,) -> Result < & 'a [u8] , LtoBitcodeFromRlib > { if obj . starts_with (b"\xDE\xC0\x17\x0B") || obj . starts_with (b"BC\xC0\xDE") { return Ok (obj) ; } let section_name = bitcode_section_name (cgcx) . to_str () . unwrap () . trim_start_matches ("__LLVM,") ; let obj = object :: File :: parse (obj) . map_err (| err | LtoBitcodeFromRlib { err : err . to_string () }) ? ; let section = obj . section_by_name (section_name) . ok_or_else (| | LtoBitcodeFromRlib { err : format ! ("Can't find section {section_name}") }) ? ; section . data () . map_err (| err | LtoBitcodeFromRlib { err : err . to_string () }) }
}

macro_rules! run_fat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_fat in module {}", module_path!());
    };
}

mkfn!{
    run_fat_introspect!();
    #[doc = " Performs fat LTO by merging all modules into a single one and returning it"] #[doc = " for further optimization."] pub (crate) fn run_fat (cgcx : & CodegenContext < LlvmCodegenBackend > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < LlvmCodegenBackend > > ,) -> ModuleCodegen < ModuleLlvm > { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let (symbols_below_threshold , upstream_modules) = prepare_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , dcx) ; let symbols_below_threshold = symbols_below_threshold . iter () . map (| c | c . as_ptr ()) . collect :: < Vec < _ > > () ; fat_lto (cgcx , dcx , modules , upstream_modules , & symbols_below_threshold) }
}

macro_rules! run_thin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_thin in module {}", module_path!());
    };
}

mkfn!{
    run_thin_introspect!();
    #[doc = " Performs thin LTO by performing necessary global analysis and returning two"] #[doc = " lists, one of the modules that need optimization and another for modules that"] #[doc = " can simply be copied over from the incr. comp. cache."] pub (crate) fn run_thin (cgcx : & CodegenContext < LlvmCodegenBackend > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , ThinBuffer) > , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < LlvmCodegenBackend > > , Vec < WorkProduct >) { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let (symbols_below_threshold , upstream_modules) = prepare_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , dcx) ; let symbols_below_threshold = symbols_below_threshold . iter () . map (| c | c . as_ptr ()) . collect :: < Vec < _ > > () ; if cgcx . opts . cg . linker_plugin_lto . enabled () { unreachable ! ("We should never reach this case if the LTO step \
                      is deferred to the linker") ; } thin_lto (cgcx , dcx , modules , upstream_modules , cached_modules , & symbols_below_threshold) }
}

macro_rules! prepare_thin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_thin in module {}", module_path!());
    };
}

mkfn!{
    prepare_thin_introspect!();
    pub (crate) fn prepare_thin (module : ModuleCodegen < ModuleLlvm >) -> (String , ThinBuffer) { let name = module . name ; let buffer = ThinBuffer :: new (module . module_llvm . llmod () , true) ; (name , buffer) }
}

macro_rules! fat_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fat_lto in module {}", module_path!());
    };
}

mkfn!{
    fat_lto_introspect!();
    fn fat_lto (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , modules : Vec < FatLtoInput < LlvmCodegenBackend > > , mut serialized_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , symbols_below_threshold : & [* const libc :: c_char] ,) -> ModuleCodegen < ModuleLlvm > { let _timer = cgcx . prof . generic_activity ("LLVM_fat_lto_build_monolithic_module") ; info ! ("going for a fat lto") ; let mut in_memory = Vec :: new () ; for module in modules { match module { FatLtoInput :: InMemory (m) => in_memory . push (m) , FatLtoInput :: Serialized { name , buffer } => { info ! ("pushing serialized module {:?}" , name) ; serialized_modules . push ((buffer , CString :: new (name) . unwrap ())) ; } } } let costliest_module = in_memory . iter () . enumerate () . filter (| & (_ , module) | module . kind == ModuleKind :: Regular) . map (| (i , module) | { let cost = unsafe { llvm :: LLVMRustModuleCost (module . module_llvm . llmod ()) } ; (cost , i) }) . max () ; let module : ModuleCodegen < ModuleLlvm > = match costliest_module { Some ((_cost , i)) => in_memory . remove (i) , None => { assert ! (! serialized_modules . is_empty () , "must have at least one serialized module") ; let (buffer , name) = serialized_modules . remove (0) ; info ! ("no in-memory regular modules to choose from, parsing {:?}" , name) ; let llvm_module = ModuleLlvm :: parse (cgcx , & name , buffer . data () , dcx) ; ModuleCodegen :: new_regular (name . into_string () . unwrap () , llvm_module) } } ; { let (llcx , llmod) = { let llvm = & module . module_llvm ; (& llvm . llcx , llvm . llmod ()) } ; info ! ("using {:?} as a base module" , module . name) ; let _handler = DiagnosticHandlers :: new (cgcx , dcx , llcx , & module , CodegenDiagnosticsStage :: LTO) ; for module in in_memory { let buffer = ModuleBuffer :: new (module . module_llvm . llmod ()) ; let llmod_id = CString :: new (& module . name [..]) . unwrap () ; serialized_modules . push ((SerializedModule :: Local (buffer) , llmod_id)) ; } serialized_modules . sort_by (| module1 , module2 | module1 . 1 . cmp (& module2 . 1)) ; let mut linker = Linker :: new (llmod) ; for (bc_decoded , name) in serialized_modules { let _timer = cgcx . prof . generic_activity_with_arg_recorder ("LLVM_fat_lto_link_module" , | recorder | { recorder . record_arg (format ! ("{name:?}")) }) ; info ! ("linking {:?}" , name) ; let data = bc_decoded . data () ; linker . add (data) . unwrap_or_else (| () | write :: llvm_err (dcx , LlvmError :: LoadBitcode { name })) ; } drop (linker) ; save_temp_bitcode (cgcx , & module , "lto.input") ; unsafe { let ptr = symbols_below_threshold . as_ptr () ; llvm :: LLVMRustRunRestrictionPass (llmod , ptr as * const * const libc :: c_char , symbols_below_threshold . len () as libc :: size_t ,) ; } save_temp_bitcode (cgcx , & module , "lto.after-restriction") ; } module }
}
mkitem!{mkstruct!{pub (crate) struct Linker < 'a > (& 'a mut llvm :: Linker < 'a >) ;}}
mkitem!{mkimpl!{impl < 'a > Linker < 'a > { pub (crate) fn new (llmod : & 'a llvm :: Module) -> Self { unsafe { Linker (llvm :: LLVMRustLinkerNew (llmod)) } } pub (crate) fn add (& mut self , bytecode : & [u8]) -> Result < () , () > { unsafe { if llvm :: LLVMRustLinkerAdd (self . 0 , bytecode . as_ptr () as * const libc :: c_char , bytecode . len () ,) { Ok (()) } else { Err (()) } } } }}}
mkitem!{mkimpl!{impl Drop for Linker < '_ > { fn drop (& mut self) { unsafe { llvm :: LLVMRustLinkerFree (& mut * (self . 0 as * mut _)) ; } } }}}

macro_rules! thin_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function thin_lto in module {}", module_path!());
    };
}

mkfn!{
    thin_lto_introspect!();
    #[doc = " Prepare \"thin\" LTO to get run on these modules."] #[doc = ""] #[doc = " The general structure of ThinLTO is quite different from the structure of"] #[doc = " \"fat\" LTO above. With \"fat\" LTO all LLVM modules in question are merged into"] #[doc = " one giant LLVM module, and then we run more optimization passes over this"] #[doc = " big module after internalizing most symbols. Thin LTO, on the other hand,"] #[doc = " avoid this large bottleneck through more targeted optimization."] #[doc = ""] #[doc = " At a high level Thin LTO looks like:"] #[doc = ""] #[doc = "    1. Prepare a \"summary\" of each LLVM module in question which describes"] #[doc = "       the values inside, cost of the values, etc."] #[doc = "    2. Merge the summaries of all modules in question into one \"index\""] #[doc = "    3. Perform some global analysis on this index"] #[doc = "    4. For each module, use the index and analysis calculated previously to"] #[doc = "       perform local transformations on the module, for example inlining"] #[doc = "       small functions from other modules."] #[doc = "    5. Run thin-specific optimization passes over each module, and then code"] #[doc = "       generate everything at the end."] #[doc = ""] #[doc = " The summary for each module is intended to be quite cheap, and the global"] #[doc = " index is relatively quite cheap to create as well. As a result, the goal of"] #[doc = " ThinLTO is to reduce the bottleneck on LTO and enable LTO to be used in more"] #[doc = " situations. For example one cheap optimization is that we can parallelize"] #[doc = " all codegen modules, easily making use of all the cores on a machine."] #[doc = ""] #[doc = " With all that in mind, the function here is designed at specifically just"] #[doc = " calculating the *index* for ThinLTO. This index will then be shared amongst"] #[doc = " all of the `LtoModuleCodegen` units returned below and destroyed once"] #[doc = " they all go out of scope."] fn thin_lto (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , modules : Vec < (String , ThinBuffer) > , serialized_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > , symbols_below_threshold : & [* const libc :: c_char] ,) -> (Vec < ThinModule < LlvmCodegenBackend > > , Vec < WorkProduct >) { let _timer = cgcx . prof . generic_activity ("LLVM_thin_lto_global_analysis") ; unsafe { info ! ("going for that thin, thin LTO") ; let green_modules : FxHashMap < _ , _ > = cached_modules . iter () . map (| (_ , wp) | (wp . cgu_name . clone () , wp . clone ())) . collect () ; let full_scope_len = modules . len () + serialized_modules . len () + cached_modules . len () ; let mut thin_buffers = Vec :: with_capacity (modules . len ()) ; let mut module_names = Vec :: with_capacity (full_scope_len) ; let mut thin_modules = Vec :: with_capacity (full_scope_len) ; for (i , (name , buffer)) in modules . into_iter () . enumerate () { info ! ("local module: {} - {}" , i , name) ; let cname = CString :: new (name . as_bytes ()) . unwrap () ; thin_modules . push (llvm :: ThinLTOModule { identifier : cname . as_ptr () , data : buffer . data () . as_ptr () , len : buffer . data () . len () , }) ; thin_buffers . push (buffer) ; module_names . push (cname) ; } let mut serialized = Vec :: with_capacity (serialized_modules . len () + cached_modules . len ()) ; let cached_modules = cached_modules . into_iter () . map (| (sm , wp) | (sm , CString :: new (wp . cgu_name) . unwrap ())) ; for (module , name) in serialized_modules . into_iter () . chain (cached_modules) { info ! ("upstream or cached module {:?}" , name) ; thin_modules . push (llvm :: ThinLTOModule { identifier : name . as_ptr () , data : module . data () . as_ptr () , len : module . data () . len () , }) ; serialized . push (module) ; module_names . push (name) ; } assert_eq ! (thin_modules . len () , module_names . len ()) ; let data = llvm :: LLVMRustCreateThinLTOData (thin_modules . as_ptr () , thin_modules . len () , symbols_below_threshold . as_ptr () , symbols_below_threshold . len () ,) . unwrap_or_else (| | write :: llvm_err (dcx , LlvmError :: PrepareThinLtoContext)) ; let data = ThinData (data) ; info ! ("thin LTO data created") ; let (key_map_path , prev_key_map , curr_key_map) = if let Some (ref incr_comp_session_dir) = cgcx . incr_comp_session_dir { let path = incr_comp_session_dir . join (THIN_LTO_KEYS_INCR_COMP_FILE_NAME) ; let prev = if path . exists () { ThinLTOKeysMap :: load_from_file (& path) . ok () } else { None } ; let curr = ThinLTOKeysMap :: from_thin_lto_modules (& data , & thin_modules , & module_names) ; (Some (path) , prev , curr) } else { assert ! (green_modules . is_empty ()) ; let curr = ThinLTOKeysMap :: default () ; (None , None , curr) } ; info ! ("thin LTO cache key map loaded") ; info ! ("prev_key_map: {:#?}" , prev_key_map) ; info ! ("curr_key_map: {:#?}" , curr_key_map) ; let shared = Arc :: new (ThinShared { data , thin_buffers , serialized_modules : serialized , module_names , }) ; let mut copy_jobs = vec ! [] ; let mut opt_jobs = vec ! [] ; info ! ("checking which modules can be-reused and which have to be re-optimized.") ; for (module_index , module_name) in shared . module_names . iter () . enumerate () { let module_name = module_name_to_str (module_name) ; if let (Some (prev_key_map) , true) = (prev_key_map . as_ref () , green_modules . contains_key (module_name)) { assert ! (cgcx . incr_comp_session_dir . is_some ()) ; if prev_key_map . keys . get (module_name) == curr_key_map . keys . get (module_name) { let work_product = green_modules [module_name] . clone () ; copy_jobs . push (work_product) ; info ! (" - {}: re-used" , module_name) ; assert ! (cgcx . incr_comp_session_dir . is_some ()) ; continue ; } } info ! (" - {}: re-compiled" , module_name) ; opt_jobs . push (ThinModule { shared : Arc :: clone (& shared) , idx : module_index }) ; } if let Some (path) = key_map_path && let Err (err) = curr_key_map . save_to_file (& path) { write :: llvm_err (dcx , LlvmError :: WriteThinLtoKey { err }) ; } (opt_jobs , copy_jobs) } }
}

macro_rules! enable_autodiff_settings_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable_autodiff_settings in module {}", module_path!());
    };
}

mkfn!{
    enable_autodiff_settings_introspect!();
    fn enable_autodiff_settings (ad : & [config :: AutoDiff]) { for val in ad { match val { config :: AutoDiff :: PrintPerf => { llvm :: set_print_perf (true) ; } config :: AutoDiff :: PrintAA => { llvm :: set_print_activity (true) ; } config :: AutoDiff :: PrintTA => { llvm :: set_print_type (true) ; } config :: AutoDiff :: PrintTAFn (fun) => { llvm :: set_print_type (true) ; llvm :: set_print_type_fun (& fun) ; } config :: AutoDiff :: Inline => { llvm :: set_inline (true) ; } config :: AutoDiff :: LooseTypes => { llvm :: set_loose_types (true) ; } config :: AutoDiff :: PrintSteps => { llvm :: set_print (true) ; } config :: AutoDiff :: PrintPasses => { } config :: AutoDiff :: PrintModBefore => { } config :: AutoDiff :: PrintModAfter => { } config :: AutoDiff :: PrintModFinal => { } config :: AutoDiff :: Enable => { } config :: AutoDiff :: NoPostopt => { } } } llvm :: set_strict_aliasing (false) ; llvm :: set_rust_rules (true) ; }
}

macro_rules! run_pass_manager_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_pass_manager in module {}", module_path!());
    };
}

mkfn!{
    run_pass_manager_introspect!();
    pub (crate) fn run_pass_manager (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < ModuleLlvm > , thin : bool ,) { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_lto_optimize" , & * module . name) ; let config = & cgcx . module_config ; debug ! ("running the pass manager") ; let opt_stage = if thin { llvm :: OptStage :: ThinLTO } else { llvm :: OptStage :: FatLTO } ; let opt_level = config . opt_level . unwrap_or (config :: OptLevel :: No) ; let enable_ad = config . autodiff . contains (& config :: AutoDiff :: Enable) ; let enable_gpu = config . offload . contains (& config :: Offload :: Enable) ; let stage = if thin { write :: AutodiffStage :: PreAD } else { if enable_ad { write :: AutodiffStage :: DuringAD } else { write :: AutodiffStage :: PostAD } } ; if enable_ad { enable_autodiff_settings (& config . autodiff) ; } unsafe { write :: llvm_optimize (cgcx , dcx , module , None , config , opt_level , opt_stage , stage) ; } if enable_gpu && ! thin { let cx = SimpleCx :: new (module . module_llvm . llmod () , & module . module_llvm . llcx , cgcx . pointer_size) ; crate :: builder :: gpu_offload :: handle_gpu_code (cgcx , & cx) ; } if cfg ! (llvm_enzyme) && enable_ad && ! thin { let opt_stage = llvm :: OptStage :: FatLTO ; let stage = write :: AutodiffStage :: PostAD ; if ! config . autodiff . contains (& config :: AutoDiff :: NoPostopt) { unsafe { write :: llvm_optimize (cgcx , dcx , module , None , config , opt_level , opt_stage , stage) ; } } if config . autodiff . contains (& config :: AutoDiff :: PrintModFinal) { unsafe { llvm :: LLVMDumpModule (module . module_llvm . llmod ()) } ; } } debug ! ("lto done") ; }
}
mkitem!{mkstruct!{pub struct ModuleBuffer (& 'static mut llvm :: ModuleBuffer) ;}}
mkitem!{mkimpl!{unsafe impl Send for ModuleBuffer { }}}
mkitem!{mkimpl!{unsafe impl Sync for ModuleBuffer { }}}
mkitem!{mkimpl!{impl ModuleBuffer { pub (crate) fn new (m : & llvm :: Module) -> ModuleBuffer { ModuleBuffer (unsafe { llvm :: LLVMRustModuleBufferCreate (m) }) } }}}
mkitem!{mkimpl!{impl ModuleBufferMethods for ModuleBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustModuleBufferPtr (self . 0) ; let len = llvm :: LLVMRustModuleBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }}}
mkitem!{mkimpl!{impl Drop for ModuleBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustModuleBufferFree (& mut * (self . 0 as * mut _)) ; } } }}}
mkitem!{mkstruct!{pub struct ThinData (& 'static mut llvm :: ThinLTOData) ;}}
mkitem!{mkimpl!{unsafe impl Send for ThinData { }}}
mkitem!{mkimpl!{unsafe impl Sync for ThinData { }}}
mkitem!{mkimpl!{impl Drop for ThinData { fn drop (& mut self) { unsafe { llvm :: LLVMRustFreeThinLTOData (& mut * (self . 0 as * mut _)) ; } } }}}
mkitem!{mkstruct!{pub struct ThinBuffer (& 'static mut llvm :: ThinLTOBuffer) ;}}
mkitem!{mkimpl!{unsafe impl Send for ThinBuffer { }}}
mkitem!{mkimpl!{unsafe impl Sync for ThinBuffer { }}}
mkitem!{mkimpl!{impl ThinBuffer { pub (crate) fn new (m : & llvm :: Module , is_thin : bool) -> ThinBuffer { unsafe { let buffer = llvm :: LLVMRustThinLTOBufferCreate (m , is_thin) ; ThinBuffer (buffer) } } pub (crate) unsafe fn from_raw_ptr (ptr : * mut llvm :: ThinLTOBuffer) -> ThinBuffer { let mut ptr = NonNull :: new (ptr) . unwrap () ; ThinBuffer (unsafe { ptr . as_mut () }) } pub (crate) fn thin_link_data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustThinLTOBufferThinLinkDataPtr (self . 0) as * const _ ; let len = llvm :: LLVMRustThinLTOBufferThinLinkDataLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }}}
mkitem!{mkimpl!{impl ThinBufferMethods for ThinBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustThinLTOBufferPtr (self . 0) as * const _ ; let len = llvm :: LLVMRustThinLTOBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }}}
mkitem!{mkimpl!{impl Drop for ThinBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustThinLTOBufferFree (& mut * (self . 0 as * mut _)) ; } } }}}

macro_rules! optimize_thin_module_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function optimize_thin_module in module {}", module_path!());
    };
}

mkfn!{
    optimize_thin_module_introspect!();
    pub (crate) fn optimize_thin_module (thin_module : ThinModule < LlvmCodegenBackend > , cgcx : & CodegenContext < LlvmCodegenBackend > ,) -> ModuleCodegen < ModuleLlvm > { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let module_name = & thin_module . shared . module_names [thin_module . idx] ; let module_llvm = ModuleLlvm :: parse (cgcx , module_name , thin_module . data () , dcx) ; let mut module = ModuleCodegen :: new_regular (thin_module . name () , module_llvm) ; if cgcx . module_config . embed_bitcode () { module . thin_lto_buffer = Some (thin_module . data () . to_vec ()) ; } { let target = & * module . module_llvm . tm ; let llmod = module . module_llvm . llmod () ; save_temp_bitcode (cgcx , & module , "thin-lto-input") ; { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_thin_lto_rename" , thin_module . name ()) ; unsafe { llvm :: LLVMRustPrepareThinLTORename (thin_module . shared . data . 0 , llmod , target . raw ()) } ; save_temp_bitcode (cgcx , & module , "thin-lto-after-rename") ; } { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_thin_lto_resolve_weak" , thin_module . name ()) ; if unsafe { ! llvm :: LLVMRustPrepareThinLTOResolveWeak (thin_module . shared . data . 0 , llmod) } { write :: llvm_err (dcx , LlvmError :: PrepareThinLtoModule) ; } save_temp_bitcode (cgcx , & module , "thin-lto-after-resolve") ; } { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_thin_lto_internalize" , thin_module . name ()) ; if unsafe { ! llvm :: LLVMRustPrepareThinLTOInternalize (thin_module . shared . data . 0 , llmod) } { write :: llvm_err (dcx , LlvmError :: PrepareThinLtoModule) ; } save_temp_bitcode (cgcx , & module , "thin-lto-after-internalize") ; } { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_thin_lto_import" , thin_module . name ()) ; if unsafe { ! llvm :: LLVMRustPrepareThinLTOImport (thin_module . shared . data . 0 , llmod , target . raw ()) } { write :: llvm_err (dcx , LlvmError :: PrepareThinLtoModule) ; } save_temp_bitcode (cgcx , & module , "thin-lto-after-import") ; } { info ! ("running thin lto passes over {}" , module . name) ; run_pass_manager (cgcx , dcx , & mut module , true) ; save_temp_bitcode (cgcx , & module , "thin-lto-after-pm") ; } } module }
}
mkitem!{mkstruct!{#[doc = " Maps LLVM module identifiers to their corresponding LLVM LTO cache keys"] #[derive (Debug , Default)] struct ThinLTOKeysMap { keys : BTreeMap < String , String > , }}}
mkitem!{mkimpl!{impl ThinLTOKeysMap { fn save_to_file (& self , path : & Path) -> io :: Result < () > { use std :: io :: Write ; let mut writer = File :: create_buffered (path) ? ; for (module , key) in & self . keys { writeln ! (writer , "{module} {key}") ? ; } Ok (()) } fn load_from_file (path : & Path) -> io :: Result < Self > { use std :: io :: BufRead ; let mut keys = BTreeMap :: default () ; let file = File :: open_buffered (path) ? ; for line in file . lines () { let line = line ? ; let mut split = line . split (' ') ; let module = split . next () . unwrap () ; let key = split . next () . unwrap () ; assert_eq ! (split . next () , None , "Expected two space-separated values, found {line:?}") ; keys . insert (module . to_string () , key . to_string ()) ; } Ok (Self { keys }) } fn from_thin_lto_modules (data : & ThinData , modules : & [llvm :: ThinLTOModule] , names : & [CString] ,) -> Self { let keys = iter :: zip (modules , names) . map (| (module , name) | { let key = build_string (| rust_str | unsafe { llvm :: LLVMRustComputeLTOCacheKey (rust_str , module . identifier , data . 0) ; }) . expect ("Invalid ThinLTO module key") ; (module_name_to_str (name) . to_string () , key) }) . collect () ; Self { keys } } }}}

macro_rules! module_name_to_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function module_name_to_str in module {}", module_path!());
    };
}

mkfn!{
    module_name_to_str_introspect!();
    fn module_name_to_str (c_str : & CStr) -> & str { c_str . to_str () . unwrap_or_else (| e | { bug ! ("Encountered non-utf8 LLVM module name `{}`: {}" , c_str . to_string_lossy () , e) }) }
}

macro_rules! parse_module_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_module in module {}", module_path!());
    };
}

mkfn!{
    parse_module_introspect!();
    pub (crate) fn parse_module < 'a > (cx : & 'a llvm :: Context , name : & CStr , data : & [u8] , dcx : DiagCtxtHandle < '_ > ,) -> & 'a llvm :: Module { unsafe { llvm :: LLVMRustParseBitcodeForLTO (cx , data . as_ptr () , data . len () , name . as_ptr ()) . unwrap_or_else (| | write :: llvm_err (dcx , LlvmError :: ParseBitcode)) } }
}