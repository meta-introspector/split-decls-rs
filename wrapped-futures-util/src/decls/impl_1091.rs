macro_rules! deps {
    () => {
        Close!();
    };
}

macro_rules! impl_1091 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for Close < '_ , W > { }
    };
}

impl_1091!()