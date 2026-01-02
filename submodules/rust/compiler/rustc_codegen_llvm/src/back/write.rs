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
mkuse!{use std :: ffi :: { CStr , CString } ;}
mkuse!{use std :: io :: { self , Write } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: ptr :: null_mut ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: { fs , slice , str } ;}
mkuse!{use libc :: { c_char , c_int , c_void , size_t } ;}
mkuse!{use llvm :: { LLVMRustLLVMHasZlibCompressionForDebugSymbols , LLVMRustLLVMHasZstdCompressionForDebugSymbols , } ;}
mkuse!{use rustc_codegen_ssa :: back :: link :: ensure_removed ;}
mkuse!{use rustc_codegen_ssa :: back :: versioned_llvm_target ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: { BitcodeSection , CodegenContext , EmitObj , ModuleConfig , TargetMachineFactoryConfig , TargetMachineFactoryFn , } ;}
mkuse!{use rustc_codegen_ssa :: base :: wants_wasm_eh ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_codegen_ssa :: { CompiledModule , ModuleCodegen , ModuleKind } ;}
mkuse!{use rustc_data_structures :: profiling :: SelfProfilerRef ;}
mkuse!{use rustc_data_structures :: small_c_str :: SmallCStr ;}
mkuse!{use rustc_errors :: { DiagCtxtHandle , Level } ;}
mkuse!{use rustc_fs_util :: { link_or_copy , path_to_c_string } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { self , Lto , OutputType , Passes , RemapPathScopeComponents , SplitDwarfKind , SwitchWithOptPath , } ;}
mkuse!{use rustc_span :: { BytePos , InnerSpan , Pos , SpanData , SyntaxContext , sym } ;}
mkuse!{use rustc_target :: spec :: { CodeModel , FloatAbi , RelocModel , SanitizerSet , SplitDebuginfo , TlsModel } ;}
mkuse!{use tracing :: { debug , trace } ;}
mkuse!{use crate :: back :: lto :: ThinBuffer ;}
mkuse!{use crate :: back :: owned_target_machine :: OwnedTargetMachine ;}
mkuse!{use crate :: back :: profiling :: { LlvmSelfProfiler , selfprofile_after_pass_callback , selfprofile_before_pass_callback , } ;}
mkuse!{use crate :: common :: AsCCharPtr ;}
mkuse!{use crate :: errors :: { CopyBitcode , FromLlvmDiag , FromLlvmOptimizationDiag , LlvmError , UnknownCompression , WithLlvmError , WriteBytecode , } ;}
mkuse!{use crate :: llvm :: diagnostic :: OptimizationDiagnosticKind :: * ;}
mkuse!{use crate :: llvm :: { self , DiagnosticInfo } ;}
mkuse!{use crate :: type_ :: Type ;}
mkuse!{use crate :: { LlvmCodegenBackend , ModuleLlvm , base , common , llvm_util } ;}

macro_rules! llvm_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_err in module {}", module_path!());
    };
}

mkfn!{
    llvm_err_introspect!();
    pub (crate) fn llvm_err < 'a > (dcx : DiagCtxtHandle < '_ > , err : LlvmError < 'a >) -> ! { match llvm :: last_error () { Some (llvm_err) => dcx . emit_fatal (WithLlvmError (err , llvm_err)) , None => dcx . emit_fatal (err) , } }
}

macro_rules! write_output_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_output_file in module {}", module_path!());
    };
}

mkfn!{
    write_output_file_introspect!();
    fn write_output_file < 'll > (dcx : DiagCtxtHandle < '_ > , target : & 'll llvm :: TargetMachine , no_builtins : bool , m : & 'll llvm :: Module , output : & Path , dwo_output : Option < & Path > , file_type : llvm :: FileType , self_profiler_ref : & SelfProfilerRef , verify_llvm_ir : bool ,) { debug ! ("write_output_file output={:?} dwo_output={:?}" , output , dwo_output) ; let output_c = path_to_c_string (output) ; let dwo_output_c ; let dwo_output_ptr = if let Some (dwo_output) = dwo_output { dwo_output_c = path_to_c_string (dwo_output) ; dwo_output_c . as_ptr () } else { std :: ptr :: null () } ; let result = unsafe { let pm = llvm :: LLVMCreatePassManager () ; llvm :: LLVMAddAnalysisPasses (target , pm) ; llvm :: LLVMRustAddLibraryInfo (pm , m , no_builtins) ; llvm :: LLVMRustWriteOutputFile (target , pm , m , output_c . as_ptr () , dwo_output_ptr , file_type , verify_llvm_ir ,) } ; if result == llvm :: LLVMRustResult :: Success { let artifact_kind = match file_type { llvm :: FileType :: ObjectFile => "object_file" , llvm :: FileType :: AssemblyFile => "assembly_file" , } ; record_artifact_size (self_profiler_ref , artifact_kind , output) ; if let Some (dwo_file) = dwo_output { record_artifact_size (self_profiler_ref , "dwo_file" , dwo_file) ; } } result . into_result () . unwrap_or_else (| () | llvm_err (dcx , LlvmError :: WriteOutput { path : output })) }
}

macro_rules! create_informational_target_machine_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_informational_target_machine in module {}", module_path!());
    };
}

mkfn!{
    create_informational_target_machine_introspect!();
    pub (crate) fn create_informational_target_machine (sess : & Session , only_base_features : bool ,) -> OwnedTargetMachine { let config = TargetMachineFactoryConfig { split_dwarf_file : None , output_obj_file : None } ; let features = llvm_util :: global_llvm_features (sess , false , only_base_features) ; target_machine_factory (sess , config :: OptLevel :: No , & features) (config) . unwrap_or_else (| err | llvm_err (sess . dcx () , err)) }
}

