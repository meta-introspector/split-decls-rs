macro_rules! deps {
    () => {
        IndexSlice!();
        Idx!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < I : Idx , T > Default for & IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw (Default :: default ()) } }
    };
}

impl_103!()