// Generated macro for Signature (struct)
macro_rules! Depcrate_schnorrSignature {
() => {
// Module: crate::schnorr
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Taproot Schnorr signature as defined in [BIP340]."] # [doc = ""] # [doc = " [BIP340]: https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki"] # [derive (Copy , Clone)] pub struct Signature { r : FieldElement , s : NonZeroScalar , }
};
}
