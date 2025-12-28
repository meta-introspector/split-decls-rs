macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T : ComObjectInner > Borrow < T > for ComObject < T > { fn borrow (& self) -> & T { self . get () } }
    };
}

impl_125!()