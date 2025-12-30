// Generated macro for create_jit_module (function)
macro_rules! Depcrate_driver_jitcreate_jit_module {
() => {
// Module: crate::driver::jit
// Provides: {"create_jit_module"}
// Dependencies: {}
fn create_jit_module (tcx : TyCtxt < '_ >) -> (UnwindModule < JITModule > , CodegenCx) { let crate_info = CrateInfo :: new (tcx , "dummy_target_cpu" . to_string ()) ; let isa = crate :: build_isa (tcx . sess , true) ; let mut jit_builder = JITBuilder :: with_isa (isa , cranelift_module :: default_libcall_names ()) ; crate :: compiler_builtins :: register_functions_for_jit (& mut jit_builder) ; jit_builder . symbol_lookup_fn (dep_symbol_lookup_fn (tcx . sess , crate_info)) ; let mut jit_module = UnwindModule :: new (JITModule :: new (jit_builder) , false) ; let cx = crate :: CodegenCx :: new (tcx , jit_module . isa () , false , sym :: dummy_cgu_name) ; crate :: allocator :: codegen (tcx , & mut jit_module) ; (jit_module , cx) }
};
}
