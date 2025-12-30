// Generated macro for impl_37 (impl)
macro_rules! Depcrate_impls_boxximpl_37 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + WriteReady > WriteReady for Box < T > { # [inline] fn write_ready (& mut self) -> Result < bool , Self :: Error > { T :: write_ready (self) } }
};
}
