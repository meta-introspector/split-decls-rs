macro_rules! deps {
    () => {
        Idx!();
        IndexSlice!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < I : Idx , T > Default for & mut IndexSlice < I , T > { # [inline] fn default () -> Self { IndexSlice :: from_raw_mut (Default :: default ()) } }
    };
}

impl_104!();