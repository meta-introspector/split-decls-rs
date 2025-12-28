macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl Drop for IUnknown { fn drop (& mut self) { unsafe { (self . vtable () . Release) (core :: mem :: transmute_copy (self)) ; } } }
    };
}

impl_203!();