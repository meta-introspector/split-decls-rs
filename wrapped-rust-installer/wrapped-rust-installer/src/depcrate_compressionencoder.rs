// Generated macro for Encoder (trait)
macro_rules! Depcrate_compressionEncoder {
() => {
// Module: crate::compression
// Provides: {"Encoder"}
// Dependencies: {}
pub (crate) trait Encoder : Send + Write { fn finish (self : Box < Self >) -> Result < () , Error > ; }
};
}
