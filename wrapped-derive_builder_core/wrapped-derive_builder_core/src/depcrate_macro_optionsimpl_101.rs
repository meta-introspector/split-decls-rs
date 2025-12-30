// Generated macro for impl_101 (impl)
macro_rules! Depcrate_macro_optionsimpl_101 {
() => {
// Module: crate::macro_options
// Provides: {"impl_101"}
// Dependencies: {}
impl BuildFnError { fn as_existing (& self) -> Option < & Path > { match self { BuildFnError :: Existing (p) => Some (p) , BuildFnError :: Generated (_) => None , } } fn as_generated (& self) -> Option < & BuildFnErrorGenerated > { match self { BuildFnError :: Generated (e) => Some (e) , BuildFnError :: Existing (_) => None , } } }
};
}
