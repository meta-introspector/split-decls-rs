macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Clone for IUnknown { fn clone (& self) -> Self { unsafe { (self . vtable () . AddRef) (core :: mem :: transmute_copy (self)) ; } Self (self . 0) } }
    };
}

impl_202!()