// Generated macro for impl_741 (impl)
macro_rules! Depcrate_ivimpl_741 {
() => {
// Module: crate::iv
// Provides: {"impl_741"}
// Dependencies: {}
impl < const L : usize > FixedLength < L > { # [doc = " Returns the size of the iv in bytes."] # [allow (clippy :: must_use_candidate)] pub fn size (& self) -> usize { L } # [doc = " Constructs a new [`FixedLength`] from pseudo-random bytes."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " * [`Unspecified`]: Returned if there is a failure generating `L` bytes."] pub fn new () -> Result < Self , Unspecified > { let mut iv_bytes = [0u8 ; L] ; rand :: fill (& mut iv_bytes) ? ; Ok (Self (iv_bytes)) } }
};
}
