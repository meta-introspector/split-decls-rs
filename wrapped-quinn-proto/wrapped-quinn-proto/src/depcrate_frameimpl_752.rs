// Generated macro for impl_752 (impl)
macro_rules! Depcrate_frameimpl_752 {
() => {
// Module: crate::frame
// Provides: {"impl_752"}
// Dependencies: {}
impl From < InvalidFrame > for TransportError { fn from (err : InvalidFrame) -> Self { let mut te = Self :: FRAME_ENCODING_ERROR (err . reason) ; te . frame = err . ty ; te } }
};
}
