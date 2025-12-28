macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < T : ComObjectInner > Drop for ComObject < T > { fn drop (& mut self) { unsafe { T :: Outer :: Release (self . ptr . as_ptr ()) ; } } }
    };
}

impl_111!();