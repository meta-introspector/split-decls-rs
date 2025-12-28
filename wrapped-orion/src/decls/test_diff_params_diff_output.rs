macro_rules! deps {
    () => {
        UnknownCryptoError!();
        TestingRandom!();
    };
}

macro_rules! test_diff_params_diff_output {
    () => {
        deps!();
        # [cfg (test)] # [cfg (feature = "safe_api")] # [doc = " Test that encrypting using different secret-key/nonce/initial-counter combinations yields different"] # [doc = " ciphertexts."] pub fn test_diff_params_diff_output < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor ,) where Key : TestingRandom + PartialEq < Key > , Nonce : TestingRandom + PartialEq < Nonce > , Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let input = & [0u8 ; 16] ; let sk1 = Key :: gen () ; let sk2 = Key :: gen () ; assert ! (sk1 != sk2) ; let n1 = Nonce :: gen () ; let n2 = Nonce :: gen () ; assert ! (n1 != n2) ; let c1 = 0u32 ; let c2 = 1u32 ; let mut dst_out_ct = vec ! [0u8 ; input . len ()] ; let mut dst_out_pt = vec ! [0u8 ; input . len ()] ; encryptor (& sk1 , & n1 , c1 , input , & mut dst_out_ct) . unwrap () ; decryptor (& sk2 , & n1 , c1 , & dst_out_ct , & mut dst_out_pt) . unwrap () ; assert_ne ! (& dst_out_pt [..] , input) ; encryptor (& sk1 , & n1 , c1 , input , & mut dst_out_ct) . unwrap () ; decryptor (& sk1 , & n2 , c1 , & dst_out_ct , & mut dst_out_pt) . unwrap () ; assert_ne ! (& dst_out_pt [..] , input) ; encryptor (& sk1 , & n1 , c1 , input , & mut dst_out_ct) . unwrap () ; decryptor (& sk1 , & n1 , c2 , & dst_out_ct , & mut dst_out_pt) . unwrap () ; assert_ne ! (& dst_out_pt [..] , input) ; }
    };
}

test_diff_params_diff_output!();