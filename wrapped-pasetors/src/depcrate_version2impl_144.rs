// Generated macro for impl_144 (impl)
macro_rules! Depcrate_version2impl_144 {
() => {
// Module: crate::version2
// Provides: {"impl_144"}
// Dependencies: {}
impl Generate < SymmetricKey < V2 > , V2 > for SymmetricKey < V2 > { fn generate () -> Result < SymmetricKey < V2 > , Error > { let mut rng_bytes = vec ! [0u8 ; V2 :: LOCAL_KEY] ; V2 :: validate_local_key (& rng_bytes) ? ; getrandom :: fill (& mut rng_bytes) ? ; Ok (Self { bytes : rng_bytes , phantom : PhantomData , }) } }
};
}
