macro_rules! deps {
    () => {
        TimeTraceProfiler!();
        ModuleLlvm!();
        SimpleCx!();
        Builder!();
        LlvmCodegenBackend!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl ExtraBackendMethods for LlvmCodegenBackend { fn codegen_allocator < 'tcx > (& self , tcx : TyCtxt < 'tcx > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> ModuleLlvm { let module_llvm = ModuleLlvm :: new_metadata (tcx , module_name) ; let cx = SimpleCx :: new (module_llvm . llmod () , & module_llvm . llcx , tcx . data_layout . pointer_size ()) ; unsafe { allocator :: codegen (tcx , cx , module_name , kind , alloc_error_handler_kind) ; } module_llvm } fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < ModuleLlvm > , u64) { base :: compile_codegen_unit (tcx , cgu_name) } fn target_machine_factory (& self , sess : & Session , optlvl : OptLevel , target_features : & [String] ,) -> TargetMachineFactoryFn < Self > { back :: write :: target_machine_factory (sess , optlvl , target_features) } fn spawn_named_thread < F , T > (time_trace : bool , name : String , f : F ,) -> std :: io :: Result < std :: thread :: JoinHandle < T > > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { std :: thread :: Builder :: new () . name (name) . spawn (move | | { let _profiler = TimeTraceProfiler :: new (time_trace) ; f () }) } }
    };
}

impl_622!();