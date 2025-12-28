macro_rules! deps {
    () => {
        SecretKey!();
        Mem!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl Drop for SecretKey { fn drop (& mut self) { Mem :: wipe (self . 0) } }
    };
}

impl_143!();