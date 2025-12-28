macro_rules! deps {
    () => {
        LenType!();
        Vec!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > Default for Vec < T , N , LenT > { fn default () -> Self { Self :: new () } }
    };
}

impl_289!()