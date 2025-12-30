// Generated macro for DecodePem (trait)
macro_rules! Depcrate_decodeDecodePem {
() => {
// Module: crate::decode
// Provides: {"DecodePem"}
// Dependencies: {}
# [doc = " PEM decoding trait."] # [doc = ""] # [doc = " This trait is automatically impl'd for any type which impls both"] # [doc = " [`DecodeOwned`] and [`PemLabel`]."] # [cfg (feature = "pem")] # [diagnostic :: on_unimplemented (note = "`DecodePem` is auto-impl'd for all lifetime-free types which impl both `Decode` and `PemLabel`")] pub trait DecodePem : DecodeOwned + PemLabel { # [doc = " Try to decode this type from PEM."] fn from_pem (pem : impl AsRef < [u8] >) -> Result < Self , < Self as Decode < 'static > > :: Error > ; }
};
}
