// Generated macro for impl_363 (impl)
macro_rules! Depcrate_rngs_threadimpl_363 {
() => {
// Module: crate::rngs::thread
// Provides: {"impl_363"}
// Dependencies: {}
impl ThreadRng { # [doc = " Immediately reseed the generator"] # [doc = ""] # [doc = " This discards any remaining random data in the cache."] pub fn reseed (& mut self) -> Result < () , OsError > { let rng = unsafe { & mut * self . rng . get () } ; rng . reseed () } }
};
}
