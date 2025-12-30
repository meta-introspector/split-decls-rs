// Generated macro for impl_478 (impl)
macro_rules! Depcrate_connection_spacesimpl_478 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_478"}
// Dependencies: {}
impl SendableFrames { # [doc = " Returns that no data is available for sending"] pub (super) fn empty () -> Self { Self { acks : false , other : false , } } # [doc = " Whether no data is sendable"] pub (super) fn is_empty (& self) -> bool { ! self . acks && ! self . other } }
};
}
