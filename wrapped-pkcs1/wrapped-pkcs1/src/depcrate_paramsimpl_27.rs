// Generated macro for impl_27 (impl)
macro_rules! Depcrate_paramsimpl_27 {
() => {
// Module: crate::params
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for RsaPssParams < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : der :: Header) -> der :: Result < Self > { Ok (Self { hash : reader . context_specific (TagNumber (0) , TagMode :: Explicit) ? . unwrap_or (SHA_1_AI) , mask_gen : reader . context_specific (TagNumber (1) , TagMode :: Explicit) ? . unwrap_or_else (default_mgf1_sha1) , salt_len : reader . context_specific (TagNumber (2) , TagMode :: Explicit) ? . unwrap_or (RsaPssParams :: SALT_LEN_DEFAULT) , trailer_field : reader . context_specific (TagNumber (3) , TagMode :: Explicit) ? . unwrap_or_default () , }) } }
};
}
