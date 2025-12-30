// Generated macro for impl_103 (impl)
macro_rules! Depcrate_decodeimpl_103 {
() => {
// Module: crate::decode
// Provides: {"impl_103"}
// Dependencies: {}
impl < E : RmpReadErr > From < MarkerReadError < E > > for ValueReadError < E > { # [cold] fn from (err : MarkerReadError < E >) -> Self { match err { MarkerReadError (err) => Self :: InvalidMarkerRead (err) , } } }
};
}
