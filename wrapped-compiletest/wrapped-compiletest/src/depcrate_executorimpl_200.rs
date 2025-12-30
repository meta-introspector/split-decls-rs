// Generated macro for impl_200 (impl)
macro_rules! Depcrate_executorimpl_200 {
() => {
// Module: crate::executor
// Provides: {"impl_200"}
// Dependencies: {}
impl RunnableTest { fn new (test : & CollectedTest) -> Self { let config = Arc :: clone (& test . config) ; let testpaths = test . testpaths . clone () ; let revision = test . revision . clone () ; Self { config , testpaths , revision } } fn run (& self , stdout : & dyn ConsoleOut , stderr : & dyn ConsoleOut) { __rust_begin_short_backtrace (| | { crate :: runtest :: run (Arc :: clone (& self . config) , stdout , stderr , & self . testpaths , self . revision . as_deref () ,) ; }) ; } }
};
}
