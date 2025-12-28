macro_rules! deps {
    () => {
        Vec!();
        LenType!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize , const M : usize > From < [T ; M] > for Vec < T , N , LenT > { fn from (array : [T ; M]) -> Self { Self :: from_array (array) } }
    };
}

impl_292!()