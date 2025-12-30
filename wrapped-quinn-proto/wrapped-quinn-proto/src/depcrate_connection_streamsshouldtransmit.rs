// Generated macro for ShouldTransmit (struct)
macro_rules! Depcrate_connection_streamsShouldTransmit {
() => {
// Module: crate::connection::streams
// Provides: {"ShouldTransmit"}
// Dependencies: {}
# [doc = " Indicates whether a frame needs to be transmitted"] # [doc = ""] # [doc = " This type wraps around bool and uses the `#[must_use]` attribute in order"] # [doc = " to prevent accidental loss of the frame transmission requirement."] # [derive (Copy , Clone , Debug , Default , Eq , PartialEq)] # [must_use = "A frame might need to be enqueued"] pub struct ShouldTransmit (bool) ;
};
}
