// Generated macro for codegen (function)
macro_rules! Depcrate_allocatorcodegen {
() => {
// Module: crate::allocator
// Provides: {"codegen"}
// Dependencies: {}
# [doc = " Returns whether an allocator shim was created"] pub (crate) fn codegen (tcx : TyCtxt < '_ > , module : & mut dyn Module) -> bool { let Some (kind) = allocator_kind_for_codegen (tcx) else { return false } ; codegen_inner (tcx , module , kind , tcx . alloc_error_handler_kind (()) . unwrap () , tcx . sess . opts . unstable_opts . oom ,) ; true }
};
}
