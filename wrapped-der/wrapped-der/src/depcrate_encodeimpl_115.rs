// Generated macro for impl_115 (impl)
macro_rules! Depcrate_encodeimpl_115 {
() => {
// Module: crate::encode
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < T > EncodePem for T where T : Encode + PemLabel + ? Sized , { fn to_pem (& self , line_ending : LineEnding) -> Result < String > { let der_len = usize :: try_from (self . encoded_len () ?) ? ; let pem_len = pem :: encapsulated_len (Self :: PEM_LABEL , line_ending , der_len) ? ; let mut buf = vec ! [0u8 ; pem_len] ; let mut writer = PemWriter :: new (Self :: PEM_LABEL , line_ending , & mut buf) ? ; self . encode (& mut writer) ? ; let actual_len = writer . finish () ? ; buf . truncate (actual_len) ; Ok (String :: from_utf8 (buf) ?) } }
};
}
