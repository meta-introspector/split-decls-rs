macro_rules! deps {
    () => {
        StoreBytes!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl StoreBytes for u64x2_generic { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { let x = u64x2_generic :: read_from_bytes (input) . unwrap () ; qmap (x , | x | x . to_le ()) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { let x = u64x2_generic :: read_from_bytes (input) . unwrap () ; qmap (x , | x | x . to_be ()) } # [inline (always)] fn write_le (self , out : & mut [u8]) { let x = qmap (self , | x | x . to_le ()) ; x . write_to (out) . unwrap () ; } # [inline (always)] fn write_be (self , out : & mut [u8]) { let x = qmap (self , | x | x . to_be ()) ; x . write_to (out) . unwrap () ; } }
    };
}

impl_324!()