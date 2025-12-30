// Generated macro for impl_243 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_243 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_243"}
// Dependencies: {}
impl Step for Analysis { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = should_build_extended_tool (run . builder , "analysis") ; run . alias ("rust-analysis") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Analysis { build_compiler : run . builder . compiler (1 , run . builder . config . host_target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let compiler = self . build_compiler ; let target = self . target ; if skip_host_target_lib (builder , compiler) { return None ; } let src = builder . stage_out (compiler , Mode :: Std) . join (target) . join (builder . cargo_dir ()) . join ("deps") . join ("save-analysis") ; t ! (std :: fs :: create_dir_all (& src)) ; let mut removed = src . clone () ; removed . push ("removed.json") ; let mut f = t ! (std :: fs :: File :: create (removed)) ; t ! (write ! (f , r#"{{ "warning": "The `rust-analysis` component has been removed." }}"#)) ; let mut tarball = Tarball :: new (builder , "rust-analysis" , & target . triple) ; tarball . include_target_in_component_name (true) ; tarball . add_dir (src , format ! ("lib/rustlib/{}/analysis" , target . triple)) ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("analysis" , self . target) . built_by (self . build_compiler)) } }
};
}
