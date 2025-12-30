// Generated macro for impl_204 (impl)
macro_rules! Depcrate_encodeimpl_204 {
() => {
// Module: crate::encode
// Provides: {"impl_204"}
// Dependencies: {}
impl < E : RmpWriteErr > From < MarkerWriteError < E > > for ValueWriteError < E > { # [cold] fn from (err : MarkerWriteError < E >) -> Self { match err { MarkerWriteError (err) => Self :: InvalidMarkerWrite (err) , } } }
};
}
