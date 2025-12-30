// Generated macro for impl_111 (impl)
macro_rules! Depcrate_decodeimpl_111 {
() => {
// Module: crate::decode
// Provides: {"impl_111"}
// Dependencies: {}
impl < E : RmpReadErr > From < MarkerReadError < E > > for NumValueReadError < E > { # [cold] fn from (err : MarkerReadError < E >) -> Self { match err { MarkerReadError (err) => Self :: InvalidMarkerRead (err) , } } }
};
}
