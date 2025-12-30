// Generated macro for impl_93 (impl)
macro_rules! Depcrate_signimpl_93 {
() => {
// Module: crate::sign
// Provides: {"impl_93"}
// Dependencies: {}
impl TryFrom < & Signature > for InnerSignature { type Error = SigningError ; fn try_from (signature : & Signature) -> Result < Self , Self :: Error > { let s_bytes : & Array < u8 , _ > = (signature . s_bytes ()) . into () ; let s = Option :: from (EdwardsScalar :: from_canonical_bytes (s_bytes)) . ok_or (SigningError :: InvalidSignatureSComponent) ? ; let r = CompressedEdwardsY :: from (* signature . r_bytes ()) . decompress () . into_option () . map (| point | point . to_edwards ()) . ok_or (SigningError :: InvalidSignatureRComponent) ? ; Ok (Self { r , s }) } }
};
}
