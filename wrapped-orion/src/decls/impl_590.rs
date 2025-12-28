macro_rules! deps {
    () => {
        UnknownCryptoError!();
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl DHKEM_X25519_SHA256_CHACHA20 { # [doc = " Size of the HPKE suite KEM ciphertext/encapsulated key."] pub const KEM_CT_SIZE : usize = 32 ; # [doc = " Size of the HPKE suite KEM shared secret."] pub const KEM_SS_SIZE : usize = 32 ; # [doc = " Version identifier for this HPKE scheme."] pub const VERSION_ID : & [u8 ; 7] = b"HPKE-v1" ; # [doc = " HPKE ID for this HPKE scheme."] pub const HPKE_ID : & [u8 ; 4] = b"HPKE" ; # [doc = " KEM ID for this HPKE scheme's KEM (in LE bytes)."] pub const KEM_ID : [u8 ; 2] = 0x0020u16 . to_be_bytes () ; # [doc = " KDF ID for this HPKE scheme's KDF (in LE bytes)."] pub const KDF_ID : [u8 ; 2] = 0x0001u16 . to_be_bytes () ; # [doc = " AEAD ID for this HPKE scheme's AEAD (in LE bytes)."] pub const AEAD_ID : [u8 ; 2] = 0x0003u16 . to_be_bytes () ; # [doc = " The maximum length of `export` secret that may be requested."] pub const EXPORT_SECRET_MAXLEN : usize = (255 * Self :: NH) ; # [doc = " Nonce size for this suite's AEAD (<https://www.rfc-editor.org/rfc/rfc9180.html#section-7.3>)."] pub const NN : usize = 12 ; # [doc = " Output size for this suite's KDF (<https://www.rfc-editor.org/rfc/rfc9180.html#section-7.2>)."] pub const NH : usize = 32 ; fn compute_nonce (& self) -> chacha20poly1305 :: Nonce { let mut n = [0u8 ; crate :: hazardous :: stream :: chacha20 :: IETF_CHACHA_NONCESIZE] ; n [4 .. 12] . copy_from_slice (& self . ctr . to_be_bytes ()) ; xor_slices ! (self . base_nonce , n) ; chacha20poly1305 :: Nonce :: from (n) } fn would_overflow (& self) -> bool { self . ctr . checked_add (1) . is_none () } fn increment_seq (& mut self) -> Result < () , UnknownCryptoError > { if let Some (next_seq) = self . ctr . checked_add (1) { self . ctr = next_seq ; } else { return Err (UnknownCryptoError) ; } if self . ctr as u128 >= ((1u128 << (8u128 * Self :: NN as u128)) - 1) { return Err (UnknownCryptoError) ; } Ok (()) } # [doc = " Minimum length: <https://www.rfc-editor.org/rfc/rfc9180.html#section-5.1.4>"] # [doc = " Maximum length: <https://www.rfc-editor.org/rfc/rfc9180.html#section-7.2.1>"] fn check_psk_length (psk : & [u8] , psk_id : & [u8]) -> Result < () , UnknownCryptoError > { if psk . len () < 32 { return Err (UnknownCryptoError) ; } Self :: check_input_max_lengths (psk) ? ; Self :: check_input_max_lengths (psk_id) } # [doc = " Maximum length: <https://www.rfc-editor.org/rfc/rfc9180.html#section-7.2.1>"] fn check_input_max_lengths (input : & [u8]) -> Result < () , UnknownCryptoError > { if input . len () > 64 { return Err (UnknownCryptoError) ; } Ok (()) } }
    };
}

impl_590!();