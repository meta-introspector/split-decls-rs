// Generated macro for impl_393 (impl)
macro_rules! Depcrateimpl_393 {
() => {
// Module: crate
// Provides: {"impl_393"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R : TryRngCore > std :: io :: Read for RngReader < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , std :: io :: Error > { self . 0 . try_fill_bytes (buf) . map_err (| err | std :: io :: Error :: other (std :: format ! ("RNG error: {err}"))) ? ; Ok (buf . len ()) } }
};
}
