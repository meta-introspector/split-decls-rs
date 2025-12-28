macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! TestableHpke {
    () => {
        deps!();
        # [doc = " A testable HPKE implementation. This is implemented separately for each HPKE mode."] pub trait TestableHpke : Clone { const HPKE_MODE : u8 ; fn kem_ct_size () -> usize ; fn gen_kp (seed : & [u8]) -> Result < (Vec < u8 > , Vec < u8 >) , UnknownCryptoError > ; fn setup_fresh_sender (pubkey_r : & [u8] , info : & [u8] , psk : & [u8] , psk_id : & [u8] , secret_key_s : & [u8] , public_ct_out : & mut [u8] ,) -> Result < Self , UnknownCryptoError > where Self : Sized ; fn setup_fresh_recipient (enc : & [u8] , secret_key_r : & [u8] , info : & [u8] , psk : & [u8] , psk_id : & [u8] , pubkey_s : & [u8] ,) -> Result < Self , UnknownCryptoError > where Self : Sized ; fn oneshot_seal (pubkey_r : & [u8] , info : & [u8] , psk : & [u8] , psk_id : & [u8] , secret_key_s : & [u8] , plaintext : & [u8] , aad : & [u8] ,) -> Result < (Vec < u8 > , Vec < u8 >) , UnknownCryptoError > where Self : Sized ; # [allow (clippy :: too_many_arguments)] fn oneshot_open (enc : & [u8] , secret_key_r : & [u8] , info : & [u8] , psk : & [u8] , psk_id : & [u8] , pubkey_s : & [u8] , ciphertext : & [u8] , aad : & [u8] ,) -> Result < Vec < u8 > , UnknownCryptoError > where Self : Sized ; fn seal (& mut self , plaintext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > ; fn open (& mut self , ciphertext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > ; fn export (& self , export_context : & [u8] , dst : & mut [u8]) -> Result < () , UnknownCryptoError > ; }
    };
}

TestableHpke!();