macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! return_if_counter_will_overflow {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] fn return_if_counter_will_overflow < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce , counter : u32 , input : & [u8] ,) -> bool where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { assert ! (! input . is_empty ()) ; let mut dst_out = vec ! [0u8 ; input . len ()] ; let enc_res = encryptor (key , nonce , counter , & [0u8 ; 0] , & mut dst_out) . is_err () ; let dec_res = decryptor (key , nonce , counter , & [0u8 ; 0] , & mut dst_out) . is_err () ; enc_res && dec_res }
    };
}

return_if_counter_will_overflow!()