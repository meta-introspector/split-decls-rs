macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! seal_plaintext_length {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " Related bug: <https://github.com/orion-rs/orion/issues/52>"] # [doc = " Test input sizes when using seal()."] fn seal_plaintext_length < Sealer , Key , Nonce > (sealer : & Sealer , key : & Key , nonce : & Nonce , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let input_0 = vec ! [0u8 ; 0] ; let mut dst_out_ct_0 = vec ! [0u8 ; input_0 . len () + tag_size] ; assert ! (sealer (key , nonce , & input_0 , default_aad , & mut dst_out_ct_0) . is_ok ()) ; let input_1 = vec ! [0u8 ; 1] ; let mut dst_out_ct_1 = vec ! [0u8 ; input_1 . len () + tag_size] ; assert ! (sealer (key , nonce , & input_1 , default_aad , & mut dst_out_ct_1) . is_ok ()) ; let input_128 = vec ! [0u8 ; 128] ; let mut dst_out_ct_128 = vec ! [0u8 ; input_128 . len () + tag_size] ; assert ! (sealer (key , nonce , & input_128 , default_aad , & mut dst_out_ct_128) . is_ok ()) ; }
    };
}

seal_plaintext_length!()