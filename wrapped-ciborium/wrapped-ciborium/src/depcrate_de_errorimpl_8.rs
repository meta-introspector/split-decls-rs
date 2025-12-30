// Generated macro for impl_8 (impl)
macro_rules! Depcrate_de_errorimpl_8 {
() => {
// Module: crate::de::error
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > Error < T > { # [doc = " A helper method for composing a semantic error"] # [inline] pub fn semantic (offset : impl Into < Option < usize > > , msg : impl Into < String >) -> Self { Self :: Semantic (offset . into () , msg . into ()) } }
};
}
