macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! initial_counter_overflow_err {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Test that an initial counter will not overflow the internal."] fn initial_counter_overflow_err < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out = [0u8 ; 128] ; assert ! (encryptor (key , nonce , u32 :: MAX , & [0u8 ; 65] , & mut dst_out) . is_err ()) ; assert ! (decryptor (key , nonce , u32 :: MAX , & [0u8 ; 65] , & mut dst_out) . is_err ()) ; }
    };
}

initial_counter_overflow_err!();