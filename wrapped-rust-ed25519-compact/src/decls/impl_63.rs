macro_rules! deps {
    () => {
        Mem!();
        SecretKey!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Drop for SecretKey { fn drop (& mut self) { Mem :: wipe (self . 0) } }
    };
}

impl_63!();