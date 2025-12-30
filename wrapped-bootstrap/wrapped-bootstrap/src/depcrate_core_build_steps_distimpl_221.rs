// Generated macro for impl_221 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_221 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_221"}
// Dependencies: {}
impl Step for RustcDocs { type Output = GeneratedTarball ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . alias ("rustc-docs") . default_condition (builder . config . compiler_docs) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RustcDocs { target : run . target }) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let target = self . target ; builder . run_default_doc_steps () ; let mut tarball = Tarball :: new (builder , "rustc-docs" , & target . triple) ; tarball . set_product_name ("Rustc Documentation") ; tarball . add_bulk_dir (builder . compiler_doc_out (target) , "share/doc/rust/html/rustc") ; tarball . generate () } }
};
}
