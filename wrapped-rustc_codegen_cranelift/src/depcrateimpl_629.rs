// Generated macro for impl_629 (impl)
macro_rules! Depcrateimpl_629 {
() => {
// Module: crate
// Provides: {"impl_629"}
// Dependencies: {}
impl CodegenCx { fn new (tcx : TyCtxt < '_ > , isa : & dyn TargetIsa , debug_info : bool , cgu_name : Symbol) -> Self { assert_eq ! (pointer_ty (tcx) , isa . pointer_type ()) ; let debug_context = if debug_info && ! tcx . sess . target . options . is_like_windows { Some (DebugContext :: new (tcx , isa , cgu_name . as_str ())) } else { None } ; CodegenCx { output_filenames : tcx . output_filenames (()) . clone () , invocation_temp : tcx . sess . invocation_temp . clone () , should_write_ir : crate :: pretty_clif :: should_write_ir (tcx) , global_asm : String :: new () , inline_asm_index : 0 , debug_context , cgu_name , } } }
};
}
