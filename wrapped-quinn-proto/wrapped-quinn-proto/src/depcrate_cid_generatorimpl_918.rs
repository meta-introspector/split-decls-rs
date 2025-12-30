// Generated macro for impl_918 (impl)
macro_rules! Depcrate_cid_generatorimpl_918 {
() => {
// Module: crate::cid_generator
// Provides: {"impl_918"}
// Dependencies: {}
impl HashedConnectionIdGenerator { # [doc = " Create a generator with a random key"] pub fn new () -> Self { Self :: from_key (rand :: rng () . random ()) } # [doc = " Create a generator with a specific key"] # [doc = ""] # [doc = " Allows [`validate`](ConnectionIdGenerator::validate) to recognize a consistent set of"] # [doc = " connection IDs across restarts"] pub fn from_key (key : u64) -> Self { Self { key , lifetime : None , } } # [doc = " Set the lifetime of CIDs created by this generator"] pub fn set_lifetime (& mut self , d : Duration) -> & mut Self { self . lifetime = Some (d) ; self } }
};
}
