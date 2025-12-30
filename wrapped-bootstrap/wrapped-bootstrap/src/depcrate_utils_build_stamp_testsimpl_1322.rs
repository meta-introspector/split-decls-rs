// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_utils_build_stamp_testsimpl_1322 {
() => {
// Module: crate::utils::build_stamp::tests
// Provides: {"impl_1322"}
// Dependencies: {}
impl TestCtx { pub fn new () -> Self { let directory = TempDir :: new () . expect ("cannot create temporary directory") ; eprintln ! ("Running test in {}" , directory . path () . display ()) ; Self { directory } } pub fn dir (& self) -> & Path { self . directory . path () } # [doc = " Starts a new invocation of bootstrap that executes `kind` as its top level command"] # [doc = " (i.e. `x <kind>`). Returns a builder that configures the created config through CLI flags."] pub fn config (& self , kind : & str) -> ConfigBuilder { ConfigBuilder :: from_args (& [kind] , self . directory . path () . to_owned ()) } }
};
}
