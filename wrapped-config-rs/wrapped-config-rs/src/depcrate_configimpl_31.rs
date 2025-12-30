// Generated macro for impl_31 (impl)
macro_rules! Depcrate_configimpl_31 {
() => {
// Module: crate::config
// Provides: {"impl_31"}
// Dependencies: {}
impl Source for Config { fn clone_into_box (& self) -> Box < dyn Source + Send + Sync > { Box :: new ((* self) . clone ()) } fn collect (& self) -> Result < Map < String , Value > > { self . cache . clone () . into_table () } }
};
}
