macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! initial_counter_max_ok {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Test that processing one block does not fail on the largest possible initial block counter."] fn initial_counter_max_ok < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out = [0u8 ; 64] ; assert ! (encryptor (key , nonce , u32 :: MAX , & [0u8 ; 64] , & mut dst_out) . is_ok ()) ; assert ! (decryptor (key , nonce , u32 :: MAX , & [0u8 ; 64] , & mut dst_out) . is_ok ()) ; }
    };
}

initial_counter_max_ok!();