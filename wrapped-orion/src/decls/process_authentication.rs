macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Poly1305!();
    };
}

macro_rules! process_authentication {
    () => {
        deps!();
        # [doc = " Authenticates the ciphertext, ad and their lengths."] pub (crate) fn process_authentication (auth_ctx : & mut Poly1305 , ad : & [u8] , ciphertext : & [u8] ,) -> Result < () , UnknownCryptoError > { auth_ctx . process_pad_to_blocksize (ad) ? ; auth_ctx . process_pad_to_blocksize (ciphertext) ? ; let (ad_len , ct_len) : (u64 , u64) = match (ad . len () . try_into () , ciphertext . len () . try_into ()) { (Ok (alen) , Ok (clen)) => (alen , clen) , _ => return Err (UnknownCryptoError) , } ; let mut tmp_pad = [0u8 ; 16] ; tmp_pad [0 .. 8] . copy_from_slice (& ad_len . to_le_bytes ()) ; tmp_pad [8 .. 16] . copy_from_slice (& ct_len . to_le_bytes ()) ; auth_ctx . update (tmp_pad . as_ref ()) }
    };
}

process_authentication!();