// Generated macro for Positioned (trait)
macro_rules! Depcrate_streamPositioned {
() => {
// Module: crate::stream
// Provides: {"Positioned"}
// Dependencies: {}
# [doc = " A type which has a position."] pub trait Positioned : StreamOnce { # [doc = " Returns the current position of the stream."] fn position (& self) -> Self :: Position ; }
};
}
