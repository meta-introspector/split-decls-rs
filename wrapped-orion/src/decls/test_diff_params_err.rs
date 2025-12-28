macro_rules! deps {
    () => {
        UnknownCryptoError!();
        TestingRandom!();
    };
}

macro_rules! test_diff_params_err {
    () => {
        deps!();
        # [cfg (test)] # [cfg (feature = "safe_api")] # [doc = " Test that sealing and opening with different secret-key/nonce yields an error."] pub fn test_diff_params_err < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , input : & [u8] , tag_size : usize ,) where Key : TestingRandom + PartialEq < Key > , Nonce : TestingRandom + PartialEq < Nonce > , Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let sk1 = Key :: gen () ; let sk2 = Key :: gen () ; assert ! (sk1 != sk2) ; let n1 = Nonce :: gen () ; let n2 = Nonce :: gen () ; assert ! (n1 != n2) ; let mut dst_out_ct = vec ! [0u8 ; input . len () + tag_size] ; let mut dst_out_pt = vec ! [0u8 ; input . len ()] ; sealer (& sk1 , & n1 , input , None , & mut dst_out_ct) . unwrap () ; assert ! (opener (& sk2 , & n1 , & dst_out_ct , None , & mut dst_out_pt) . is_err ()) ; sealer (& sk1 , & n1 , input , None , & mut dst_out_ct) . unwrap () ; assert ! (opener (& sk1 , & n2 , & dst_out_ct , None , & mut dst_out_pt) . is_err ()) ; }
    };
}

test_diff_params_err!()