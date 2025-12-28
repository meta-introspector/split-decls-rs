macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < LenT : LenType , const N : usize > Default for String < N , LenT > { fn default () -> Self { Self :: new () } }
    };
}

impl_232!();