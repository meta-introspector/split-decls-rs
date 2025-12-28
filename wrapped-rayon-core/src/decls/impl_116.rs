macro_rules! deps {
    () => {
        Terminator!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'a > Drop for Terminator < 'a > { fn drop (& mut self) { self . 0 . terminate () } }
    };
}

impl_116!();