macro_rules! create_target_machine_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_target_machine in module {}", module_path!());
    };
}

mkfn!{
    create_target_machine_introspect!();
    pub (crate) fn create_target_machine (tcx : TyCtxt < '_ > , mod_name : & str) -> OwnedTargetMachine { let split_dwarf_file = if tcx . sess . target_can_use_split_dwarf () { tcx . output_filenames (()) . split_dwarf_path (tcx . sess . split_debuginfo () , tcx . sess . opts . unstable_opts . split_dwarf_kind , mod_name , tcx . sess . invocation_temp . as_deref () ,) } else { None } ; let output_obj_file = Some (tcx . output_filenames (()) . temp_path_for_cgu (OutputType :: Object , mod_name , tcx . sess . invocation_temp . as_deref () ,)) ; let config = TargetMachineFactoryConfig { split_dwarf_file , output_obj_file } ; target_machine_factory (tcx . sess , tcx . backend_optimization_level (()) , tcx . global_backend_features (()) ,) (config) . unwrap_or_else (| err | llvm_err (tcx . dcx () , err)) }
}

macro_rules! to_llvm_opt_settings_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_llvm_opt_settings in module {}", module_path!());
    };
}

mkfn!{
    to_llvm_opt_settings_introspect!();
    fn to_llvm_opt_settings (cfg : config :: OptLevel) -> (llvm :: CodeGenOptLevel , llvm :: CodeGenOptSize) { use self :: config :: OptLevel :: * ; match cfg { No => (llvm :: CodeGenOptLevel :: None , llvm :: CodeGenOptSizeNone) , Less => (llvm :: CodeGenOptLevel :: Less , llvm :: CodeGenOptSizeNone) , More => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeNone) , Aggressive => (llvm :: CodeGenOptLevel :: Aggressive , llvm :: CodeGenOptSizeNone) , Size => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeDefault) , SizeMin => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeAggressive) , } }
}

macro_rules! to_pass_builder_opt_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_pass_builder_opt_level in module {}", module_path!());
    };
}

mkfn!{
    to_pass_builder_opt_level_introspect!();
    fn to_pass_builder_opt_level (cfg : config :: OptLevel) -> llvm :: PassBuilderOptLevel { use config :: OptLevel :: * ; match cfg { No => llvm :: PassBuilderOptLevel :: O0 , Less => llvm :: PassBuilderOptLevel :: O1 , More => llvm :: PassBuilderOptLevel :: O2 , Aggressive => llvm :: PassBuilderOptLevel :: O3 , Size => llvm :: PassBuilderOptLevel :: Os , SizeMin => llvm :: PassBuilderOptLevel :: Oz , } }
}

macro_rules! to_llvm_relocation_model_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_llvm_relocation_model in module {}", module_path!());
    };
}

mkfn!{
    to_llvm_relocation_model_introspect!();
    fn to_llvm_relocation_model (relocation_model : RelocModel) -> llvm :: RelocModel { match relocation_model { RelocModel :: Static => llvm :: RelocModel :: Static , RelocModel :: Pic | RelocModel :: Pie => llvm :: RelocModel :: PIC , RelocModel :: DynamicNoPic => llvm :: RelocModel :: DynamicNoPic , RelocModel :: Ropi => llvm :: RelocModel :: ROPI , RelocModel :: Rwpi => llvm :: RelocModel :: RWPI , RelocModel :: RopiRwpi => llvm :: RelocModel :: ROPI_RWPI , } }
}

macro_rules! to_llvm_code_model_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_llvm_code_model in module {}", module_path!());
    };
}

mkfn!{
    to_llvm_code_model_introspect!();
    pub (crate) fn to_llvm_code_model (code_model : Option < CodeModel >) -> llvm :: CodeModel { match code_model { Some (CodeModel :: Tiny) => llvm :: CodeModel :: Tiny , Some (CodeModel :: Small) => llvm :: CodeModel :: Small , Some (CodeModel :: Kernel) => llvm :: CodeModel :: Kernel , Some (CodeModel :: Medium) => llvm :: CodeModel :: Medium , Some (CodeModel :: Large) => llvm :: CodeModel :: Large , None => llvm :: CodeModel :: None , } }
}

macro_rules! to_llvm_float_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_llvm_float_abi in module {}", module_path!());
    };
}

mkfn!{
    to_llvm_float_abi_introspect!();
    fn to_llvm_float_abi (float_abi : Option < FloatAbi >) -> llvm :: FloatAbi { match float_abi { None => llvm :: FloatAbi :: Default , Some (FloatAbi :: Soft) => llvm :: FloatAbi :: Soft , Some (FloatAbi :: Hard) => llvm :: FloatAbi :: Hard , } }
}

macro_rules! target_machine_factory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_machine_factory in module {}", module_path!());
    };
}

