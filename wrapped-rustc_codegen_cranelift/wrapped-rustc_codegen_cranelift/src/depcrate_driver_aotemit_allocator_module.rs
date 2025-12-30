// Generated macro for emit_allocator_module (function)
macro_rules! Depcrate_driver_aotemit_allocator_module {
() => {
// Module: crate::driver::aot
// Provides: {"emit_allocator_module"}
// Dependencies: {}
fn emit_allocator_module (tcx : TyCtxt < '_ >) -> Option < CompiledModule > { let mut allocator_module = make_module (tcx . sess , "allocator_shim" . to_string ()) ; let created_alloc_shim = crate :: allocator :: codegen (tcx , & mut allocator_module) ; if created_alloc_shim { let product = allocator_module . finish () ; match emit_module (tcx . output_filenames (()) , tcx . sess . invocation_temp . as_deref () , & tcx . sess . prof , product . object , ModuleKind :: Allocator , "allocator_shim" . to_owned () , & crate :: debuginfo :: producer (tcx . sess) ,) { Ok (allocator_module) => Some (allocator_module) , Err (err) => tcx . dcx () . fatal (err) , } } else { None } }
};
}
