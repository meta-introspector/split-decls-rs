// Generated macro for impl_331 (impl)
macro_rules! Depcrate_core_build_steps_docimpl_331 {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"impl_331"}
// Dependencies: {}
impl Step for ErrorIndex { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/tools/error_index_generator") . default_condition (builder . config . docs) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (ErrorIndex { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , }) ; } # [doc = " Generates the HTML rendered error-index by running the"] # [doc = " `error_index_generator` tool."] fn run (self , builder : & Builder < '_ >) { builder . info (& format ! ("Documenting error index ({})" , self . compilers . target ())) ; let out = builder . doc_out (self . compilers . target ()) ; t ! (fs :: create_dir_all (& out)) ; tool :: ErrorIndex :: command (builder , self . compilers) . arg ("html") . arg (out) . arg (& builder . version) . run (builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: doc ("error-index" , self . compilers . target ()) . built_by (self . compilers . build_compiler ()) ,) } }
};
}