mkfn!{
    target_machine_factory_introspect!();
    pub (crate) fn target_machine_factory (sess : & Session , optlvl : config :: OptLevel , target_features : & [String] ,) -> TargetMachineFactoryFn < LlvmCodegenBackend > { let reloc_model = to_llvm_relocation_model (sess . relocation_model ()) ; let (opt_level , _) = to_llvm_opt_settings (optlvl) ; let float_abi = if sess . target . arch == "arm" && sess . opts . cg . soft_float { llvm :: FloatAbi :: Soft } else { to_llvm_float_abi (sess . target . llvm_floatabi) } ; let ffunction_sections = sess . opts . unstable_opts . function_sections . unwrap_or (sess . target . function_sections) ; let fdata_sections = ffunction_sections ; let funique_section_names = ! sess . opts . unstable_opts . no_unique_section_names ; let code_model = to_llvm_code_model (sess . code_model ()) ; let mut singlethread = sess . target . singlethread ; if singlethread && sess . target . is_like_wasm && sess . target_features . contains (& sym :: atomics) { singlethread = false ; } let triple = SmallCStr :: new (& versioned_llvm_target (sess)) ; let cpu = SmallCStr :: new (llvm_util :: target_cpu (sess)) ; let features = CString :: new (target_features . join (",")) . unwrap () ; let abi = SmallCStr :: new (& sess . target . llvm_abiname) ; let trap_unreachable = sess . opts . unstable_opts . trap_unreachable . unwrap_or (sess . target . trap_unreachable) ; let emit_stack_size_section = sess . opts . unstable_opts . emit_stack_sizes ; let verbose_asm = sess . opts . unstable_opts . verbose_asm ; let relax_elf_relocations = sess . opts . unstable_opts . relax_elf_relocations . unwrap_or (sess . target . relax_elf_relocations) ; let use_init_array = ! sess . opts . unstable_opts . use_ctors_section . unwrap_or (sess . target . use_ctors_section) ; let path_mapping = sess . source_map () . path_mapping () . clone () ; let use_emulated_tls = matches ! (sess . tls_model () , TlsModel :: Emulated) ; let args_cstr_buff = { let mut args_cstr_buff : Vec < u8 > = Vec :: new () ; let exe_path = std :: env :: current_exe () . unwrap_or_default () ; let exe_path_str = exe_path . into_os_string () . into_string () . unwrap_or_default () ; args_cstr_buff . extend_from_slice (exe_path_str . as_bytes ()) ; args_cstr_buff . push (0) ; for arg in sess . expanded_args . iter () { args_cstr_buff . extend_from_slice (arg . as_bytes ()) ; args_cstr_buff . push (0) ; } args_cstr_buff } ; let debuginfo_compression = sess . opts . debuginfo_compression . to_string () ; match sess . opts . debuginfo_compression { rustc_session :: config :: DebugInfoCompression :: Zlib => { if ! unsafe { LLVMRustLLVMHasZlibCompressionForDebugSymbols () } { sess . dcx () . emit_warn (UnknownCompression { algorithm : "zlib" }) ; } } rustc_session :: config :: DebugInfoCompression :: Zstd => { if ! unsafe { LLVMRustLLVMHasZstdCompressionForDebugSymbols () } { sess . dcx () . emit_warn (UnknownCompression { algorithm : "zstd" }) ; } } rustc_session :: config :: DebugInfoCompression :: None => { } } ; let debuginfo_compression = SmallCStr :: new (& debuginfo_compression) ; let file_name_display_preference = sess . filename_display_preference (RemapPathScopeComponents :: DEBUGINFO) ; let use_wasm_eh = wants_wasm_eh (sess) ; Arc :: new (move | config : TargetMachineFactoryConfig | { let path_to_cstring_helper = | path : Option < PathBuf > | -> CString { let path = path . unwrap_or_default () ; let path = path_mapping . to_real_filename (path) . to_string_lossy (file_name_display_preference) . into_owned () ; CString :: new (path) . unwrap () } ; let split_dwarf_file = path_to_cstring_helper (config . split_dwarf_file) ; let output_obj_file = path_to_cstring_helper (config . output_obj_file) ; OwnedTargetMachine :: new (& triple , & cpu , & features , & abi , code_model , reloc_model , opt_level , float_abi , ffunction_sections , fdata_sections , funique_section_names , trap_unreachable , singlethread , verbose_asm , emit_stack_size_section , relax_elf_relocations , use_init_array , & split_dwarf_file , & output_obj_file , & debuginfo_compression , use_emulated_tls , & args_cstr_buff , use_wasm_eh ,) }) }
}

macro_rules! save_temp_bitcode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function save_temp_bitcode in module {}", module_path!());
    };
}

mkfn!{
    save_temp_bitcode_introspect!();
    pub (crate) fn save_temp_bitcode (cgcx : & CodegenContext < LlvmCodegenBackend > , module : & ModuleCodegen < ModuleLlvm > , name : & str ,) { if ! cgcx . save_temps { return ; } let ext = format ! ("{name}.bc") ; let path = cgcx . output_filenames . temp_path_ext_for_cgu (& ext , & module . name , cgcx . invocation_temp . as_deref () ,) ; write_bitcode_to_file (module , & path) }
}

macro_rules! write_bitcode_to_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_bitcode_to_file in module {}", module_path!());
    };
}

