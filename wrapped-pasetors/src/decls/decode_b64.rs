macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! decode_b64 {
    () => {
        deps!();
        # [doc = " Decode string with Base64 URL-safe and no padding."] pub (crate) fn decode_b64 < T : AsRef < [u8] > > (encoded : T) -> Result < Vec < u8 > , Error > { let inlen = encoded . as_ref () . len () ; let mut buf = vec ! [0u8 ; Base64UrlSafeNoPadding :: encoded_len (inlen) ?] ; let ret : Vec < u8 > = Base64UrlSafeNoPadding :: decode (& mut buf , encoded , None) ? . into () ; Ok (ret) }
    };
}

decode_b64!();