// Generated macro for impl_219 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_219 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_219"}
// Dependencies: {}
impl Step for JsonDocs { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = run . builder . config . docs ; run . alias ("rust-docs-json") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (JsonDocs { build_compiler : run . builder . compiler_for_std (run . builder . top_stage) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let target = self . target ; let directory = builder . ensure (crate :: core :: build_steps :: doc :: Std :: from_build_compiler (self . build_compiler , target , DocumentationFormat :: Json ,)) ; let dest = "share/doc/rust/json" ; let mut tarball = Tarball :: new (builder , "rust-docs-json" , & target . triple) ; tarball . set_product_name ("Rust Documentation In JSON Format") ; tarball . is_preview (true) ; tarball . add_bulk_dir (directory , dest) ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("json-docs" , self . target) . built_by (self . build_compiler)) } }
};
}
