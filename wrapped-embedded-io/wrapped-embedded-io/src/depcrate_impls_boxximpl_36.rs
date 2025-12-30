// Generated macro for impl_36 (impl)
macro_rules! Depcrate_impls_boxximpl_36 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + ReadReady > ReadReady for Box < T > { # [inline] fn read_ready (& mut self) -> Result < bool , Self :: Error > { T :: read_ready (self) } }
};
}
