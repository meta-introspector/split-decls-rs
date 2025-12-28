macro_rules! deps {
    () => {
        ModuleCodegen!();
        WriteBackendMethods!();
        TargetMachineFactoryFn!();
        CodegenBackend!();
    };
}

macro_rules! ExtraBackendMethods {
    () => {
        deps!();
        pub trait ExtraBackendMethods : CodegenBackend + WriteBackendMethods + Sized + Send + Sync + DynSend + DynSync { fn codegen_allocator < 'tcx > (& self , tcx : TyCtxt < 'tcx > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> Self :: Module ; # [doc = " This generates the codegen unit and returns it along with"] # [doc = " a `u64` giving an estimate of the unit's processing cost."] fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < Self :: Module > , u64) ; fn target_machine_factory (& self , sess : & Session , opt_level : config :: OptLevel , target_features : & [String] ,) -> TargetMachineFactoryFn < Self > ; fn spawn_named_thread < F , T > (_time_trace : bool , name : String , f : F ,) -> std :: io :: Result < std :: thread :: JoinHandle < T > > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { std :: thread :: Builder :: new () . name (name) . spawn (f) } # [doc = " Returns `true` if this backend can be safely called from multiple threads."] # [doc = ""] # [doc = " Defaults to `true`."] fn supports_parallel (& self) -> bool { true } }
    };
}

ExtraBackendMethods!();