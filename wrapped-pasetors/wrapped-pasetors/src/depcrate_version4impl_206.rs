// Generated macro for impl_206 (impl)
macro_rules! Depcrate_version4impl_206 {
() => {
// Module: crate::version4
// Provides: {"impl_206"}
// Dependencies: {}
impl Generate < SymmetricKey < V4 > , V4 > for SymmetricKey < V4 > { fn generate () -> Result < SymmetricKey < V4 > , Error > { let mut rng_bytes = vec ! [0u8 ; V4 :: LOCAL_KEY] ; V4 :: validate_local_key (& rng_bytes) ? ; getrandom :: fill (& mut rng_bytes) ? ; Ok (Self { bytes : rng_bytes , phantom : PhantomData , }) } }
};
}
