macro_rules! deps {
    () => {
        WriteVectored!();
    };
}

macro_rules! impl_1232 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for WriteVectored < '_ , '_ , W > { }
    };
}

impl_1232!()