mkfn!{
    write_bitcode_to_file_introspect!();
    fn write_bitcode_to_file (module : & ModuleCodegen < ModuleLlvm > , path : & Path) { unsafe { let path = path_to_c_string (& path) ; let llmod = module . module_llvm . llmod () ; llvm :: LLVMWriteBitcodeToFile (llmod , path . as_ptr ()) ; } }
}
mkitem!{mkenum!{# [doc = " In what context is a dignostic handler being attached to a codegen unit?"] pub (crate) enum CodegenDiagnosticsStage { # [doc = " Prelink optimization stage."] Opt , # [doc = " LTO/ThinLTO postlink optimization stage."] LTO , # [doc = " Code generation."] Codegen , }}}
mkitem!{mkstruct!{pub (crate) struct DiagnosticHandlers < 'a > { data : * mut (& 'a CodegenContext < LlvmCodegenBackend > , DiagCtxtHandle < 'a >) , llcx : & 'a llvm :: Context , old_handler : Option < & 'a llvm :: DiagnosticHandler > , }}}
mkitem!{mkimpl!{impl < 'a > DiagnosticHandlers < 'a > { pub (crate) fn new (cgcx : & 'a CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < 'a > , llcx : & 'a llvm :: Context , module : & ModuleCodegen < ModuleLlvm > , stage : CodegenDiagnosticsStage ,) -> Self { let remark_passes_all : bool ; let remark_passes : Vec < CString > ; match & cgcx . remark { Passes :: All => { remark_passes_all = true ; remark_passes = Vec :: new () ; } Passes :: Some (passes) => { remark_passes_all = false ; remark_passes = passes . iter () . map (| name | CString :: new (name . as_str ()) . unwrap ()) . collect () ; } } ; let remark_passes : Vec < * const c_char > = remark_passes . iter () . map (| name : & CString | name . as_ptr ()) . collect () ; let remark_file = cgcx . remark_dir . as_ref () . map (| dir | { let stage_suffix = match stage { CodegenDiagnosticsStage :: Codegen => "codegen" , CodegenDiagnosticsStage :: Opt => "opt" , CodegenDiagnosticsStage :: LTO => "lto" , } ; dir . join (format ! ("{}.{stage_suffix}.opt.yaml" , module . name)) }) . and_then (| dir | dir . to_str () . and_then (| p | CString :: new (p) . ok ())) ; let pgo_available = cgcx . opts . cg . profile_use . is_some () ; let data = Box :: into_raw (Box :: new ((cgcx , dcx))) ; unsafe { let old_handler = llvm :: LLVMRustContextGetDiagnosticHandler (llcx) ; llvm :: LLVMRustContextConfigureDiagnosticHandler (llcx , diagnostic_handler , data . cast () , remark_passes_all , remark_passes . as_ptr () , remark_passes . len () , remark_file . as_ref () . map (| dir | dir . as_ptr ()) . unwrap_or (std :: ptr :: null ()) , pgo_available ,) ; DiagnosticHandlers { data , llcx , old_handler } } } }}}
mkitem!{mkimpl!{impl < 'a > Drop for DiagnosticHandlers < 'a > { fn drop (& mut self) { unsafe { llvm :: LLVMRustContextSetDiagnosticHandler (self . llcx , self . old_handler) ; drop (Box :: from_raw (self . data)) ; } } }}}

macro_rules! report_inline_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_inline_asm in module {}", module_path!());
    };
}

mkfn!{
    report_inline_asm_introspect!();
    fn report_inline_asm (cgcx : & CodegenContext < LlvmCodegenBackend > , msg : String , level : llvm :: DiagnosticLevel , cookie : u64 , source : Option < (String , Vec < InnerSpan >) > ,) { let span = if cookie == 0 || matches ! (cgcx . lto , Lto :: Fat | Lto :: Thin) { SpanData :: default () } else { SpanData { lo : BytePos :: from_u32 (cookie as u32) , hi : BytePos :: from_u32 ((cookie >> 32) as u32) , ctxt : SyntaxContext :: root () , parent : None , } } ; let level = match level { llvm :: DiagnosticLevel :: Error => Level :: Error , llvm :: DiagnosticLevel :: Warning => Level :: Warning , llvm :: DiagnosticLevel :: Note | llvm :: DiagnosticLevel :: Remark => Level :: Note , } ; let msg = msg . strip_prefix ("error: ") . unwrap_or (& msg) . to_string () ; cgcx . diag_emitter . inline_asm_error (span , msg , level , source) ; }
}

macro_rules! diagnostic_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function diagnostic_handler in module {}", module_path!());
    };
}

mkfn!{
    diagnostic_handler_introspect!();
    unsafe extern "C" fn diagnostic_handler (info : & DiagnosticInfo , user : * mut c_void) { if user . is_null () { return ; } let (cgcx , dcx) = unsafe { * (user as * const (& CodegenContext < LlvmCodegenBackend > , DiagCtxtHandle < '_ >)) } ; match unsafe { llvm :: diagnostic :: Diagnostic :: unpack (info) } { llvm :: diagnostic :: InlineAsm (inline) => { report_inline_asm (cgcx , inline . message , inline . level , inline . cookie , inline . source) ; } llvm :: diagnostic :: Optimization (opt) => { dcx . emit_note (FromLlvmOptimizationDiag { filename : & opt . filename , line : opt . line , column : opt . column , pass_name : & opt . pass_name , kind : match opt . kind { OptimizationRemark => "success" , OptimizationMissed | OptimizationFailure => "missed" , OptimizationAnalysis | OptimizationAnalysisFPCommute | OptimizationAnalysisAliasing => "analysis" , OptimizationRemarkOther => "other" , } , message : & opt . message , }) ; } llvm :: diagnostic :: PGO (diagnostic_ref) | llvm :: diagnostic :: Linker (diagnostic_ref) => { let message = llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteDiagnosticInfoToString (diagnostic_ref , s) }) . expect ("non-UTF8 diagnostic") ; dcx . emit_warn (FromLlvmDiag { message }) ; } llvm :: diagnostic :: Unsupported (diagnostic_ref) => { let message = llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteDiagnosticInfoToString (diagnostic_ref , s) }) . expect ("non-UTF8 diagnostic") ; dcx . emit_err (FromLlvmDiag { message }) ; } llvm :: diagnostic :: UnknownDiagnostic (..) => { } } }
}

macro_rules! get_pgo_gen_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_pgo_gen_path in module {}", module_path!());
    };
}

mkfn!{
    get_pgo_gen_path_introspect!();
    fn get_pgo_gen_path (config : & ModuleConfig) -> Option < CString > { match config . pgo_gen { SwitchWithOptPath :: Enabled (ref opt_dir_path) => { let path = if let Some (dir_path) = opt_dir_path { dir_path . join ("default_%m.profraw") } else { PathBuf :: from ("default_%m.profraw") } ; Some (CString :: new (format ! ("{}" , path . display ())) . unwrap ()) } SwitchWithOptPath :: Disabled => None , } }
}

macro_rules! get_pgo_use_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_pgo_use_path in module {}", module_path!());
    };
}

mkfn!{
    get_pgo_use_path_introspect!();
    fn get_pgo_use_path (config : & ModuleConfig) -> Option < CString > { config . pgo_use . as_ref () . map (| path_buf | CString :: new (path_buf . to_string_lossy () . as_bytes ()) . unwrap ()) }
}

macro_rules! get_pgo_sample_use_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_pgo_sample_use_path in module {}", module_path!());
    };
}

mkfn!{
    get_pgo_sample_use_path_introspect!();
    fn get_pgo_sample_use_path (config : & ModuleConfig) -> Option < CString > { config . pgo_sample_use . as_ref () . map (| path_buf | CString :: new (path_buf . to_string_lossy () . as_bytes ()) . unwrap ()) }
}

macro_rules! get_instr_profile_output_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_instr_profile_output_path in module {}", module_path!());
    };
}

