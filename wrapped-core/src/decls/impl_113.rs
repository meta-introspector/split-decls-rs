macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T : ComObjectInner > AsRef < T > for ComObject < T > { # [inline (always)] fn as_ref (& self) -> & T { self . get () } }
    };
}

impl_113!()