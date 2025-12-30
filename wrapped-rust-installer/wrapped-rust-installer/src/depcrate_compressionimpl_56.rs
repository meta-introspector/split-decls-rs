// Generated macro for impl_56 (impl)
macro_rules! Depcrate_compressionimpl_56 {
() => {
// Module: crate::compression
// Provides: {"impl_56"}
// Dependencies: {}
impl < W : Send + Write > Encoder for XzEncoder < W > { fn finish (self : Box < Self >) -> Result < () , Error > { XzEncoder :: finish (* self) . context ("failed to finish .xz file") ? ; Ok (()) } }
};
}
