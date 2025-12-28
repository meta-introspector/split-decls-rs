macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_1227 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for Write < '_ , W > { }
    };
}

impl_1227!()