mkfn!{
    get_instr_profile_output_path_introspect!();
    fn get_instr_profile_output_path (config : & ModuleConfig) -> Option < CString > { config . instrument_coverage . then (| | c"default_%m_%p.profraw" . to_owned ()) }
}
mkitem!{mkenum!{# [derive (Debug , Eq , PartialEq)] pub (crate) enum AutodiffStage { PreAD , DuringAD , PostAD , }}}

macro_rules! llvm_optimize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_optimize in module {}", module_path!());
    };
}

mkfn!{
    llvm_optimize_introspect!();
    pub (crate) unsafe fn llvm_optimize (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , module : & ModuleCodegen < ModuleLlvm > , thin_lto_buffer : Option < & mut * mut llvm :: ThinLTOBuffer > , config : & ModuleConfig , opt_level : config :: OptLevel , opt_stage : llvm :: OptStage , autodiff_stage : AutodiffStage ,) { let consider_ad = cfg ! (llvm_enzyme) && config . autodiff . contains (& config :: AutoDiff :: Enable) ; let run_enzyme = autodiff_stage == AutodiffStage :: DuringAD ; let print_before_enzyme = config . autodiff . contains (& config :: AutoDiff :: PrintModBefore) ; let print_after_enzyme = config . autodiff . contains (& config :: AutoDiff :: PrintModAfter) ; let print_passes = config . autodiff . contains (& config :: AutoDiff :: PrintPasses) ; let merge_functions ; let unroll_loops ; let vectorize_slp ; let vectorize_loop ; if consider_ad && autodiff_stage != AutodiffStage :: PostAD { merge_functions = false ; unroll_loops = false ; vectorize_slp = false ; vectorize_loop = false ; } else { unroll_loops = opt_level != config :: OptLevel :: Size && opt_level != config :: OptLevel :: SizeMin ; merge_functions = config . merge_functions ; vectorize_slp = config . vectorize_slp ; vectorize_loop = config . vectorize_loop ; } trace ! (? unroll_loops , ? vectorize_slp , ? vectorize_loop , ? run_enzyme) ; if thin_lto_buffer . is_some () { assert ! (matches ! (opt_stage , llvm :: OptStage :: PreLinkNoLTO | llvm :: OptStage :: PreLinkFatLTO | llvm :: OptStage :: PreLinkThinLTO) , "the bitcode for LTO can only be obtained at the pre-link stage") ; } let pgo_gen_path = get_pgo_gen_path (config) ; let pgo_use_path = get_pgo_use_path (config) ; let pgo_sample_use_path = get_pgo_sample_use_path (config) ; let is_lto = opt_stage == llvm :: OptStage :: ThinLTO || opt_stage == llvm :: OptStage :: FatLTO ; let instr_profile_output_path = get_instr_profile_output_path (config) ; let sanitize_dataflow_abilist : Vec < _ > = config . sanitizer_dataflow_abilist . iter () . map (| file | CString :: new (file . as_str ()) . unwrap ()) . collect () ; let sanitize_dataflow_abilist_ptrs : Vec < _ > = sanitize_dataflow_abilist . iter () . map (| file | file . as_ptr ()) . collect () ; let sanitizer_options = if ! is_lto { Some (llvm :: SanitizerOptions { sanitize_address : config . sanitizer . contains (SanitizerSet :: ADDRESS) , sanitize_address_recover : config . sanitizer_recover . contains (SanitizerSet :: ADDRESS) , sanitize_cfi : config . sanitizer . contains (SanitizerSet :: CFI) , sanitize_dataflow : config . sanitizer . contains (SanitizerSet :: DATAFLOW) , sanitize_dataflow_abilist : sanitize_dataflow_abilist_ptrs . as_ptr () , sanitize_dataflow_abilist_len : sanitize_dataflow_abilist_ptrs . len () , sanitize_kcfi : config . sanitizer . contains (SanitizerSet :: KCFI) , sanitize_memory : config . sanitizer . contains (SanitizerSet :: MEMORY) , sanitize_memory_recover : config . sanitizer_recover . contains (SanitizerSet :: MEMORY) , sanitize_memory_track_origins : config . sanitizer_memory_track_origins as c_int , sanitize_thread : config . sanitizer . contains (SanitizerSet :: THREAD) , sanitize_hwaddress : config . sanitizer . contains (SanitizerSet :: HWADDRESS) , sanitize_hwaddress_recover : config . sanitizer_recover . contains (SanitizerSet :: HWADDRESS) , sanitize_kernel_address : config . sanitizer . contains (SanitizerSet :: KERNELADDRESS) , sanitize_kernel_address_recover : config . sanitizer_recover . contains (SanitizerSet :: KERNELADDRESS) , }) } else { None } ; let mut llvm_profiler = cgcx . prof . llvm_recording_enabled () . then (| | LlvmSelfProfiler :: new (cgcx . prof . get_self_profiler () . unwrap ())) ; let llvm_selfprofiler = llvm_profiler . as_mut () . map (| s | s as * mut _ as * mut c_void) . unwrap_or (std :: ptr :: null_mut ()) ; let extra_passes = if ! is_lto { config . passes . join (",") } else { "" . to_string () } ; let llvm_plugins = config . llvm_plugins . join (",") ; let result = unsafe { llvm :: LLVMRustOptimize (module . module_llvm . llmod () , & * module . module_llvm . tm . raw () , to_pass_builder_opt_level (opt_level) , opt_stage , cgcx . opts . cg . linker_plugin_lto . enabled () , config . no_prepopulate_passes , config . verify_llvm_ir , config . lint_llvm_ir , thin_lto_buffer , config . emit_thin_lto , config . emit_thin_lto_summary , merge_functions , unroll_loops , vectorize_slp , vectorize_loop , config . no_builtins , config . emit_lifetime_markers , run_enzyme , print_before_enzyme , print_after_enzyme , print_passes , sanitizer_options . as_ref () , pgo_gen_path . as_ref () . map_or (std :: ptr :: null () , | s | s . as_ptr ()) , pgo_use_path . as_ref () . map_or (std :: ptr :: null () , | s | s . as_ptr ()) , config . instrument_coverage , instr_profile_output_path . as_ref () . map_or (std :: ptr :: null () , | s | s . as_ptr ()) , pgo_sample_use_path . as_ref () . map_or (std :: ptr :: null () , | s | s . as_ptr ()) , config . debug_info_for_profiling , llvm_selfprofiler , selfprofile_before_pass_callback , selfprofile_after_pass_callback , extra_passes . as_c_char_ptr () , extra_passes . len () , llvm_plugins . as_c_char_ptr () , llvm_plugins . len () ,) } ; result . into_result () . unwrap_or_else (| () | llvm_err (dcx , LlvmError :: RunLlvmPasses)) }
}

