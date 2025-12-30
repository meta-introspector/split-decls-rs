// Generated macro for impl_106 (impl)
macro_rules! Depcrate_ed25519impl_106 {
() => {
// Module: crate::ed25519
// Provides: {"impl_106"}
// Dependencies: {}
impl Noise { # [doc = " Number of raw bytes for a noise component."] pub const BYTES : usize = 16 ; # [doc = " Creates a new noise component from raw bytes."] pub fn new (noise : [u8 ; Noise :: BYTES]) -> Self { Noise (noise) } # [doc = " Creates noise from a slice."] pub fn from_slice (noise : & [u8]) -> Result < Self , Error > { let mut noise_ = [0u8 ; Noise :: BYTES] ; if noise . len () != noise_ . len () { return Err (Error :: InvalidSeed) ; } noise_ . copy_from_slice (noise) ; Ok (Noise :: new (noise_)) } }
};
}
