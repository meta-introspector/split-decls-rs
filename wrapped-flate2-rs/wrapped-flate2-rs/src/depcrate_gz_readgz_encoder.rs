// Generated macro for gz_encoder (function)
macro_rules! Depcrate_gz_readgz_encoder {
() => {
// Module: crate::gz::read
// Provides: {"gz_encoder"}
// Dependencies: {}
pub fn gz_encoder < R : Read > (inner : bufread :: GzEncoder < BufReader < R > >) -> GzEncoder < R > { GzEncoder { inner } }
};
}
