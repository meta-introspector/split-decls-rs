// Generated macro for EncodePem (trait)
macro_rules! Depcrate_encodeEncodePem {
() => {
// Module: crate::encode
// Provides: {"EncodePem"}
// Dependencies: {}
# [doc = " PEM encoding trait."] # [doc = ""] # [doc = " This trait is automatically impl'd for any type which impls both"] # [doc = " [`Encode`] and [`PemLabel`]."] # [cfg (feature = "pem")] # [diagnostic :: on_unimplemented (note = "`EncodePem` is auto-impl'd for types which impl both `Encode` and `PemLabel`")] pub trait EncodePem : Encode + PemLabel { # [doc = " Try to encode this type as PEM."] fn to_pem (& self , line_ending : LineEnding) -> Result < String > ; }
};
}
