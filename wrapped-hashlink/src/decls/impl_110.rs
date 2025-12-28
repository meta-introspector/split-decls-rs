macro_rules! deps {
    () => {
        FreeLink!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < K , V > Clone for FreeLink < K , V > { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_110!()