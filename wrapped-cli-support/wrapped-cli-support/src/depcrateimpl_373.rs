// Generated macro for impl_373 (impl)
macro_rules! Depcrateimpl_373 {
() => {
// Module: crate
// Provides: {"impl_373"}
// Dependencies: {}
impl OutputMode { fn uses_es_modules (& self) -> bool { matches ! (self , OutputMode :: Bundler { .. } | OutputMode :: Web | OutputMode :: Node { module : true } | OutputMode :: Deno) } fn nodejs (& self) -> bool { matches ! (self , OutputMode :: Node { .. }) } fn no_modules (& self) -> bool { matches ! (self , OutputMode :: NoModules { .. }) } fn esm_integration (& self) -> bool { matches ! (self , OutputMode :: Bundler { .. } | OutputMode :: Node { module : true }) } }
};
}
