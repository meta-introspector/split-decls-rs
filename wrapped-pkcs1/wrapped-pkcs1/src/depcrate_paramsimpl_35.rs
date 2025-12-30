// Generated macro for impl_35 (impl)
macro_rules! Depcrate_paramsimpl_35 {
() => {
// Module: crate::params
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for RsaOaepParams < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : der :: Header) -> der :: Result < Self > { Ok (Self { hash : reader . context_specific (TagNumber (0) , TagMode :: Explicit) ? . unwrap_or (SHA_1_AI) , mask_gen : reader . context_specific (TagNumber (1) , TagMode :: Explicit) ? . unwrap_or_else (default_mgf1_sha1) , p_source : reader . context_specific (TagNumber (2) , TagMode :: Explicit) ? . unwrap_or_else (default_pempty_string) , }) } }
};
}
