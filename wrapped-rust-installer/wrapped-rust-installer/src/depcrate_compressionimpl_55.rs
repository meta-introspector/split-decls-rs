// Generated macro for impl_55 (impl)
macro_rules! Depcrate_compressionimpl_55 {
() => {
// Module: crate::compression
// Provides: {"impl_55"}
// Dependencies: {}
impl < W : Send + Write > Encoder for GzEncoder < W > { fn finish (self : Box < Self >) -> Result < () , Error > { GzEncoder :: finish (* self) . context ("failed to finish .gz file") ? ; Ok (()) } }
};
}