macro_rules! optimize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function optimize in module {}", module_path!());
    };
}

mkfn!{
    optimize_introspect!();
    pub (crate) fn optimize (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < ModuleLlvm > , config : & ModuleConfig ,) { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_optimize" , & * module . name) ; let llcx = & * module . module_llvm . llcx ; let _handlers = DiagnosticHandlers :: new (cgcx , dcx , llcx , module , CodegenDiagnosticsStage :: Opt) ; if config . emit_no_opt_bc { let out = cgcx . output_filenames . temp_path_ext_for_cgu ("no-opt.bc" , & module . name , cgcx . invocation_temp . as_deref () ,) ; write_bitcode_to_file (module , & out) } if let Some (opt_level) = config . opt_level { let opt_stage = match cgcx . lto { Lto :: Fat => llvm :: OptStage :: PreLinkFatLTO , Lto :: Thin | Lto :: ThinLocal => llvm :: OptStage :: PreLinkThinLTO , _ if cgcx . opts . cg . linker_plugin_lto . enabled () => llvm :: OptStage :: PreLinkThinLTO , _ => llvm :: OptStage :: PreLinkNoLTO , } ; let consider_ad = cfg ! (llvm_enzyme) && config . autodiff . contains (& config :: AutoDiff :: Enable) ; let autodiff_stage = if consider_ad { AutodiffStage :: PreAD } else { AutodiffStage :: PostAD } ; let mut thin_lto_buffer = if (module . kind == ModuleKind :: Regular && config . emit_obj == EmitObj :: ObjectCode (BitcodeSection :: Full)) || config . emit_thin_lto_summary { Some (null_mut ()) } else { None } ; unsafe { llvm_optimize (cgcx , dcx , module , thin_lto_buffer . as_mut () , config , opt_level , opt_stage , autodiff_stage ,) } ; if let Some (thin_lto_buffer) = thin_lto_buffer { let thin_lto_buffer = unsafe { ThinBuffer :: from_raw_ptr (thin_lto_buffer) } ; module . thin_lto_buffer = Some (thin_lto_buffer . data () . to_vec ()) ; let bc_summary_out = cgcx . output_filenames . temp_path_for_cgu (OutputType :: ThinLinkBitcode , & module . name , cgcx . invocation_temp . as_deref () ,) ; if config . emit_thin_lto_summary && let Some (thin_link_bitcode_filename) = bc_summary_out . file_name () { let summary_data = thin_lto_buffer . thin_link_data () ; cgcx . prof . artifact_size ("llvm_bitcode_summary" , thin_link_bitcode_filename . to_string_lossy () , summary_data . len () as u64 ,) ; let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_bitcode_summary" , & * module . name ,) ; if let Err (err) = fs :: write (& bc_summary_out , summary_data) { dcx . emit_err (WriteBytecode { path : & bc_summary_out , err }) ; } } } } }
}

macro_rules! codegen_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen in module {}", module_path!());
    };
}

