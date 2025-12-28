macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < T : ? Sized > Borrow < T > for Arc < T > { fn borrow (& self) -> & T { self } }
    };
}

impl_247!()