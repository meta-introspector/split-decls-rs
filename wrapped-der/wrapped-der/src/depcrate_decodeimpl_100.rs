// Generated macro for impl_100 (impl)
macro_rules! Depcrate_decodeimpl_100 {
() => {
// Module: crate::decode
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < T : DecodeOwned < Error = Error > + PemLabel > DecodePem for T { fn from_pem (pem : impl AsRef < [u8] >) -> Result < T , Error > { let mut reader = PemReader :: new (pem . as_ref ()) ? ; Self :: validate_pem_label (reader . type_label ()) . map_err (Error :: from) ? ; T :: decode (& mut reader) } }
};
}
