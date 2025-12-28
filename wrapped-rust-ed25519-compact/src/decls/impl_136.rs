macro_rules! deps {
    () => {
        Mem!();
        DHOutput!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl Drop for DHOutput { fn drop (& mut self) { Mem :: wipe (self . 0) } }
    };
}

impl_136!()