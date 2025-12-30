// Generated macro for impl_217 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_217 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_217"}
// Dependencies: {}
impl Step for Docs { type Output = Option < GeneratedTarball > ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let default = run . builder . config . docs ; run . alias ("rust-docs") . default_condition (default) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Docs { host : run . target }) ; } # [doc = " Builds the `rust-docs` installer component."] fn run (self , builder : & Builder < '_ >) -> Option < GeneratedTarball > { let host = self . host ; builder . run_default_doc_steps () ; let dest = "share/doc/rust/html" ; let mut tarball = Tarball :: new (builder , "rust-docs" , & host . triple) ; tarball . set_product_name ("Rust Documentation") ; tarball . add_bulk_dir (builder . doc_out (host) , dest) ; tarball . add_file (builder . src . join ("src/doc/robots.txt") , dest , FileType :: Regular) ; tarball . add_file (builder . src . join ("src/doc/sitemap.txt") , dest , FileType :: Regular) ; Some (tarball . generate ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: dist ("docs" , self . host)) } }
};
}
