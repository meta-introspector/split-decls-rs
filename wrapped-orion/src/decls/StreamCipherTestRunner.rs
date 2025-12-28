macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! StreamCipherTestRunner {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Test runner for stream ciphers."] pub fn StreamCipherTestRunner < Encryptor , Decryptor , Key , Nonce > (encryptor : Encryptor , decryptor : Decryptor , key : Key , nonce : Nonce , counter : u32 , input : & [u8] , expected_ct : Option < & [u8] > ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { if ! input . is_empty () { encrypt_decrypt_out_length (& encryptor , & decryptor , & key , & nonce , input) ; encrypt_decrypt_equals_expected (& encryptor , & decryptor , & key , & nonce , counter , input , expected_ct ,) ; } encrypt_decrypt_input_empty (& encryptor , & decryptor , & key , & nonce) ; initial_counter_overflow_err (& encryptor , & decryptor , & key , & nonce) ; initial_counter_max_ok (& encryptor , & decryptor , & key , & nonce) ; }
    };
}

StreamCipherTestRunner!();