macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! open_modified_aad_err {
    () => {
        deps!();
        # [cfg (feature = "safe_api")] # [doc = " When opening sealed data with modified aad, an error should be returned."] fn open_modified_aad_err < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , key : & Key , nonce : & Nonce , input : & [u8] , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let mut dst_out_ct = vec ! [0u8 ; input . len () + tag_size] ; sealer (key , nonce , input , default_aad , & mut dst_out_ct) . unwrap () ; let mut dst_out_pt = input . to_vec () ; assert ! (opener (key , nonce , & dst_out_ct , Some (b"BAD AAD") , & mut dst_out_pt) . is_err ()) ; }
    };
}

open_modified_aad_err!();