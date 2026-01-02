mkuse!{use std :: any :: Any ;}
mkuse!{use std :: ffi :: CStr ;}
mkuse!{use std :: mem :: ManuallyDrop ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use back :: owned_target_machine :: OwnedTargetMachine ;}
mkuse!{use back :: write :: { create_informational_target_machine , create_target_machine } ;}
mkuse!{use context :: SimpleCx ;}
mkuse!{use errors :: ParseTargetMachineConfig ;}
mkuse!{use llvm_util :: target_config ;}
mkuse!{use rustc_ast :: expand :: allocator :: AllocatorKind ;}
mkuse!{use rustc_codegen_ssa :: back :: lto :: { SerializedModule , ThinModule } ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: { CodegenContext , FatLtoInput , ModuleConfig , TargetMachineFactoryConfig , TargetMachineFactoryFn , } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_codegen_ssa :: { CodegenResults , CompiledModule , ModuleCodegen , TargetConfig } ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_metadata :: EncodedMetadata ;}
mkuse!{use rustc_middle :: dep_graph :: { WorkProduct , WorkProductId } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { OptLevel , OutputFilenames , PrintKind , PrintRequest } ;}
mkuse!{use rustc_span :: Symbol ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{allocator, { 
                getname!(allocator);
                getsrc!(allocator);
                getpath!(allocator);
                get_deps!(allocator);
                get_crates!(allocator);
                mkinclude!(allocator);
                 
            }}
mkmod!{asm, { 
                getname!(asm);
                getsrc!(asm);
                getpath!(asm);
                get_deps!(asm);
                get_crates!(asm);
                mkinclude!(asm);
                 
            }}
mkmod!{attributes, { 
                getname!(attributes);
                getsrc!(attributes);
                getpath!(attributes);
                get_deps!(attributes);
                get_crates!(attributes);
                mkinclude!(attributes);
                 
            }}
mkmod!{back, { 
                getname!(back);
                getsrc!(back);
                getpath!(back);
                get_deps!(back);
                get_crates!(back);
                mkinclude!(back);
                 
            }}
mkmod!{base, { 
                getname!(base);
                getsrc!(base);
                getpath!(base);
                get_deps!(base);
                get_crates!(base);
                mkinclude!(base);
                 
            }}
mkmod!{builder, { 
                getname!(builder);
                getsrc!(builder);
                getpath!(builder);
                get_deps!(builder);
                get_crates!(builder);
                mkinclude!(builder);
                 
            }}
mkmod!{callee, { 
                getname!(callee);
                getsrc!(callee);
                getpath!(callee);
                get_deps!(callee);
                get_crates!(callee);
                mkinclude!(callee);
                 
            }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkmod!{consts, { 
                getname!(consts);
                getsrc!(consts);
                getpath!(consts);
                get_deps!(consts);
                get_crates!(consts);
                mkinclude!(consts);
                 
            }}
mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkmod!{coverageinfo, { 
                getname!(coverageinfo);
                getsrc!(coverageinfo);
                getpath!(coverageinfo);
                get_deps!(coverageinfo);
                get_crates!(coverageinfo);
                mkinclude!(coverageinfo);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{declare, { 
                getname!(declare);
                getsrc!(declare);
                getpath!(declare);
                get_deps!(declare);
                get_crates!(declare);
                mkinclude!(declare);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{llvm, { 
                getname!(llvm);
                getsrc!(llvm);
                getpath!(llvm);
                get_deps!(llvm);
                get_crates!(llvm);
                mkinclude!(llvm);
                 
            }}
mkmod!{llvm_util, { 
                getname!(llvm_util);
                getsrc!(llvm_util);
                getpath!(llvm_util);
                get_deps!(llvm_util);
                get_crates!(llvm_util);
                mkinclude!(llvm_util);
                 
            }}
mkmod!{mono_item, { 
                getname!(mono_item);
                getsrc!(mono_item);
                getpath!(mono_item);
                get_deps!(mono_item);
                get_crates!(mono_item);
                mkinclude!(mono_item);
                 
            }}
mkmod!{type_, { 
                getname!(type_);
                getsrc!(type_);
                getpath!(type_);
                get_deps!(type_);
                get_crates!(type_);
                mkinclude!(type_);
                 
            }}
mkmod!{type_of, { 
                getname!(type_of);
                getsrc!(type_of);
                getpath!(type_of);
                get_deps!(type_of);
                get_crates!(type_of);
                mkinclude!(type_of);
                 
            }}
mkmod!{va_arg, { 
                getname!(va_arg);
                getsrc!(va_arg);
                getpath!(va_arg);
                get_deps!(va_arg);
                get_crates!(va_arg);
                mkinclude!(va_arg);
                 
            }}
mkmod!{value, { 
                getname!(value);
                getsrc!(value);
                getpath!(value);
                get_deps!(value);
                get_crates!(value);
                mkinclude!(value);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkitem!{mkstruct!{# [derive (Clone)] pub struct LlvmCodegenBackend (()) ;}}
mkitem!{mkstruct!{struct TimeTraceProfiler { enabled : bool , }}}
mkitem!{mkimpl!{impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }}}
mkitem!{mkimpl!{impl Drop for TimeTraceProfiler { fn drop (& mut self) { if self . enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerFinishThread () } } } }}}
mkitem!{mkimpl!{impl ExtraBackendMethods for LlvmCodegenBackend { fn codegen_allocator < 'tcx > (& self , tcx : TyCtxt < 'tcx > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> ModuleLlvm { let module_llvm = ModuleLlvm :: new_metadata (tcx , module_name) ; let cx = SimpleCx :: new (module_llvm . llmod () , & module_llvm . llcx , tcx . data_layout . pointer_size ()) ; unsafe { allocator :: codegen (tcx , cx , module_name , kind , alloc_error_handler_kind) ; } module_llvm } fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < ModuleLlvm > , u64) { base :: compile_codegen_unit (tcx , cgu_name) } fn target_machine_factory (& self , sess : & Session , optlvl : OptLevel , target_features : & [String] ,) -> TargetMachineFactoryFn < Self > { back :: write :: target_machine_factory (sess , optlvl , target_features) } fn spawn_named_thread < F , T > (time_trace : bool , name : String , f : F ,) -> std :: io :: Result < std :: thread :: JoinHandle < T > > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { std :: thread :: Builder :: new () . name (name) . spawn (move | | { let _profiler = TimeTraceProfiler :: new (time_trace) ; f () }) } }}}
mkitem!{mkimpl!{impl WriteBackendMethods for LlvmCodegenBackend { type Module = ModuleLlvm ; type ModuleBuffer = back :: lto :: ModuleBuffer ; type TargetMachine = OwnedTargetMachine ; type TargetMachineError = crate :: errors :: LlvmError < 'static > ; type ThinData = back :: lto :: ThinData ; type ThinBuffer = back :: lto :: ThinBuffer ; fn print_pass_timings (& self) { let timings = llvm :: build_string (| s | unsafe { llvm :: LLVMRustPrintPassTimings (s) }) . unwrap () ; print ! ("{timings}") ; } fn print_statistics (& self) { let stats = llvm :: build_string (| s | unsafe { llvm :: LLVMRustPrintStatistics (s) }) . unwrap () ; print ! ("{stats}") ; } fn run_and_optimize_fat_lto (cgcx : & CodegenContext < Self > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < Self > > ,) -> ModuleCodegen < Self :: Module > { let mut module = back :: lto :: run_fat (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , modules) ; let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; back :: lto :: run_pass_manager (cgcx , dcx , & mut module , false) ; module } fn run_thin_lto (cgcx : & CodegenContext < Self > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , Self :: ThinBuffer) > , cached_modules : Vec < (SerializedModule < Self :: ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < Self > > , Vec < WorkProduct >) { back :: lto :: run_thin (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , modules , cached_modules ,) } fn optimize (cgcx : & CodegenContext < Self > , dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < Self :: Module > , config : & ModuleConfig ,) { back :: write :: optimize (cgcx , dcx , module , config) } fn optimize_thin (cgcx : & CodegenContext < Self > , thin : ThinModule < Self > ,) -> ModuleCodegen < Self :: Module > { back :: lto :: optimize_thin_module (thin , cgcx) } fn codegen (cgcx : & CodegenContext < Self > , module : ModuleCodegen < Self :: Module > , config : & ModuleConfig ,) -> CompiledModule { back :: write :: codegen (cgcx , module , config) } fn prepare_thin (module : ModuleCodegen < Self :: Module >) -> (String , Self :: ThinBuffer) { back :: lto :: prepare_thin (module) } fn serialize_module (module : ModuleCodegen < Self :: Module >) -> (String , Self :: ModuleBuffer) { (module . name , back :: lto :: ModuleBuffer :: new (module . module_llvm . llmod ())) } }}}
mkitem!{mkimpl!{impl LlvmCodegenBackend { pub fn new () -> Box < dyn CodegenBackend > { Box :: new (LlvmCodegenBackend (())) } }}}
mkitem!{mkimpl!{impl CodegenBackend for LlvmCodegenBackend { fn locale_resource (& self) -> & 'static str { crate :: DEFAULT_LOCALE_RESOURCE } fn init (& self , sess : & Session) { llvm_util :: init (sess) ; } fn provide (& self , providers : & mut Providers) { providers . global_backend_features = | tcx , () | llvm_util :: global_llvm_features (tcx . sess , true , false) } fn print (& self , req : & PrintRequest , out : & mut String , sess : & Session) { use std :: fmt :: Write ; match req . kind { PrintKind :: RelocationModels => { writeln ! (out , "Available relocation models:") . unwrap () ; for name in & ["static" , "pic" , "pie" , "dynamic-no-pic" , "ropi" , "rwpi" , "ropi-rwpi" , "default" ,] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: CodeModels => { writeln ! (out , "Available code models:") . unwrap () ; for name in & ["tiny" , "small" , "kernel" , "medium" , "large"] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: TlsModels => { writeln ! (out , "Available TLS models:") . unwrap () ; for name in & ["global-dynamic" , "local-dynamic" , "initial-exec" , "local-exec" , "emulated"] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: StackProtectorStrategies => { writeln ! (out , r#"Available stack protector strategies:
    all
        Generate stack canaries in all functions.

    strong
        Generate stack canaries in a function if it either:
        - has a local variable of `[T; N]` type, regardless of `T` and `N`
        - takes the address of a local variable.

          (Note that a local variable being borrowed is not equivalent to its
          address being taken: e.g. some borrows may be removed by optimization,
          while by-value argument passing may be implemented with reference to a
          local stack variable in the ABI.)

    basic
        Generate stack canaries in functions with local variables of `[T; N]`
        type, where `T` is byte-sized and `N` >= 8.

    none
        Do not generate stack canaries.
"#) . unwrap () ; } _other => llvm_util :: print (req , out , sess) , } } fn print_passes (& self) { llvm_util :: print_passes () ; } fn print_version (& self) { llvm_util :: print_version () ; } fn target_config (& self , sess : & Session) -> TargetConfig { target_config (sess) } fn codegen_crate < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Box < dyn Any > { Box :: new (rustc_codegen_ssa :: base :: codegen_crate (LlvmCodegenBackend (()) , tcx , crate :: llvm_util :: target_cpu (tcx . sess) . to_string () ,)) } fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { let (codegen_results , work_products) = ongoing_codegen . downcast :: < rustc_codegen_ssa :: back :: write :: OngoingCodegen < LlvmCodegenBackend > > () . expect ("Expected LlvmCodegenBackend's OngoingCodegen, found Box<Any>") . join (sess) ; if sess . opts . unstable_opts . llvm_time_trace { sess . time ("llvm_dump_timing_file" , | | { let file_name = outputs . with_extension ("llvm_timings.json") ; llvm_util :: time_trace_profiler_finish (& file_name) ; }) ; } (codegen_results , work_products) } fn link (& self , sess : & Session , codegen_results : CodegenResults , metadata : EncodedMetadata , outputs : & OutputFilenames ,) { use rustc_codegen_ssa :: back :: link :: link_binary ; use crate :: back :: archive :: LlvmArchiveBuilderBuilder ; link_binary (sess , & LlvmArchiveBuilderBuilder , codegen_results , metadata , outputs) ; } }}}
mkitem!{mkstruct!{pub struct ModuleLlvm { llcx : & 'static mut llvm :: Context , llmod_raw : * const llvm :: Module , tm : ManuallyDrop < OwnedTargetMachine > , }}}
mkitem!{mkimpl!{unsafe impl Send for ModuleLlvm { }}}
mkitem!{mkimpl!{unsafe impl Sync for ModuleLlvm { }}}
mkitem!{mkimpl!{impl ModuleLlvm { fn new (tcx : TyCtxt < '_ > , mod_name : & str) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (tcx . sess . fewer_names ()) ; let llmod_raw = context :: create_module (tcx , llcx , mod_name) as * const _ ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (create_target_machine (tcx , mod_name)) , } } } fn new_metadata (tcx : TyCtxt < '_ > , mod_name : & str) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (tcx . sess . fewer_names ()) ; let llmod_raw = context :: create_module (tcx , llcx , mod_name) as * const _ ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (create_informational_target_machine (tcx . sess , false)) , } } } fn tm_from_cgcx (cgcx : & CodegenContext < LlvmCodegenBackend > , name : & str , dcx : DiagCtxtHandle < '_ > ,) -> OwnedTargetMachine { let tm_factory_config = TargetMachineFactoryConfig :: new (cgcx , name) ; match (cgcx . tm_factory) (tm_factory_config) { Ok (m) => m , Err (e) => { dcx . emit_fatal (ParseTargetMachineConfig (e)) ; } } } fn parse (cgcx : & CodegenContext < LlvmCodegenBackend > , name : & CStr , buffer : & [u8] , dcx : DiagCtxtHandle < '_ > ,) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (cgcx . fewer_names) ; let llmod_raw = back :: lto :: parse_module (llcx , name , buffer , dcx) ; let tm = ModuleLlvm :: tm_from_cgcx (cgcx , name . to_str () . unwrap () , dcx) ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (tm) } } } fn llmod (& self) -> & llvm :: Module { unsafe { & * self . llmod_raw } } }}}
mkitem!{mkimpl!{impl Drop for ModuleLlvm { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . tm) ; llvm :: LLVMContextDispose (& mut * (self . llcx as * mut _)) ; } } }}}