mkfn!{
    codegen_introspect!();
    pub (crate) fn codegen (cgcx : & CodegenContext < LlvmCodegenBackend > , module : ModuleCodegen < ModuleLlvm > , config : & ModuleConfig ,) -> CompiledModule { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen" , & * module . name) ; { let llmod = module . module_llvm . llmod () ; let llcx = & * module . module_llvm . llcx ; let tm = & * module . module_llvm . tm ; let _handlers = DiagnosticHandlers :: new (cgcx , dcx , llcx , & module , CodegenDiagnosticsStage :: Codegen) ; if cgcx . msvc_imps_needed { create_msvc_imps (cgcx , llcx , llmod) ; } let bc_out = cgcx . output_filenames . temp_path_for_cgu (OutputType :: Bitcode , & module . name , cgcx . invocation_temp . as_deref () ,) ; let obj_out = cgcx . output_filenames . temp_path_for_cgu (OutputType :: Object , & module . name , cgcx . invocation_temp . as_deref () ,) ; if config . bitcode_needed () { if config . emit_bc || config . emit_obj == EmitObj :: Bitcode { let thin = { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_make_bitcode" , & * module . name ,) ; ThinBuffer :: new (llmod , config . emit_thin_lto) } ; let data = thin . data () ; let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_bitcode" , & * module . name) ; if let Some (bitcode_filename) = bc_out . file_name () { cgcx . prof . artifact_size ("llvm_bitcode" , bitcode_filename . to_string_lossy () , data . len () as u64 ,) ; } if let Err (err) = fs :: write (& bc_out , data) { dcx . emit_err (WriteBytecode { path : & bc_out , err }) ; } } if config . embed_bitcode () && module . kind == ModuleKind :: Regular { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_embed_bitcode" , & * module . name) ; let thin_bc = module . thin_lto_buffer . as_deref () . expect ("cannot find embedded bitcode") ; embed_bitcode (cgcx , llcx , llmod , & thin_bc) ; } } if config . emit_ir { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_ir" , & * module . name) ; let out = cgcx . output_filenames . temp_path_for_cgu (OutputType :: LlvmAssembly , & module . name , cgcx . invocation_temp . as_deref () ,) ; let out_c = path_to_c_string (& out) ; extern "C" fn demangle_callback (input_ptr : * const c_char , input_len : size_t , output_ptr : * mut c_char , output_len : size_t ,) -> size_t { let input = unsafe { slice :: from_raw_parts (input_ptr as * const u8 , input_len as usize) } ; let Ok (input) = str :: from_utf8 (input) else { return 0 } ; let output = unsafe { slice :: from_raw_parts_mut (output_ptr as * mut u8 , output_len as usize) } ; let mut cursor = io :: Cursor :: new (output) ; let Ok (demangled) = rustc_demangle :: try_demangle (input) else { return 0 } ; if write ! (cursor , "{demangled:#}") . is_err () { return 0 ; } cursor . position () as size_t } let result = unsafe { llvm :: LLVMRustPrintModule (llmod , out_c . as_ptr () , demangle_callback) } ; if result == llvm :: LLVMRustResult :: Success { record_artifact_size (& cgcx . prof , "llvm_ir" , & out) ; } result . into_result () . unwrap_or_else (| () | llvm_err (dcx , LlvmError :: WriteIr { path : & out })) ; } if config . emit_asm { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_asm" , & * module . name) ; let path = cgcx . output_filenames . temp_path_for_cgu (OutputType :: Assembly , & module . name , cgcx . invocation_temp . as_deref () ,) ; let llmod = if let EmitObj :: ObjectCode (_) = config . emit_obj { llvm :: LLVMCloneModule (llmod) } else { llmod } ; write_output_file (dcx , tm . raw () , config . no_builtins , llmod , & path , None , llvm :: FileType :: AssemblyFile , & cgcx . prof , config . verify_llvm_ir ,) ; } match config . emit_obj { EmitObj :: ObjectCode (_) => { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_obj" , & * module . name) ; let dwo_out = cgcx . output_filenames . temp_path_dwo_for_cgu (& module . name , cgcx . invocation_temp . as_deref ()) ; let dwo_out = match (cgcx . split_debuginfo , cgcx . split_dwarf_kind) { (SplitDebuginfo :: Off , _) => None , _ if ! cgcx . target_can_use_split_dwarf => None , (_ , SplitDwarfKind :: Single) => None , (_ , SplitDwarfKind :: Split) => Some (dwo_out . as_path ()) , } ; write_output_file (dcx , tm . raw () , config . no_builtins , llmod , & obj_out , dwo_out , llvm :: FileType :: ObjectFile , & cgcx . prof , config . verify_llvm_ir ,) ; } EmitObj :: Bitcode => { debug ! ("copying bitcode {:?} to obj {:?}" , bc_out , obj_out) ; if let Err (err) = link_or_copy (& bc_out , & obj_out) { dcx . emit_err (CopyBitcode { err }) ; } if ! config . emit_bc { debug ! ("removing_bitcode {:?}" , bc_out) ; ensure_removed (dcx , & bc_out) ; } } EmitObj :: None => { } } record_llvm_cgu_instructions_stats (& cgcx . prof , llmod) ; } let dwarf_object_emitted = matches ! (config . emit_obj , EmitObj :: ObjectCode (_)) && cgcx . target_can_use_split_dwarf && cgcx . split_debuginfo != SplitDebuginfo :: Off && cgcx . split_dwarf_kind == SplitDwarfKind :: Split ; module . into_compiled_module (config . emit_obj != EmitObj :: None , dwarf_object_emitted , config . emit_bc , config . emit_asm , config . emit_ir , & cgcx . output_filenames , cgcx . invocation_temp . as_deref () ,) }
}

macro_rules! create_section_with_flags_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_section_with_flags_asm in module {}", module_path!());
    };
}

mkfn!{
    create_section_with_flags_asm_introspect!();
    fn create_section_with_flags_asm (section_name : & str , section_flags : & str , data : & [u8]) -> Vec < u8 > { let mut asm = format ! (".section {section_name},\"{section_flags}\"\n") . into_bytes () ; asm . extend_from_slice (b".ascii \"") ; asm . reserve (data . len ()) ; for & byte in data { if byte == b'\\' || byte == b'"' { asm . push (b'\\') ; asm . push (byte) ; } else if byte < 0x20 || byte >= 0x80 { asm . push (b'\\') ; asm . push (b'0' + ((byte >> 6) & 0x7)) ; asm . push (b'0' + ((byte >> 3) & 0x7)) ; asm . push (b'0' + ((byte >> 0) & 0x7)) ; } else { asm . push (byte) ; } } asm . extend_from_slice (b"\"\n") ; asm }
}

