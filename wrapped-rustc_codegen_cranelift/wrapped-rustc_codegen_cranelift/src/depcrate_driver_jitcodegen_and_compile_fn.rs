// Generated macro for codegen_and_compile_fn (function)
macro_rules! Depcrate_driver_jitcodegen_and_compile_fn {
() => {
// Module: crate::driver::jit
// Provides: {"codegen_and_compile_fn"}
// Dependencies: {}
fn codegen_and_compile_fn < 'tcx > (tcx : TyCtxt < 'tcx > , cx : & mut crate :: CodegenCx , cached_context : & mut Context , module : & mut dyn Module , instance : Instance < 'tcx > ,) { if tcx . codegen_instance_attrs (instance . def) . flags . contains (CodegenFnAttrFlags :: NAKED) { tcx . dcx () . span_fatal (tcx . def_span (instance . def_id ()) , "Naked asm is not supported in JIT mode") ; } cranelift_codegen :: timing :: set_thread_profiler (Box :: new (super :: MeasuremeProfiler (tcx . prof . clone () ,))) ; tcx . prof . generic_activity ("codegen and compile fn") . run (| | { let _inst_guard = crate :: PrintOnPanic (| | format ! ("{:?} {}" , instance , tcx . symbol_name (instance) . name)) ; let cached_func = std :: mem :: replace (& mut cached_context . func , Function :: new ()) ; let codegened_func = crate :: base :: codegen_fn (tcx , cx , & mut TypeDebugContext :: default () , cached_func , module , instance ,) ; crate :: base :: compile_fn (cx , & tcx . prof , cached_context , module , codegened_func) ; }) ; }
};
}
