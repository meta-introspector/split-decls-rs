macro_rules! deps {
    () => {
        HmacImpl!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < H > HmacImpl for SimpleHmac < H > where H : Digest + BlockSizeUser + Clone , { # [inline (always)] fn new_from_slice (key : & [u8]) -> Self { KeyInit :: new_from_slice (key) . expect ("HMAC can take a key of any size") } # [inline (always)] fn update (& mut self , data : & [u8]) { Update :: update (self , data) ; } # [inline (always)] fn finalize (self) -> Output < H > { self . finalize_fixed () } }
    };
}

impl_9!();