macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! chacha20 {
    () => {
        deps!();
        # [doc = " IETF ChaCha20 as specified in the [RFC 8439](https://tools.ietf.org/html/rfc8439)."] pub mod chacha20 ;
    };
}

chacha20!()