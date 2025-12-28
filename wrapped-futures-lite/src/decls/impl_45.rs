macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > Unpin for Empty < T > { }
    };
}

impl_45!()