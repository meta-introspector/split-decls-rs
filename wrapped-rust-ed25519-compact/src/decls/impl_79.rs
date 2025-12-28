macro_rules! deps {
    () => {
        Mem!();
        SigningState!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Drop for SigningState { fn drop (& mut self) { Mem :: wipe (self . az) ; Mem :: wipe (self . nonce) ; } }
    };
}

impl_79!();