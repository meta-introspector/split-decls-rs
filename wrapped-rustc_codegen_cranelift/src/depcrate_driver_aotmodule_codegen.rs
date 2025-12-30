// Generated macro for module_codegen (function)
macro_rules! Depcrate_driver_aotmodule_codegen {
() => {
// Module: crate::driver::aot
// Provides: {"module_codegen"}
// Dependencies: {}
fn module_codegen (tcx : TyCtxt < '_ > , (global_asm_config , cgu_name , token) : (Arc < GlobalAsmConfig > , rustc_span :: Symbol , ConcurrencyLimiterToken ,) ,) -> OngoingModuleCodegen { let mut module = make_module (tcx . sess , cgu_name . as_str () . to_string ()) ; let (mut cx , codegened_functions) = codegen_cgu_content (tcx , & mut module , cgu_name) ; let cgu_name = cgu_name . as_str () . to_owned () ; let producer = crate :: debuginfo :: producer (tcx . sess) ; let profiler = tcx . prof . clone () ; OngoingModuleCodegen :: Async (std :: thread :: spawn (move | | { profiler . clone () . generic_activity_with_arg ("compile functions" , & * cgu_name) . run (| | { cranelift_codegen :: timing :: set_thread_profiler (Box :: new (super :: MeasuremeProfiler (profiler . clone () ,))) ; let mut cached_context = Context :: new () ; for codegened_func in codegened_functions { crate :: base :: compile_fn (& mut cx , & profiler , & mut cached_context , & mut module , codegened_func ,) ; } }) ; let global_asm_object_file = profiler . generic_activity_with_arg ("compile assembly" , & * cgu_name) . run (| | { crate :: global_asm :: compile_global_asm (& global_asm_config , & cgu_name , & cx . global_asm , cx . invocation_temp . as_deref () ,) }) ? ; let codegen_result = profiler . generic_activity_with_arg ("write object file" , & * cgu_name) . run (| | { emit_cgu (& global_asm_config . output_filenames , cx . invocation_temp . as_deref () , & profiler , cgu_name , module , cx . debug_context , global_asm_object_file , & producer ,) }) ; std :: mem :: drop (token) ; codegen_result })) }
};
}
