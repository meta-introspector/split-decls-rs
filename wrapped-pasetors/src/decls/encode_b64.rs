macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! encode_b64 {
    () => {
        deps!();
        # [doc = " Encode bytes with Base64 URL-safe and no padding."] pub (crate) fn encode_b64 < T : AsRef < [u8] > > (bytes : T) -> Result < String , Error > { let inlen = bytes . as_ref () . len () ; let mut buf = vec ! [0u8 ; Base64UrlSafeNoPadding :: encoded_len (inlen) ?] ; let ret : String = Base64UrlSafeNoPadding :: encode_to_str (& mut buf , bytes) ? . into () ; Ok (ret) }
    };
}

encode_b64!()