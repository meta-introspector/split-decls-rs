macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < T : ? Sized > AsRef < T > for Arc < T > { fn as_ref (& self) -> & T { self } }
    };
}

impl_246!()