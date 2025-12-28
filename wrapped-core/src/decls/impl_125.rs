macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T : ComObjectInner > Borrow < T > for ComObject < T > { fn borrow (& self) -> & T { self . get () } }
    };
}

impl_125!();