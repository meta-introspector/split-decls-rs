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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: panic :: AssertUnwindSafe ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: sync :: mpsc :: { Receiver , Sender , channel } ;}
mkuse!{use std :: { fs , io , mem , str , thread } ;}
mkuse!{use rustc_abi :: Size ;}
mkuse!{use rustc_ast :: attr ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: jobserver :: { self , Acquired } ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_data_structures :: profiling :: { SelfProfilerRef , VerboseTimingGuard } ;}
mkuse!{use rustc_errors :: emitter :: Emitter ;}
mkuse!{use rustc_errors :: translation :: Translator ;}
mkuse!{use rustc_errors :: { Diag , DiagArgMap , DiagCtxt , DiagMessage , ErrCode , FatalErrorMarker , Level , MultiSpan , Style , Suggestions , } ;}
mkuse!{use rustc_fs_util :: link_or_copy ;}
mkuse!{use rustc_incremental :: { copy_cgu_workproduct_to_incr_comp_cache_dir , in_incr_comp_dir , in_incr_comp_dir_sess , } ;}
mkuse!{use rustc_metadata :: fs :: copy_to_stdout ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: dep_graph :: { WorkProduct , WorkProductId } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { self , CrateType , Lto , OutFileName , OutputFilenames , OutputType , Passes , SwitchWithOptPath , } ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use rustc_span :: { FileName , InnerSpan , Span , SpanData , sym } ;}
mkuse!{use rustc_target :: spec :: { MergeFunctions , SanitizerSet } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: link :: { self , ensure_removed } ;}
mkuse!{use super :: lto :: { self , SerializedModule } ;}
mkuse!{use crate :: back :: lto :: check_lto_allowed ;}
mkuse!{use crate :: errors :: ErrorCreatingRemarkDir ;}
mkuse!{use crate :: traits :: * ;}
mkuse!{use crate :: { CachedModuleCodegen , CodegenResults , CompiledModule , CrateInfo , ModuleCodegen , ModuleKind , errors , } ;}
mkitem!{const PRE_LTO_BC_EXT : & str = "pre-lto.bc" ;}
mkitem!{mkenum!{#[doc = " What kind of object file to emit."] #[derive (Clone , Copy , PartialEq)] pub enum EmitObj { None , Bitcode , ObjectCode (BitcodeSection) , }}}
mkitem!{mkenum!{#[doc = " What kind of llvm bitcode section to embed in an object file."] #[derive (Clone , Copy , PartialEq)] pub enum BitcodeSection { None , Full , }}}
mkitem!{mkstruct!{#[doc = " Module-specific configuration for `optimize_and_codegen`."] pub struct ModuleConfig { #[doc = " Names of additional optimization passes to run."] pub passes : Vec < String > , #[doc = " Some(level) to optimize at a certain level, or None to run"] #[doc = " absolutely no optimizations (used for the allocator module)."] pub opt_level : Option < config :: OptLevel > , pub pgo_gen : SwitchWithOptPath , pub pgo_use : Option < PathBuf > , pub pgo_sample_use : Option < PathBuf > , pub debug_info_for_profiling : bool , pub instrument_coverage : bool , pub sanitizer : SanitizerSet , pub sanitizer_recover : SanitizerSet , pub sanitizer_dataflow_abilist : Vec < String > , pub sanitizer_memory_track_origins : usize , pub emit_pre_lto_bc : bool , pub emit_no_opt_bc : bool , pub emit_bc : bool , pub emit_ir : bool , pub emit_asm : bool , pub emit_obj : EmitObj , pub emit_thin_lto : bool , pub emit_thin_lto_summary : bool , pub verify_llvm_ir : bool , pub lint_llvm_ir : bool , pub no_prepopulate_passes : bool , pub no_builtins : bool , pub vectorize_loop : bool , pub vectorize_slp : bool , pub merge_functions : bool , pub emit_lifetime_markers : bool , pub llvm_plugins : Vec < String > , pub autodiff : Vec < config :: AutoDiff > , pub offload : Vec < config :: Offload > , }}}
mkitem!{mkimpl!{impl ModuleConfig { fn new (kind : ModuleKind , tcx : TyCtxt < '_ > , no_builtins : bool) -> ModuleConfig { macro_rules ! if_regular { ($ regular : expr , $ other : expr) => { if let ModuleKind :: Regular = kind { $ regular } else { $ other } } ; } let sess = tcx . sess ; let opt_level_and_size = if_regular ! (Some (sess . opts . optimize) , None) ; let save_temps = sess . opts . cg . save_temps ; let should_emit_obj = sess . opts . output_types . contains_key (& OutputType :: Exe) || match kind { ModuleKind :: Regular => sess . opts . output_types . contains_key (& OutputType :: Object) , ModuleKind :: Allocator => false , } ; let emit_obj = if ! should_emit_obj { EmitObj :: None } else if sess . target . obj_is_bitcode || (sess . opts . cg . linker_plugin_lto . enabled () && ! no_builtins) { EmitObj :: Bitcode } else if need_bitcode_in_object (tcx) { EmitObj :: ObjectCode (BitcodeSection :: Full) } else { EmitObj :: ObjectCode (BitcodeSection :: None) } ; ModuleConfig { passes : if_regular ! (sess . opts . cg . passes . clone () , vec ! []) , opt_level : opt_level_and_size , pgo_gen : if_regular ! (sess . opts . cg . profile_generate . clone () , SwitchWithOptPath :: Disabled) , pgo_use : if_regular ! (sess . opts . cg . profile_use . clone () , None) , pgo_sample_use : if_regular ! (sess . opts . unstable_opts . profile_sample_use . clone () , None) , debug_info_for_profiling : sess . opts . unstable_opts . debug_info_for_profiling , instrument_coverage : if_regular ! (sess . instrument_coverage () , false) , sanitizer : if_regular ! (sess . opts . unstable_opts . sanitizer , SanitizerSet :: empty ()) , sanitizer_dataflow_abilist : if_regular ! (sess . opts . unstable_opts . sanitizer_dataflow_abilist . clone () , Vec :: new ()) , sanitizer_recover : if_regular ! (sess . opts . unstable_opts . sanitizer_recover , SanitizerSet :: empty ()) , sanitizer_memory_track_origins : if_regular ! (sess . opts . unstable_opts . sanitizer_memory_track_origins , 0) , emit_pre_lto_bc : if_regular ! (save_temps || need_pre_lto_bitcode_for_incr_comp (sess) , false) , emit_no_opt_bc : if_regular ! (save_temps , false) , emit_bc : if_regular ! (save_temps || sess . opts . output_types . contains_key (& OutputType :: Bitcode) , save_temps) , emit_ir : if_regular ! (sess . opts . output_types . contains_key (& OutputType :: LlvmAssembly) , false) , emit_asm : if_regular ! (sess . opts . output_types . contains_key (& OutputType :: Assembly) , false) , emit_obj , emit_thin_lto : sess . opts . unstable_opts . emit_thin_lto && sess . lto () != Lto :: Fat , emit_thin_lto_summary : if_regular ! (sess . opts . output_types . contains_key (& OutputType :: ThinLinkBitcode) , false) , verify_llvm_ir : sess . verify_llvm_ir () , lint_llvm_ir : sess . opts . unstable_opts . lint_llvm_ir , no_prepopulate_passes : sess . opts . cg . no_prepopulate_passes , no_builtins : no_builtins || sess . target . no_builtins , vectorize_loop : ! sess . opts . cg . no_vectorize_loops && (sess . opts . optimize == config :: OptLevel :: More || sess . opts . optimize == config :: OptLevel :: Aggressive) , vectorize_slp : ! sess . opts . cg . no_vectorize_slp && sess . opts . optimize == config :: OptLevel :: Aggressive , merge_functions : match sess . opts . unstable_opts . merge_functions . unwrap_or (sess . target . merge_functions) { MergeFunctions :: Disabled => false , MergeFunctions :: Trampolines | MergeFunctions :: Aliases => { use config :: OptLevel :: * ; match sess . opts . optimize { Aggressive | More | SizeMin | Size => true , Less | No => false , } } } , emit_lifetime_markers : sess . emit_lifetime_markers () , llvm_plugins : if_regular ! (sess . opts . unstable_opts . llvm_plugins . clone () , vec ! []) , autodiff : if_regular ! (sess . opts . unstable_opts . autodiff . clone () , vec ! []) , offload : if_regular ! (sess . opts . unstable_opts . offload . clone () , vec ! []) , } } pub fn bitcode_needed (& self) -> bool { self . emit_bc || self . emit_thin_lto_summary || self . emit_obj == EmitObj :: Bitcode || self . emit_obj == EmitObj :: ObjectCode (BitcodeSection :: Full) } pub fn embed_bitcode (& self) -> bool { self . emit_obj == EmitObj :: ObjectCode (BitcodeSection :: Full) } }}}
mkitem!{mkstruct!{#[doc = " Configuration passed to the function returned by the `target_machine_factory`."] pub struct TargetMachineFactoryConfig { #[doc = " Split DWARF is enabled in LLVM by checking that `TM.MCOptions.SplitDwarfFile` isn't empty,"] #[doc = " so the path to the dwarf object has to be provided when we create the target machine."] #[doc = " This can be ignored by backends which do not need it for their Split DWARF support."] pub split_dwarf_file : Option < PathBuf > , #[doc = " The name of the output object file. Used for setting OutputFilenames in target options"] #[doc = " so that LLVM can emit the CodeView S_OBJNAME record in pdb files"] pub output_obj_file : Option < PathBuf > , }}}
mkitem!{mkimpl!{impl TargetMachineFactoryConfig { pub fn new (cgcx : & CodegenContext < impl WriteBackendMethods > , module_name : & str ,) -> TargetMachineFactoryConfig { let split_dwarf_file = if cgcx . target_can_use_split_dwarf { cgcx . output_filenames . split_dwarf_path (cgcx . split_debuginfo , cgcx . split_dwarf_kind , module_name , cgcx . invocation_temp . as_deref () ,) } else { None } ; let output_obj_file = Some (cgcx . output_filenames . temp_path_for_cgu (OutputType :: Object , module_name , cgcx . invocation_temp . as_deref () ,)) ; TargetMachineFactoryConfig { split_dwarf_file , output_obj_file } } }}}
mkitem!{pub type TargetMachineFactoryFn < B > = Arc < dyn Fn (TargetMachineFactoryConfig ,) -> Result < < B as WriteBackendMethods > :: TargetMachine , < B as WriteBackendMethods > :: TargetMachineError , > + Send + Sync , > ;}
mkitem!{mkstruct!{#[doc = " Additional resources used by optimize_and_codegen (not module specific)"] #[derive (Clone)] pub struct CodegenContext < B : WriteBackendMethods > { pub prof : SelfProfilerRef , pub lto : Lto , pub save_temps : bool , pub fewer_names : bool , pub time_trace : bool , pub opts : Arc < config :: Options > , pub crate_types : Vec < CrateType > , pub output_filenames : Arc < OutputFilenames > , pub invocation_temp : Option < String > , pub module_config : Arc < ModuleConfig > , pub allocator_config : Arc < ModuleConfig > , pub tm_factory : TargetMachineFactoryFn < B > , pub msvc_imps_needed : bool , pub is_pe_coff : bool , pub target_can_use_split_dwarf : bool , pub target_arch : String , pub target_is_like_darwin : bool , pub target_is_like_aix : bool , pub split_debuginfo : rustc_target :: spec :: SplitDebuginfo , pub split_dwarf_kind : rustc_session :: config :: SplitDwarfKind , pub pointer_size : Size , #[doc = " All commandline args used to invoke the compiler, with @file args fully expanded."] #[doc = " This will only be used within debug info, e.g. in the pdb file on windows"] #[doc = " This is mainly useful for other tools that reads that debuginfo to figure out"] #[doc = " how to call the compiler with the same arguments."] pub expanded_args : Vec < String > , #[doc = " Emitter to use for diagnostics produced during codegen."] pub diag_emitter : SharedEmitter , #[doc = " LLVM optimizations for which we want to print remarks."] pub remark : Passes , #[doc = " Directory into which should the LLVM optimization remarks be written."] #[doc = " If `None`, they will be written to stderr."] pub remark_dir : Option < PathBuf > , #[doc = " The incremental compilation session directory, or None if we are not"] #[doc = " compiling incrementally"] pub incr_comp_session_dir : Option < PathBuf > , #[doc = " `true` if the codegen should be run in parallel."] #[doc = ""] #[doc = " Depends on [`ExtraBackendMethods::supports_parallel()`] and `-Zno_parallel_backend`."] pub parallel : bool , }}}
mkitem!{mkimpl!{impl < B : WriteBackendMethods > CodegenContext < B > { pub fn create_dcx (& self) -> DiagCtxt { DiagCtxt :: new (Box :: new (self . diag_emitter . clone ())) } }}}

macro_rules! generate_thin_lto_work_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_thin_lto_work in module {}", module_path!());
    };
}

mkfn!{
    generate_thin_lto_work_introspect!();
    fn generate_thin_lto_work < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , needs_thin_lto : Vec < (String , B :: ThinBuffer) > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > ,) -> Vec < (WorkItem < B > , u64) > { let _prof_timer = cgcx . prof . generic_activity ("codegen_thin_generate_lto_work") ; let (lto_modules , copy_jobs) = B :: run_thin_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , needs_thin_lto , import_only_modules ,) ; lto_modules . into_iter () . map (| module | { let cost = module . cost () ; (WorkItem :: ThinLto (module) , cost) }) . chain (copy_jobs . into_iter () . map (| wp | { (WorkItem :: CopyPostLtoArtifacts (CachedModuleCodegen { name : wp . cgu_name . clone () , source : wp , }) , 0 ,) })) . collect () }
}
mkitem!{mkstruct!{struct CompiledModules { modules : Vec < CompiledModule > , allocator_module : Option < CompiledModule > , }}}

macro_rules! need_bitcode_in_object_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function need_bitcode_in_object in module {}", module_path!());
    };
}

mkfn!{
    need_bitcode_in_object_introspect!();
    fn need_bitcode_in_object (tcx : TyCtxt < '_ >) -> bool { let sess = tcx . sess ; sess . opts . cg . embed_bitcode && tcx . crate_types () . contains (& CrateType :: Rlib) && sess . opts . output_types . contains_key (& OutputType :: Exe) }
}

macro_rules! need_pre_lto_bitcode_for_incr_comp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function need_pre_lto_bitcode_for_incr_comp in module {}", module_path!());
    };
}

mkfn!{
    need_pre_lto_bitcode_for_incr_comp_introspect!();
    fn need_pre_lto_bitcode_for_incr_comp (sess : & Session) -> bool { if sess . opts . incremental . is_none () { return false ; } match sess . lto () { Lto :: No => false , Lto :: Fat | Lto :: Thin | Lto :: ThinLocal => true , } }
}

macro_rules! start_async_codegen_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function start_async_codegen in module {}", module_path!());
    };
}

mkfn!{
    start_async_codegen_introspect!();
    pub (crate) fn start_async_codegen < B : ExtraBackendMethods > (backend : B , tcx : TyCtxt < '_ > , target_cpu : String , allocator_module : Option < ModuleCodegen < B :: Module > > ,) -> OngoingCodegen < B > { let (coordinator_send , coordinator_receive) = channel () ; let crate_attrs = tcx . hir_attrs (rustc_hir :: CRATE_HIR_ID) ; let no_builtins = attr :: contains_name (crate_attrs , sym :: no_builtins) ; let crate_info = CrateInfo :: new (tcx , target_cpu) ; let regular_config = ModuleConfig :: new (ModuleKind :: Regular , tcx , no_builtins) ; let allocator_config = ModuleConfig :: new (ModuleKind :: Allocator , tcx , no_builtins) ; let (shared_emitter , shared_emitter_main) = SharedEmitter :: new () ; let (codegen_worker_send , codegen_worker_receive) = channel () ; let coordinator_thread = start_executing_work (backend . clone () , tcx , & crate_info , shared_emitter , codegen_worker_send , coordinator_receive , Arc :: new (regular_config) , Arc :: new (allocator_config) , allocator_module , coordinator_send . clone () ,) ; OngoingCodegen { backend , crate_info , codegen_worker_receive , shared_emitter_main , coordinator : Coordinator { sender : coordinator_send , future : Some (coordinator_thread) , phantom : PhantomData , } , output_filenames : Arc :: clone (tcx . output_filenames (())) , } }
}

macro_rules! copy_all_cgu_workproducts_to_incr_comp_cache_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_all_cgu_workproducts_to_incr_comp_cache_dir in module {}", module_path!());
    };
}

mkfn!{
    copy_all_cgu_workproducts_to_incr_comp_cache_dir_introspect!();
    fn copy_all_cgu_workproducts_to_incr_comp_cache_dir (sess : & Session , compiled_modules : & CompiledModules ,) -> FxIndexMap < WorkProductId , WorkProduct > { let mut work_products = FxIndexMap :: default () ; if sess . opts . incremental . is_none () { return work_products ; } let _timer = sess . timer ("copy_all_cgu_workproducts_to_incr_comp_cache_dir") ; for module in compiled_modules . modules . iter () . filter (| m | m . kind == ModuleKind :: Regular) { let mut files = Vec :: new () ; if let Some (object_file_path) = & module . object { files . push ((OutputType :: Object . extension () , object_file_path . as_path ())) ; } if let Some (dwarf_object_file_path) = & module . dwarf_object { files . push (("dwo" , dwarf_object_file_path . as_path ())) ; } if let Some (path) = & module . assembly { files . push ((OutputType :: Assembly . extension () , path . as_path ())) ; } if let Some (path) = & module . llvm_ir { files . push ((OutputType :: LlvmAssembly . extension () , path . as_path ())) ; } if let Some (path) = & module . bytecode { files . push ((OutputType :: Bitcode . extension () , path . as_path ())) ; } if let Some ((id , product)) = copy_cgu_workproduct_to_incr_comp_cache_dir (sess , & module . name , files . as_slice () , & module . links_from_incr_cache ,) { work_products . insert (id , product) ; } } work_products }
}

macro_rules! produce_final_output_artifacts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function produce_final_output_artifacts in module {}", module_path!());
    };
}

mkfn!{
    produce_final_output_artifacts_introspect!();
    fn produce_final_output_artifacts (sess : & Session , compiled_modules : & CompiledModules , crate_output : & OutputFilenames ,) { let mut user_wants_bitcode = false ; let mut user_wants_objects = false ; let copy_gracefully = | from : & Path , to : & OutFileName | match to { OutFileName :: Stdout if let Err (e) = copy_to_stdout (from) => { sess . dcx () . emit_err (errors :: CopyPath :: new (from , to . as_path () , e)) ; } OutFileName :: Real (path) if let Err (e) = fs :: copy (from , path) => { sess . dcx () . emit_err (errors :: CopyPath :: new (from , path , e)) ; } _ => { } } ; let copy_if_one_unit = | output_type : OutputType , keep_numbered : bool | { if let [module] = & compiled_modules . modules [..] { let path = crate_output . temp_path_for_cgu (output_type , & module . name , sess . invocation_temp . as_deref () ,) ; let output = crate_output . path (output_type) ; if ! output_type . is_text_output () && output . is_tty () { sess . dcx () . emit_err (errors :: BinaryOutputToTty { shorthand : output_type . shorthand () }) ; } else { copy_gracefully (& path , & output) ; } if ! sess . opts . cg . save_temps && ! keep_numbered { ensure_removed (sess . dcx () , & path) ; } } else { if crate_output . outputs . contains_explicit_name (& output_type) { sess . dcx () . emit_warn (errors :: IgnoringEmitPath { extension : output_type . extension () }) ; } else if crate_output . single_output_file . is_some () { sess . dcx () . emit_warn (errors :: IgnoringOutput { extension : output_type . extension () }) ; } else { } } } ; for output_type in crate_output . outputs . keys () { match * output_type { OutputType :: Bitcode => { user_wants_bitcode = true ; copy_if_one_unit (OutputType :: Bitcode , true) ; } OutputType :: ThinLinkBitcode => { copy_if_one_unit (OutputType :: ThinLinkBitcode , false) ; } OutputType :: LlvmAssembly => { copy_if_one_unit (OutputType :: LlvmAssembly , false) ; } OutputType :: Assembly => { copy_if_one_unit (OutputType :: Assembly , false) ; } OutputType :: Object => { user_wants_objects = true ; copy_if_one_unit (OutputType :: Object , true) ; } OutputType :: Mir | OutputType :: Metadata | OutputType :: Exe | OutputType :: DepInfo => { } } } if ! sess . opts . cg . save_temps { let needs_crate_object = crate_output . outputs . contains_key (& OutputType :: Exe) ; let keep_numbered_bitcode = user_wants_bitcode && sess . codegen_units () . as_usize () > 1 ; let keep_numbered_objects = needs_crate_object || (user_wants_objects && sess . codegen_units () . as_usize () > 1) ; for module in compiled_modules . modules . iter () { if ! keep_numbered_objects { if let Some (ref path) = module . object { ensure_removed (sess . dcx () , path) ; } if let Some (ref path) = module . dwarf_object { ensure_removed (sess . dcx () , path) ; } } if let Some (ref path) = module . bytecode { if ! keep_numbered_bitcode { ensure_removed (sess . dcx () , path) ; } } } if ! user_wants_bitcode && let Some (ref allocator_module) = compiled_modules . allocator_module && let Some (ref path) = allocator_module . bytecode { ensure_removed (sess . dcx () , path) ; } } if sess . opts . json_artifact_notifications { if let [module] = & compiled_modules . modules [..] { module . for_each_output (| _path , ty | { if sess . opts . output_types . contains_key (& ty) { let descr = ty . shorthand () ; let path = crate_output . path (ty) ; sess . dcx () . emit_artifact_notification (path . as_path () , descr) ; } }) ; } else { for module in & compiled_modules . modules { module . for_each_output (| path , ty | { if sess . opts . output_types . contains_key (& ty) { let descr = ty . shorthand () ; sess . dcx () . emit_artifact_notification (& path , descr) ; } }) ; } } } }
}
mkitem!{mkenum!{pub (crate) enum WorkItem < B : WriteBackendMethods > { #[doc = " Optimize a newly codegened, totally unoptimized module."] Optimize (ModuleCodegen < B :: Module >) , #[doc = " Copy the post-LTO artifacts from the incremental cache to the output"] #[doc = " directory."] CopyPostLtoArtifacts (CachedModuleCodegen) , #[doc = " Performs fat LTO on the given module."] FatLto { exported_symbols_for_lto : Arc < Vec < String > > , each_linked_rlib_for_lto : Vec < PathBuf > , needs_fat_lto : Vec < FatLtoInput < B > > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > , } , #[doc = " Performs thin-LTO on the given module."] ThinLto (lto :: ThinModule < B >) , }}}
mkitem!{mkimpl!{impl < B : WriteBackendMethods > WorkItem < B > { #[doc = " Generate a short description of this work item suitable for use as a thread name."] fn short_description (& self) -> String { #[cfg (not (windows))] fn desc (short : & str , _long : & str , name : & str) -> String { assert_eq ! (short . len () , 3) ; let name = if let Some (index) = name . find ("-cgu.") { & name [index + 1 ..] } else { name } ; format ! ("{short} {name}") } #[cfg (windows)] fn desc (_short : & str , long : & str , name : & str) -> String { format ! ("{long} {name}") } match self { WorkItem :: Optimize (m) => desc ("opt" , "optimize module" , & m . name) , WorkItem :: CopyPostLtoArtifacts (m) => desc ("cpy" , "copy LTO artifacts for" , & m . name) , WorkItem :: FatLto { .. } => desc ("lto" , "fat LTO module" , "everything") , WorkItem :: ThinLto (m) => desc ("lto" , "thin-LTO module" , m . name ()) , } } }}}
mkitem!{mkenum!{#[doc = " A result produced by the backend."] pub (crate) enum WorkItemResult < B : WriteBackendMethods > { #[doc = " The backend has finished compiling a CGU, nothing more required."] Finished (CompiledModule) , #[doc = " The backend has finished compiling a CGU, which now needs to go through"] #[doc = " fat LTO."] NeedsFatLto (FatLtoInput < B >) , #[doc = " The backend has finished compiling a CGU, which now needs to go through"] #[doc = " thin LTO."] NeedsThinLto (String , B :: ThinBuffer) , }}}
mkitem!{mkenum!{pub enum FatLtoInput < B : WriteBackendMethods > { Serialized { name : String , buffer : SerializedModule < B :: ModuleBuffer > } , InMemory (ModuleCodegen < B :: Module >) , }}}
mkitem!{mkenum!{#[doc = " Actual LTO type we end up choosing based on multiple factors."] pub (crate) enum ComputedLtoType { No , Thin , Fat , }}}

macro_rules! compute_per_cgu_lto_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_per_cgu_lto_type in module {}", module_path!());
    };
}

mkfn!{
    compute_per_cgu_lto_type_introspect!();
    pub (crate) fn compute_per_cgu_lto_type (sess_lto : & Lto , opts : & config :: Options , sess_crate_types : & [CrateType] , module_kind : ModuleKind ,) -> ComputedLtoType { let linker_does_lto = opts . cg . linker_plugin_lto . enabled () ; let is_allocator = module_kind == ModuleKind :: Allocator ; let is_rlib = matches ! (sess_crate_types , [CrateType :: Rlib]) ; match sess_lto { Lto :: ThinLocal if ! linker_does_lto && ! is_allocator => ComputedLtoType :: Thin , Lto :: Thin if ! linker_does_lto && ! is_rlib => ComputedLtoType :: Thin , Lto :: Fat if ! is_rlib => ComputedLtoType :: Fat , _ => ComputedLtoType :: No , } }
}

macro_rules! execute_optimize_work_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function execute_optimize_work_item in module {}", module_path!());
    };
}

mkfn!{
    execute_optimize_work_item_introspect!();
    fn execute_optimize_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , mut module : ModuleCodegen < B :: Module > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_optimize" , & * module . name) ; let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let module_config = match module . kind { ModuleKind :: Regular => & cgcx . module_config , ModuleKind :: Allocator => & cgcx . allocator_config , } ; B :: optimize (cgcx , dcx , & mut module , module_config) ; let lto_type = compute_per_cgu_lto_type (& cgcx . lto , & cgcx . opts , & cgcx . crate_types , module . kind) ; let bitcode = if module_config . emit_pre_lto_bc { let filename = pre_lto_bitcode_filename (& module . name) ; cgcx . incr_comp_session_dir . as_ref () . map (| path | path . join (& filename)) } else { None } ; match lto_type { ComputedLtoType :: No => { let module = B :: codegen (cgcx , module , module_config) ; WorkItemResult :: Finished (module) } ComputedLtoType :: Thin => { let (name , thin_buffer) = B :: prepare_thin (module) ; if let Some (path) = bitcode { fs :: write (& path , thin_buffer . data ()) . unwrap_or_else (| e | { panic ! ("Error writing pre-lto-bitcode file `{}`: {}" , path . display () , e) ; }) ; } WorkItemResult :: NeedsThinLto (name , thin_buffer) } ComputedLtoType :: Fat => match bitcode { Some (path) => { let (name , buffer) = B :: serialize_module (module) ; fs :: write (& path , buffer . data ()) . unwrap_or_else (| e | { panic ! ("Error writing pre-lto-bitcode file `{}`: {}" , path . display () , e) ; }) ; WorkItemResult :: NeedsFatLto (FatLtoInput :: Serialized { name , buffer : SerializedModule :: Local (buffer) , }) } None => WorkItemResult :: NeedsFatLto (FatLtoInput :: InMemory (module)) , } , } }
}

macro_rules! execute_copy_from_cache_work_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function execute_copy_from_cache_work_item in module {}", module_path!());
    };
}

mkfn!{
    execute_copy_from_cache_work_item_introspect!();
    fn execute_copy_from_cache_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , module : CachedModuleCodegen ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_copy_artifacts_from_incr_cache" , & * module . name) ; let incr_comp_session_dir = cgcx . incr_comp_session_dir . as_ref () . unwrap () ; let mut links_from_incr_cache = Vec :: new () ; let mut load_from_incr_comp_dir = | output_path : PathBuf , saved_path : & str | { let source_file = in_incr_comp_dir (incr_comp_session_dir , saved_path) ; debug ! ("copying preexisting module `{}` from {:?} to {}" , module . name , source_file , output_path . display ()) ; match link_or_copy (& source_file , & output_path) { Ok (_) => { links_from_incr_cache . push (source_file) ; Some (output_path) } Err (error) => { cgcx . create_dcx () . handle () . emit_err (errors :: CopyPathBuf { source_file , output_path , error , }) ; None } } } ; let dwarf_object = module . source . saved_files . get ("dwo") . as_ref () . and_then (| saved_dwarf_object_file | { let dwarf_obj_out = cgcx . output_filenames . split_dwarf_path (cgcx . split_debuginfo , cgcx . split_dwarf_kind , & module . name , cgcx . invocation_temp . as_deref () ,) . expect ("saved dwarf object in work product but `split_dwarf_path` returned `None`" ,) ; load_from_incr_comp_dir (dwarf_obj_out , saved_dwarf_object_file) }) ; let mut load_from_incr_cache = | perform , output_type : OutputType | { if perform { let saved_file = module . source . saved_files . get (output_type . extension ()) ? ; let output_path = cgcx . output_filenames . temp_path_for_cgu (output_type , & module . name , cgcx . invocation_temp . as_deref () ,) ; load_from_incr_comp_dir (output_path , & saved_file) } else { None } } ; let module_config = & cgcx . module_config ; let should_emit_obj = module_config . emit_obj != EmitObj :: None ; let assembly = load_from_incr_cache (module_config . emit_asm , OutputType :: Assembly) ; let llvm_ir = load_from_incr_cache (module_config . emit_ir , OutputType :: LlvmAssembly) ; let bytecode = load_from_incr_cache (module_config . emit_bc , OutputType :: Bitcode) ; let object = load_from_incr_cache (should_emit_obj , OutputType :: Object) ; if should_emit_obj && object . is_none () { cgcx . create_dcx () . handle () . emit_fatal (errors :: NoSavedObjectFile { cgu_name : & module . name }) } WorkItemResult :: Finished (CompiledModule { links_from_incr_cache , kind : ModuleKind :: Regular , name : module . name , object , dwarf_object , bytecode , assembly , llvm_ir , }) }
}

macro_rules! execute_fat_lto_work_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function execute_fat_lto_work_item in module {}", module_path!());
    };
}

mkfn!{
    execute_fat_lto_work_item_introspect!();
    fn execute_fat_lto_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , mut needs_fat_lto : Vec < FatLtoInput < B > > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_perform_lto" , "everything") ; for (module , wp) in import_only_modules { needs_fat_lto . push (FatLtoInput :: Serialized { name : wp . cgu_name , buffer : module }) } let module = B :: run_and_optimize_fat_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , needs_fat_lto ,) ; let module = B :: codegen (cgcx , module , & cgcx . module_config) ; WorkItemResult :: Finished (module) }
}

macro_rules! execute_thin_lto_work_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function execute_thin_lto_work_item in module {}", module_path!());
    };
}

mkfn!{
    execute_thin_lto_work_item_introspect!();
    fn execute_thin_lto_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , module : lto :: ThinModule < B > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_perform_lto" , module . name ()) ; let module = B :: optimize_thin (cgcx , module) ; let module = B :: codegen (cgcx , module , & cgcx . module_config) ; WorkItemResult :: Finished (module) }
}
mkitem!{mkenum!{#[doc = " Messages sent to the coordinator."] pub (crate) enum Message < B : WriteBackendMethods > { #[doc = " A jobserver token has become available. Sent from the jobserver helper"] #[doc = " thread."] Token (io :: Result < Acquired >) , #[doc = " The backend has finished processing a work item for a codegen unit."] #[doc = " Sent from a backend worker thread."] WorkItem { result : Result < WorkItemResult < B > , Option < WorkerFatalError > > } , #[doc = " The frontend has finished generating something (backend IR or a"] #[doc = " post-LTO artifact) for a codegen unit, and it should be passed to the"] #[doc = " backend. Sent from the main thread."] CodegenDone { llvm_work_item : WorkItem < B > , cost : u64 } , #[doc = " Similar to `CodegenDone`, but for reusing a pre-LTO artifact"] #[doc = " Sent from the main thread."] AddImportOnlyModule { module_data : SerializedModule < B :: ModuleBuffer > , work_product : WorkProduct , } , #[doc = " The frontend has finished generating everything for all codegen units."] #[doc = " Sent from the main thread."] CodegenComplete , #[doc = " Some normal-ish compiler error occurred, and codegen should be wound"] #[doc = " down. Sent from the main thread."] CodegenAborted , }}}
mkitem!{mkstruct!{#[doc = " A message sent from the coordinator thread to the main thread telling it to"] #[doc = " process another codegen unit."] pub struct CguMessage ;}}
mkitem!{mkstruct!{struct Diagnostic { level : Level , messages : Vec < (DiagMessage , Style) > , code : Option < ErrCode > , children : Vec < Subdiagnostic > , args : DiagArgMap , }}}
mkitem!{mkstruct!{pub (crate) struct Subdiagnostic { level : Level , messages : Vec < (DiagMessage , Style) > , }}}
mkitem!{mkenum!{#[derive (PartialEq , Clone , Copy , Debug)] enum MainThreadState { #[doc = " Doing nothing."] Idle , #[doc = " Doing codegen, i.e. MIR-to-LLVM-IR conversion."] Codegenning , #[doc = " Idle, but lending the compiler process's Token to an LLVM thread so it can do useful work."] Lending , }}}

macro_rules! start_executing_work_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function start_executing_work in module {}", module_path!());
    };
}

mkfn!{
    start_executing_work_introspect!();
    fn start_executing_work < B : ExtraBackendMethods > (backend : B , tcx : TyCtxt < '_ > , crate_info : & CrateInfo , shared_emitter : SharedEmitter , codegen_worker_send : Sender < CguMessage > , coordinator_receive : Receiver < Message < B > > , regular_config : Arc < ModuleConfig > , allocator_config : Arc < ModuleConfig > , allocator_module : Option < ModuleCodegen < B :: Module > > , tx_to_llvm_workers : Sender < Message < B > > ,) -> thread :: JoinHandle < Result < CompiledModules , () > > { let coordinator_send = tx_to_llvm_workers ; let sess = tcx . sess ; let mut each_linked_rlib_for_lto = Vec :: new () ; let mut each_linked_rlib_file_for_lto = Vec :: new () ; drop (link :: each_linked_rlib (crate_info , None , & mut | cnum , path | { if link :: ignored_for_lto (sess , crate_info , cnum) { return ; } each_linked_rlib_for_lto . push (cnum) ; each_linked_rlib_file_for_lto . push (path . to_path_buf ()) ; })) ; let exported_symbols_for_lto = Arc :: new (lto :: exported_symbols_for_lto (tcx , & each_linked_rlib_for_lto)) ; let coordinator_send2 = coordinator_send . clone () ; let helper = jobserver :: client () . into_helper_thread (move | token | { drop (coordinator_send2 . send (Message :: Token :: < B > (token))) ; }) . expect ("failed to spawn helper thread") ; let ol = if tcx . sess . opts . unstable_opts . no_codegen || ! tcx . sess . opts . output_types . should_codegen () { config :: OptLevel :: No } else { tcx . backend_optimization_level (()) } ; let backend_features = tcx . global_backend_features (()) ; let remark_dir = if let Some (ref dir) = sess . opts . unstable_opts . remark_dir { let result = fs :: create_dir_all (dir) . and_then (| _ | dir . canonicalize ()) ; match result { Ok (dir) => Some (dir) , Err (error) => sess . dcx () . emit_fatal (ErrorCreatingRemarkDir { error }) , } } else { None } ; let cgcx = CodegenContext :: < B > { crate_types : tcx . crate_types () . to_vec () , lto : sess . lto () , fewer_names : sess . fewer_names () , save_temps : sess . opts . cg . save_temps , time_trace : sess . opts . unstable_opts . llvm_time_trace , opts : Arc :: new (sess . opts . clone ()) , prof : sess . prof . clone () , remark : sess . opts . cg . remark . clone () , remark_dir , incr_comp_session_dir : sess . incr_comp_session_dir_opt () . map (| r | r . clone ()) , expanded_args : tcx . sess . expanded_args . clone () , diag_emitter : shared_emitter . clone () , output_filenames : Arc :: clone (tcx . output_filenames (())) , module_config : regular_config , allocator_config , tm_factory : backend . target_machine_factory (tcx . sess , ol , backend_features) , msvc_imps_needed : msvc_imps_needed (tcx) , is_pe_coff : tcx . sess . target . is_like_windows , target_can_use_split_dwarf : tcx . sess . target_can_use_split_dwarf () , target_arch : tcx . sess . target . arch . to_string () , target_is_like_darwin : tcx . sess . target . is_like_darwin , target_is_like_aix : tcx . sess . target . is_like_aix , split_debuginfo : tcx . sess . split_debuginfo () , split_dwarf_kind : tcx . sess . opts . unstable_opts . split_dwarf_kind , parallel : backend . supports_parallel () && ! sess . opts . unstable_opts . no_parallel_backend , pointer_size : tcx . data_layout . pointer_size () , invocation_temp : sess . invocation_temp . clone () , } ; return B :: spawn_named_thread (cgcx . time_trace , "coordinator" . to_string () , move | | { let mut compiled_modules = vec ! [] ; let mut needs_fat_lto = Vec :: new () ; let mut needs_thin_lto = Vec :: new () ; let mut lto_import_only_modules = Vec :: new () ; let mut started_lto = false ; #[doc = " Possible state transitions:"] #[doc = " - Ongoing -> Completed"] #[doc = " - Ongoing -> Aborted"] #[doc = " - Completed -> Aborted"] #[derive (Debug , PartialEq)] enum CodegenState { Ongoing , Completed , Aborted , } use CodegenState :: * ; let mut codegen_state = Ongoing ; let mut work_items = Vec :: < (WorkItem < B > , u64) > :: new () ; let mut tokens = Vec :: new () ; let mut main_thread_state = MainThreadState :: Idle ; let mut running_with_own_token = 0 ; let running_with_any_token = | main_thread_state , running_with_own_token | { running_with_own_token + if main_thread_state == MainThreadState :: Lending { 1 } else { 0 } } ; let mut llvm_start_time : Option < VerboseTimingGuard < '_ > > = None ; let compiled_allocator_module = allocator_module . and_then (| allocator_module | { match execute_optimize_work_item (& cgcx , allocator_module) { WorkItemResult :: Finished (compiled_module) => return Some (compiled_module) , WorkItemResult :: NeedsFatLto (fat_lto_input) => needs_fat_lto . push (fat_lto_input) , WorkItemResult :: NeedsThinLto (name , thin_buffer) => { needs_thin_lto . push ((name , thin_buffer)) } } None }) ; loop { if codegen_state == Ongoing { if main_thread_state == MainThreadState :: Idle { let extra_tokens = tokens . len () . checked_sub (running_with_own_token) . unwrap () ; let additional_running = std :: cmp :: min (extra_tokens , work_items . len ()) ; let anticipated_running = running_with_own_token + additional_running + 1 ; if ! queue_full_enough (work_items . len () , anticipated_running) { if codegen_worker_send . send (CguMessage) . is_err () { panic ! ("Could not send CguMessage to main thread") } main_thread_state = MainThreadState :: Codegenning ; } else { let (item , _) = work_items . pop () . expect ("queue empty - queue_full_enough() broken?") ; main_thread_state = MainThreadState :: Lending ; spawn_work (& cgcx , coordinator_send . clone () , & mut llvm_start_time , item) ; } } } else if codegen_state == Completed { if running_with_any_token (main_thread_state , running_with_own_token) == 0 && work_items . is_empty () { if needs_fat_lto . is_empty () && needs_thin_lto . is_empty () && lto_import_only_modules . is_empty () { break ; } assert ! (! started_lto) ; started_lto = true ; let needs_fat_lto = mem :: take (& mut needs_fat_lto) ; let needs_thin_lto = mem :: take (& mut needs_thin_lto) ; let import_only_modules = mem :: take (& mut lto_import_only_modules) ; let each_linked_rlib_file_for_lto = mem :: take (& mut each_linked_rlib_file_for_lto) ; check_lto_allowed (& cgcx) ; if ! needs_fat_lto . is_empty () { assert ! (needs_thin_lto . is_empty ()) ; work_items . push ((WorkItem :: FatLto { exported_symbols_for_lto : Arc :: clone (& exported_symbols_for_lto) , each_linked_rlib_for_lto : each_linked_rlib_file_for_lto , needs_fat_lto , import_only_modules , } , 0 ,)) ; if cgcx . parallel { helper . request_token () ; } } else { for (work , cost) in generate_thin_lto_work (& cgcx , & exported_symbols_for_lto , & each_linked_rlib_file_for_lto , needs_thin_lto , import_only_modules ,) { let insertion_index = work_items . binary_search_by_key (& cost , | & (_ , cost) | cost) . unwrap_or_else (| e | e) ; work_items . insert (insertion_index , (work , cost)) ; if cgcx . parallel { helper . request_token () ; } } } } match main_thread_state { MainThreadState :: Idle => { if let Some ((item , _)) = work_items . pop () { main_thread_state = MainThreadState :: Lending ; spawn_work (& cgcx , coordinator_send . clone () , & mut llvm_start_time , item) ; } else { assert ! (running_with_own_token > 0) ; running_with_own_token -= 1 ; main_thread_state = MainThreadState :: Lending ; } } MainThreadState :: Codegenning => bug ! ("codegen worker should not be codegenning after \
                              codegen was already completed") , MainThreadState :: Lending => { } } } else { assert ! (codegen_state == Aborted) ; if running_with_any_token (main_thread_state , running_with_own_token) == 0 { break ; } } if codegen_state != Aborted { while running_with_own_token < tokens . len () && let Some ((item , _)) = work_items . pop () { spawn_work (& cgcx , coordinator_send . clone () , & mut llvm_start_time , item) ; running_with_own_token += 1 ; } } tokens . truncate (running_with_own_token) ; match coordinator_receive . recv () . unwrap () { Message :: Token (token) => { match token { Ok (token) => { tokens . push (token) ; if main_thread_state == MainThreadState :: Lending { main_thread_state = MainThreadState :: Idle ; running_with_own_token += 1 ; } } Err (e) => { let msg = & format ! ("failed to acquire jobserver token: {e}") ; shared_emitter . fatal (msg) ; codegen_state = Aborted ; } } } Message :: CodegenDone { llvm_work_item , cost } => { let insertion_index = work_items . binary_search_by_key (& cost , | & (_ , cost) | cost) ; let insertion_index = match insertion_index { Ok (idx) | Err (idx) => idx , } ; work_items . insert (insertion_index , (llvm_work_item , cost)) ; if cgcx . parallel { helper . request_token () ; } assert_eq ! (main_thread_state , MainThreadState :: Codegenning) ; main_thread_state = MainThreadState :: Idle ; } Message :: CodegenComplete => { if codegen_state != Aborted { codegen_state = Completed ; } assert_eq ! (main_thread_state , MainThreadState :: Codegenning) ; main_thread_state = MainThreadState :: Idle ; } Message :: CodegenAborted => { codegen_state = Aborted ; } Message :: WorkItem { result } => { if main_thread_state == MainThreadState :: Lending { main_thread_state = MainThreadState :: Idle ; } else { running_with_own_token -= 1 ; } match result { Ok (WorkItemResult :: Finished (compiled_module)) => { compiled_modules . push (compiled_module) ; } Ok (WorkItemResult :: NeedsFatLto (fat_lto_input)) => { assert ! (! started_lto) ; assert ! (needs_thin_lto . is_empty ()) ; needs_fat_lto . push (fat_lto_input) ; } Ok (WorkItemResult :: NeedsThinLto (name , thin_buffer)) => { assert ! (! started_lto) ; assert ! (needs_fat_lto . is_empty ()) ; needs_thin_lto . push ((name , thin_buffer)) ; } Err (Some (WorkerFatalError)) => { codegen_state = Aborted ; } Err (None) => { bug ! ("worker thread panicked") ; } } } Message :: AddImportOnlyModule { module_data , work_product } => { assert ! (! started_lto) ; assert_eq ! (codegen_state , Ongoing) ; assert_eq ! (main_thread_state , MainThreadState :: Codegenning) ; lto_import_only_modules . push ((module_data , work_product)) ; main_thread_state = MainThreadState :: Idle ; } } } if codegen_state == Aborted { return Err (()) ; } drop (llvm_start_time) ; compiled_modules . sort_by (| a , b | a . name . cmp (& b . name)) ; Ok (CompiledModules { modules : compiled_modules , allocator_module : compiled_allocator_module , }) }) . expect ("failed to spawn coordinator thread") ; fn queue_full_enough (items_in_queue : usize , workers_running : usize) -> bool { let quarter_of_workers = workers_running - 3 * workers_running / 4 ; items_in_queue > 0 && items_in_queue >= quarter_of_workers } }
}
mkitem!{mkstruct!{#[doc = " `FatalError` is explicitly not `Send`."] #[must_use] pub (crate) struct WorkerFatalError ;}}

macro_rules! spawn_work_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_work in module {}", module_path!());
    };
}

mkfn!{
    spawn_work_introspect!();
    fn spawn_work < 'a , B : ExtraBackendMethods > (cgcx : & 'a CodegenContext < B > , coordinator_send : Sender < Message < B > > , llvm_start_time : & mut Option < VerboseTimingGuard < 'a > > , work : WorkItem < B > ,) { if llvm_start_time . is_none () { * llvm_start_time = Some (cgcx . prof . verbose_generic_activity ("LLVM_passes")) ; } let cgcx = cgcx . clone () ; B :: spawn_named_thread (cgcx . time_trace , work . short_description () , move | | { let result = std :: panic :: catch_unwind (AssertUnwindSafe (| | match work { WorkItem :: Optimize (m) => execute_optimize_work_item (& cgcx , m) , WorkItem :: CopyPostLtoArtifacts (m) => execute_copy_from_cache_work_item (& cgcx , m) , WorkItem :: FatLto { exported_symbols_for_lto , each_linked_rlib_for_lto , needs_fat_lto , import_only_modules , } => execute_fat_lto_work_item (& cgcx , & exported_symbols_for_lto , & each_linked_rlib_for_lto , needs_fat_lto , import_only_modules ,) , WorkItem :: ThinLto (m) => execute_thin_lto_work_item (& cgcx , m) , })) ; let msg = match result { Ok (result) => Message :: WorkItem :: < B > { result : Ok (result) } , Err (err) if err . is :: < FatalErrorMarker > () => { Message :: WorkItem :: < B > { result : Err (Some (WorkerFatalError)) } } Err (_) => Message :: WorkItem :: < B > { result : Err (None) } , } ; drop (coordinator_send . send (msg)) ; }) . expect ("failed to spawn work thread") ; }
}
mkitem!{mkenum!{enum SharedEmitterMessage { Diagnostic (Diagnostic) , InlineAsmError (SpanData , String , Level , Option < (String , Vec < InnerSpan >) >) , Fatal (String) , }}}
mkitem!{mkstruct!{#[derive (Clone)] pub struct SharedEmitter { sender : Sender < SharedEmitterMessage > , }}}
mkitem!{mkstruct!{pub struct SharedEmitterMain { receiver : Receiver < SharedEmitterMessage > , }}}
mkitem!{mkimpl!{impl SharedEmitter { fn new () -> (SharedEmitter , SharedEmitterMain) { let (sender , receiver) = channel () ; (SharedEmitter { sender } , SharedEmitterMain { receiver }) } pub fn inline_asm_error (& self , span : SpanData , msg : String , level : Level , source : Option < (String , Vec < InnerSpan >) > ,) { drop (self . sender . send (SharedEmitterMessage :: InlineAsmError (span , msg , level , source))) ; } fn fatal (& self , msg : & str) { drop (self . sender . send (SharedEmitterMessage :: Fatal (msg . to_string ()))) ; } }}}
mkitem!{mkimpl!{impl Emitter for SharedEmitter { fn emit_diagnostic (& mut self , mut diag : rustc_errors :: DiagInner , _registry : & rustc_errors :: registry :: Registry ,) { assert_eq ! (diag . span , MultiSpan :: new ()) ; assert_eq ! (diag . suggestions , Suggestions :: Enabled (vec ! [])) ; assert_eq ! (diag . sort_span , rustc_span :: DUMMY_SP) ; assert_eq ! (diag . is_lint , None) ; let args = mem :: replace (& mut diag . args , DiagArgMap :: default ()) ; drop (self . sender . send (SharedEmitterMessage :: Diagnostic (Diagnostic { level : diag . level () , messages : diag . messages , code : diag . code , children : diag . children . into_iter () . map (| child | Subdiagnostic { level : child . level , messages : child . messages }) . collect () , args , })) ,) ; } fn source_map (& self) -> Option < & SourceMap > { None } fn translator (& self) -> & Translator { panic ! ("shared emitter attempted to translate a diagnostic") ; } }}}
mkitem!{mkimpl!{impl SharedEmitterMain { fn check (& self , sess : & Session , blocking : bool) { loop { let message = if blocking { match self . receiver . recv () { Ok (message) => Ok (message) , Err (_) => Err (()) , } } else { match self . receiver . try_recv () { Ok (message) => Ok (message) , Err (_) => Err (()) , } } ; match message { Ok (SharedEmitterMessage :: Diagnostic (diag)) => { let dcx = sess . dcx () ; let mut d = rustc_errors :: DiagInner :: new_with_messages (diag . level , diag . messages) ; d . code = diag . code ; d . children = diag . children . into_iter () . map (| sub | rustc_errors :: Subdiag { level : sub . level , messages : sub . messages , span : MultiSpan :: new () , }) . collect () ; d . args = diag . args ; dcx . emit_diagnostic (d) ; sess . dcx () . abort_if_errors () ; } Ok (SharedEmitterMessage :: InlineAsmError (span , msg , level , source)) => { assert_matches ! (level , Level :: Error | Level :: Warning | Level :: Note) ; let mut err = Diag :: < () > :: new (sess . dcx () , level , msg) ; if ! span . is_dummy () { err . span (span . span ()) ; } if let Some ((buffer , spans)) = source { let source = sess . source_map () . new_source_file (FileName :: inline_asm_source_code (& buffer) , buffer) ; let spans : Vec < _ > = spans . iter () . map (| sp | { Span :: with_root_ctxt (source . normalized_byte_pos (sp . start as u32) , source . normalized_byte_pos (sp . end as u32) ,) }) . collect () ; err . span_note (spans , "instantiated into assembly here") ; } err . emit () ; } Ok (SharedEmitterMessage :: Fatal (msg)) => { sess . dcx () . fatal (msg) ; } Err (_) => { break ; } } } } }}}
mkitem!{mkstruct!{pub struct Coordinator < B : ExtraBackendMethods > { sender : Sender < Message < B > > , future : Option < thread :: JoinHandle < Result < CompiledModules , () > > > , phantom : PhantomData < B > , }}}
mkitem!{mkimpl!{impl < B : ExtraBackendMethods > Coordinator < B > { fn join (mut self) -> std :: thread :: Result < Result < CompiledModules , () > > { self . future . take () . unwrap () . join () } }}}
mkitem!{mkimpl!{impl < B : ExtraBackendMethods > Drop for Coordinator < B > { fn drop (& mut self) { if let Some (future) = self . future . take () { drop (self . sender . send (Message :: CodegenAborted :: < B >)) ; drop (future . join ()) ; } } }}}
mkitem!{mkstruct!{pub struct OngoingCodegen < B : ExtraBackendMethods > { pub backend : B , pub crate_info : CrateInfo , pub output_filenames : Arc < OutputFilenames > , pub coordinator : Coordinator < B > , pub codegen_worker_receive : Receiver < CguMessage > , pub shared_emitter_main : SharedEmitterMain , }}}
mkitem!{mkimpl!{impl < B : ExtraBackendMethods > OngoingCodegen < B > { pub fn join (self , sess : & Session) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { self . shared_emitter_main . check (sess , true) ; let compiled_modules = sess . time ("join_worker_thread" , | | match self . coordinator . join () { Ok (Ok (compiled_modules)) => compiled_modules , Ok (Err (())) => { sess . dcx () . abort_if_errors () ; panic ! ("expected abort due to worker thread errors") } Err (_) => { bug ! ("panic during codegen/LLVM phase") ; } }) ; sess . dcx () . abort_if_errors () ; let work_products = copy_all_cgu_workproducts_to_incr_comp_cache_dir (sess , & compiled_modules) ; produce_final_output_artifacts (sess , & compiled_modules , & self . output_filenames) ; if sess . codegen_units () . as_usize () == 1 && sess . opts . unstable_opts . time_llvm_passes { self . backend . print_pass_timings () } if sess . print_llvm_stats () { self . backend . print_statistics () } (CodegenResults { crate_info : self . crate_info , modules : compiled_modules . modules , allocator_module : compiled_modules . allocator_module , } , work_products ,) } pub (crate) fn codegen_finished (& self , tcx : TyCtxt < '_ >) { self . wait_for_signal_to_codegen_item () ; self . check_for_errors (tcx . sess) ; drop (self . coordinator . sender . send (Message :: CodegenComplete :: < B >)) ; } pub (crate) fn check_for_errors (& self , sess : & Session) { self . shared_emitter_main . check (sess , false) ; } pub (crate) fn wait_for_signal_to_codegen_item (& self) { match self . codegen_worker_receive . recv () { Ok (CguMessage) => { } Err (_) => { } } } }}}

macro_rules! submit_codegened_module_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function submit_codegened_module_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    submit_codegened_module_to_llvm_introspect!();
    pub (crate) fn submit_codegened_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : ModuleCodegen < B :: Module > , cost : u64 ,) { let llvm_work_item = WorkItem :: Optimize (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost })) ; }
}

macro_rules! submit_post_lto_module_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function submit_post_lto_module_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    submit_post_lto_module_to_llvm_introspect!();
    pub (crate) fn submit_post_lto_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : CachedModuleCodegen ,) { let llvm_work_item = WorkItem :: CopyPostLtoArtifacts (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost : 0 })) ; }
}

macro_rules! submit_pre_lto_module_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function submit_pre_lto_module_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    submit_pre_lto_module_to_llvm_introspect!();
    pub (crate) fn submit_pre_lto_module_to_llvm < B : ExtraBackendMethods > (tcx : TyCtxt < '_ > , coordinator : & Coordinator < B > , module : CachedModuleCodegen ,) { let filename = pre_lto_bitcode_filename (& module . name) ; let bc_path = in_incr_comp_dir_sess (tcx . sess , & filename) ; let file = fs :: File :: open (& bc_path) . unwrap_or_else (| e | panic ! ("failed to open bitcode file `{}`: {}" , bc_path . display () , e)) ; let mmap = unsafe { Mmap :: map (file) . unwrap_or_else (| e | { panic ! ("failed to mmap bitcode file `{}`: {}" , bc_path . display () , e) }) } ; drop (coordinator . sender . send (Message :: AddImportOnlyModule :: < B > { module_data : SerializedModule :: FromUncompressedFile (mmap) , work_product : module . source , })) ; }
}

macro_rules! pre_lto_bitcode_filename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pre_lto_bitcode_filename in module {}", module_path!());
    };
}

mkfn!{
    pre_lto_bitcode_filename_introspect!();
    fn pre_lto_bitcode_filename (module_name : & str) -> String { format ! ("{module_name}.{PRE_LTO_BC_EXT}") }
}

macro_rules! msvc_imps_needed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function msvc_imps_needed in module {}", module_path!());
    };
}

mkfn!{
    msvc_imps_needed_introspect!();
    fn msvc_imps_needed (tcx : TyCtxt < '_ >) -> bool { assert ! (! (tcx . sess . opts . cg . linker_plugin_lto . enabled () && tcx . sess . target . is_like_windows && tcx . sess . opts . cg . prefer_dynamic)) ; let can_have_static_objects = tcx . sess . lto () == Lto :: Thin || tcx . crate_types () . contains (& CrateType :: Rlib) ; tcx . sess . target . is_like_windows && can_have_static_objects && ! tcx . sess . opts . cg . linker_plugin_lto . enabled () }
}