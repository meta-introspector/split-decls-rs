// Generated macro for impl_60 (impl)
macro_rules! Depcrate_compressionimpl_60 {
() => {
// Module: crate::compression
// Provides: {"impl_60"}
// Dependencies: {}
impl Encoder for CombinedEncoder { fn finish (self : Box < Self >) -> Result < () , Error > { self . encoders . into_par_iter () . try_for_each (Encoder :: finish) } }
};
}
