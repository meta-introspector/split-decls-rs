macro_rules! deps {
    () => {
        OwnedKey!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Drop for OwnedKey { fn drop (& mut self) { unsafe { RegCloseKey (self . 0) ; } } }
    };
}

impl_101!()