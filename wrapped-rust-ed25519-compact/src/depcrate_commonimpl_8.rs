// Generated macro for impl_8 (impl)
macro_rules! Depcrate_commonimpl_8 {
() => {
// Module: crate::common
// Provides: {"impl_8"}
// Dependencies: {}
impl Seed { # [doc = " Number of raw bytes in a seed."] pub const BYTES : usize = 32 ; # [doc = " Creates a seed from raw bytes."] pub fn new (seed : [u8 ; Seed :: BYTES]) -> Self { Seed (seed) } # [doc = " Creates a seed from a slice."] pub fn from_slice (seed : & [u8]) -> Result < Self , Error > { let mut seed_ = [0u8 ; Seed :: BYTES] ; if seed . len () != seed_ . len () { return Err (Error :: InvalidSeed) ; } seed_ . copy_from_slice (seed) ; Ok (Seed :: new (seed_)) } # [doc = " Tentatively overwrite the content of the seed with zeros."] pub fn wipe (self) { Mem :: wipe (self . 0) } }
};
}
