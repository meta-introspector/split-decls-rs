macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl digest :: KeyInit for Hasher { # [inline] fn new (key : & digest :: Key < Self >) -> Self { let key_bytes : [u8 ; 32] = (* key) . into () ; Hasher :: new_keyed (& key_bytes) } }
    };
}

impl_137!();