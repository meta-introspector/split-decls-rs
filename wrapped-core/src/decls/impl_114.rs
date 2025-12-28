macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T : ComObjectInner > Deref for ComObject < T > { type Target = T :: Outer ; # [inline (always)] fn deref (& self) -> & Self :: Target { self . get_box () } }
    };
}

impl_114!()