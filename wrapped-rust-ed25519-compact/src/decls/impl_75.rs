macro_rules! deps {
    () => {
        VerifyingState!();
        Mem!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Drop for VerifyingState { fn drop (& mut self) { Mem :: wipe (self . signature . 0) ; } }
    };
}

impl_75!()