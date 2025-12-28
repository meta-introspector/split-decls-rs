macro_rules! deps {
    () => {
        ModuleKind!();
        CrateInfo!();
        ModuleConfig!();
        SharedEmitter!();
        ExtraBackendMethods!();
        ModuleCodegen!();
        OngoingCodegen!();
        Coordinator!();
    };
}

macro_rules! start_async_codegen {
    () => {
        deps!();
        pub (crate) fn start_async_codegen < B : ExtraBackendMethods > (backend : B , tcx : TyCtxt < '_ > , target_cpu : String , allocator_module : Option < ModuleCodegen < B :: Module > > ,) -> OngoingCodegen < B > { let (coordinator_send , coordinator_receive) = channel () ; let crate_attrs = tcx . hir_attrs (rustc_hir :: CRATE_HIR_ID) ; let no_builtins = attr :: contains_name (crate_attrs , sym :: no_builtins) ; let crate_info = CrateInfo :: new (tcx , target_cpu) ; let regular_config = ModuleConfig :: new (ModuleKind :: Regular , tcx , no_builtins) ; let allocator_config = ModuleConfig :: new (ModuleKind :: Allocator , tcx , no_builtins) ; let (shared_emitter , shared_emitter_main) = SharedEmitter :: new () ; let (codegen_worker_send , codegen_worker_receive) = channel () ; let coordinator_thread = start_executing_work (backend . clone () , tcx , & crate_info , shared_emitter , codegen_worker_send , coordinator_receive , Arc :: new (regular_config) , Arc :: new (allocator_config) , allocator_module , coordinator_send . clone () ,) ; OngoingCodegen { backend , crate_info , codegen_worker_receive , shared_emitter_main , coordinator : Coordinator { sender : coordinator_send , future : Some (coordinator_thread) , phantom : PhantomData , } , output_filenames : Arc :: clone (tcx . output_filenames (())) , } }
    };
}

start_async_codegen!();