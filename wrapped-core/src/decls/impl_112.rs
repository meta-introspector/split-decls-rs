macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < T : ComObjectInner > Clone for ComObject < T > { # [inline (always)] fn clone (& self) -> Self { unsafe { self . ptr . as_ref () . AddRef () ; Self { ptr : self . ptr } } } }
    };
}

impl_112!()