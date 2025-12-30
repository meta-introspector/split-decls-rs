// Generated macro for impl_569 (impl)
macro_rules! Depcrateimpl_569 {
() => {
// Module: crate
// Provides: {"impl_569"}
// Dependencies: {}
impl ExtraBackendMethods for GccCodegenBackend { fn supports_parallel (& self) -> bool { false } fn codegen_allocator (& self , tcx : TyCtxt < '_ > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> Self :: Module { let mut mods = GccContext { context : Arc :: new (SyncContext :: new (new_context (tcx))) , relocation_model : tcx . sess . relocation_model () , should_combine_object_files : false , temp_dir : None , } ; unsafe { allocator :: codegen (tcx , & mut mods , module_name , kind , alloc_error_handler_kind) ; } mods } fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < Self :: Module > , u64) { base :: compile_codegen_unit (tcx , cgu_name , self . target_info . clone ()) } fn target_machine_factory (& self , _sess : & Session , _opt_level : OptLevel , _features : & [String] ,) -> TargetMachineFactoryFn < Self > { Arc :: new (| _ | Ok (())) } }
};
}
