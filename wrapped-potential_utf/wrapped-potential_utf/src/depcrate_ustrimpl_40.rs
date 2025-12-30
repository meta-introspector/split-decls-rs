// Generated macro for impl_40 (impl)
macro_rules! Depcrate_ustrimpl_40 {
() => {
// Module: crate::ustr
// Provides: {"impl_40"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (feature = "zerovec")] unsafe impl zerovec :: ule :: VarULE for PotentialUtf8 { # [inline] fn validate_bytes (_ : & [u8]) -> Result < () , zerovec :: ule :: UleError > { Ok (()) } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { PotentialUtf8 :: from_bytes (bytes) } }
};
}
