// Generated macro for impl_13 (impl)
macro_rules! Depcrate_de_errorimpl_13 {
() => {
// Module: crate::de::error
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Debug > DeError for Error < T > { # [inline] fn custom < U : Display > (msg : U) -> Self { Self :: Semantic (None , msg . to_string ()) } }
};
}
