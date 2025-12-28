macro_rules! StreamXChaCha20Poly1305 {
    () => {
        # [doc = " Streaming XChaCha20Poly1305 state."] pub struct StreamXChaCha20Poly1305 { key : SecretKey , counter : u32 , inonce : [u8 ; INONCEBYTES] , }
    };
}

StreamXChaCha20Poly1305!()