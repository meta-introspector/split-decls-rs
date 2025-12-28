macro_rules! deps {
    () => {
        WriteAllVectored!();
    };
}

macro_rules! impl_1242 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for WriteAllVectored < '_ , '_ , W > { }
    };
}

impl_1242!()