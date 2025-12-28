macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! encrypt_decrypt_input_empty {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] fn encrypt_decrypt_input_empty < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out = [0u8 ; 64] ; assert ! (encryptor (key , nonce , 0 , & [0u8 ; 0] , & mut dst_out) . is_err ()) ; assert ! (decryptor (key , nonce , 0 , & [0u8 ; 0] , & mut dst_out) . is_err ()) ; }
    };
}

encrypt_decrypt_input_empty!()