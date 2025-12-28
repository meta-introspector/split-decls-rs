macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Drop for Key { fn drop (& mut self) { unsafe { RegCloseKey (self . 0) } ; } }
    };
}

impl_60!()