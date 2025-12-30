// Generated macro for impl_91 (impl)
macro_rules! Depcrate_builderimpl_91 {
() => {
// Module: crate::builder
// Provides: {"impl_91"}
// Dependencies: {}
impl < P , R : ? Sized > PasswordRecipientInfoBuilder < P , R > where P : PwriEncryptor , R : CryptoRng , { # [doc = " Wrap the content-encryption key according to [RFC 3211, §2.3.1]:"] # [doc = "     ...."] # [doc = "     The formatted CEK block then looks as follows:"] # [doc = "     CEK byte count || check value || CEK || padding (if required)"] # [doc = ""] # [doc = " [RFC 3211, §2.3.1]: https://www.rfc-editor.org/rfc/rfc3211#section-2.3.1"] fn pad_content_encryption_key (& mut self , content_encryption_key : & [u8] , rng : & mut R ,) -> Result < Vec < u8 > > { let content_encryption_key_length = content_encryption_key . len () ; let padded_key_length_wo_padding = 1 + 3 + content_encryption_key_length ; let key_enc_alg_blocklength_bytes = P :: BLOCK_LENGTH_BITS / 8 ; let padding_length = (2 * key_enc_alg_blocklength_bytes) . saturating_sub (padded_key_length_wo_padding) ; let cek_byte_count : u8 = content_encryption_key . len () . try_into () . map_err (| _ | { Error :: Builder ("Content encryption key length must not exceed 255" . to_string ()) }) ? ; let mut padded_cek : Vec < u8 > = Vec :: with_capacity (4 + content_encryption_key_length + padding_length) ; padded_cek . push (cek_byte_count) ; padded_cek . push (0xff ^ content_encryption_key [0]) ; padded_cek . push (0xff ^ content_encryption_key [1]) ; padded_cek . push (0xff ^ content_encryption_key [2]) ; padded_cek . extend_from_slice (content_encryption_key) ; if padding_length > 0 { let mut padding = vec ! [0_u8 ; padding_length] ; rng . fill_bytes (padding . as_mut_slice ()) ; padded_cek . append (& mut padding) ; } Ok (padded_cek) } }
};
}