macro_rules! bitcode_section_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bitcode_section_name in module {}", module_path!());
    };
}

mkfn!{
    bitcode_section_name_introspect!();
    pub (crate) fn bitcode_section_name (cgcx : & CodegenContext < LlvmCodegenBackend >) -> & 'static CStr { if cgcx . target_is_like_darwin { c"__LLVM,__bitcode" } else if cgcx . target_is_like_aix { c".ipa" } else { c".llvmbc" } }
}

macro_rules! embed_bitcode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function embed_bitcode in module {}", module_path!());
    };
}

mkfn!{
    embed_bitcode_introspect!();
    # [doc = " Embed the bitcode of an LLVM module for LTO in the LLVM module itself."] fn embed_bitcode (cgcx : & CodegenContext < LlvmCodegenBackend > , llcx : & llvm :: Context , llmod : & llvm :: Module , bitcode : & [u8] ,) { if cgcx . target_is_like_darwin || cgcx . target_is_like_aix || cgcx . target_arch == "wasm32" || cgcx . target_arch == "wasm64" { let llconst = common :: bytes_in_context (llcx , bitcode) ; let llglobal = llvm :: add_global (llmod , common :: val_ty (llconst) , c"rustc.embedded.module") ; llvm :: set_initializer (llglobal , llconst) ; llvm :: set_section (llglobal , bitcode_section_name (cgcx)) ; llvm :: set_linkage (llglobal , llvm :: Linkage :: PrivateLinkage) ; llvm :: LLVMSetGlobalConstant (llglobal , llvm :: TRUE) ; let llconst = common :: bytes_in_context (llcx , & []) ; let llglobal = llvm :: add_global (llmod , common :: val_ty (llconst) , c"rustc.embedded.cmdline") ; llvm :: set_initializer (llglobal , llconst) ; let section = if cgcx . target_is_like_darwin { c"__LLVM,__cmdline" } else if cgcx . target_is_like_aix { c".info" } else { c".llvmcmd" } ; llvm :: set_section (llglobal , section) ; llvm :: set_linkage (llglobal , llvm :: Linkage :: PrivateLinkage) ; } else { let section_flags = if cgcx . is_pe_coff { "n" } else { "e" } ; let asm = create_section_with_flags_asm (".llvmbc" , section_flags , bitcode) ; llvm :: append_module_inline_asm (llmod , & asm) ; let asm = create_section_with_flags_asm (".llvmcmd" , section_flags , & []) ; llvm :: append_module_inline_asm (llmod , & asm) ; } }
}

macro_rules! create_msvc_imps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_msvc_imps in module {}", module_path!());
    };
}

mkfn!{
    create_msvc_imps_introspect!();
    fn create_msvc_imps (cgcx : & CodegenContext < LlvmCodegenBackend > , llcx : & llvm :: Context , llmod : & llvm :: Module ,) { if ! cgcx . msvc_imps_needed { return ; } let prefix = if cgcx . target_arch == "x86" { "\x01__imp__" } else { "\x01__imp_" } ; let ptr_ty = Type :: ptr_llcx (llcx) ; let globals = base :: iter_globals (llmod) . filter (| & val | { llvm :: get_linkage (val) == llvm :: Linkage :: ExternalLinkage && ! llvm :: is_declaration (val) }) . filter_map (| val | { let name = llvm :: get_value_name (val) ; if ignored (& name) { None } else { Some ((val , name)) } }) . map (move | (val , name) | { let mut imp_name = prefix . as_bytes () . to_vec () ; imp_name . extend (name) ; let imp_name = CString :: new (imp_name) . unwrap () ; (imp_name , val) }) . collect :: < Vec < _ > > () ; for (imp_name , val) in globals { let imp = llvm :: add_global (llmod , ptr_ty , & imp_name) ; llvm :: set_initializer (imp , val) ; llvm :: set_linkage (imp , llvm :: Linkage :: ExternalLinkage) ; } fn ignored (symbol_name : & [u8]) -> bool { symbol_name . starts_with (b"__llvm_profile_") } }
}

macro_rules! record_artifact_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function record_artifact_size in module {}", module_path!());
    };
}

mkfn!{
    record_artifact_size_introspect!();
    fn record_artifact_size (self_profiler_ref : & SelfProfilerRef , artifact_kind : & 'static str , path : & Path ,) { if ! self_profiler_ref . enabled () { return ; } if let Some (artifact_name) = path . file_name () { let file_size = std :: fs :: metadata (path) . map (| m | m . len ()) . unwrap_or (0) ; self_profiler_ref . artifact_size (artifact_kind , artifact_name . to_string_lossy () , file_size) ; } }
}

macro_rules! record_llvm_cgu_instructions_stats_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function record_llvm_cgu_instructions_stats in module {}", module_path!());
    };
}

mkfn!{
    record_llvm_cgu_instructions_stats_introspect!();
    fn record_llvm_cgu_instructions_stats (prof : & SelfProfilerRef , llmod : & llvm :: Module) { if ! prof . enabled () { return ; } let raw_stats = llvm :: build_string (| s | unsafe { llvm :: LLVMRustModuleInstructionStats (llmod , s) }) . expect ("cannot get module instruction stats") ; # [derive (serde :: Deserialize)] struct InstructionsStats { module : String , total : u64 , } let InstructionsStats { module , total } = serde_json :: from_str (& raw_stats) . expect ("cannot parse llvm cgu instructions stats") ; prof . artifact_size ("cgu_instructions" , module , total) ; }
}