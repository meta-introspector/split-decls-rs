macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Drop for Error { fn drop (& mut self) { unsafe { (vtable (self . inner . ptr) . object_drop) (self . inner) ; } } }
    };
}

impl_52!();