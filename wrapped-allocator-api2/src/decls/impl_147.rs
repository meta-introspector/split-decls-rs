macro_rules! deps {
    () => {
        SetLenOnDrop!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Drop for SetLenOnDrop < '_ > { # [inline (always)] fn drop (& mut self) { * self . len = self . local_len ; } }
    };
}

impl_147!();