macro_rules! deps {
    () => {
        Vec!();
        LenType!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > FromIterator < T > for Vec < T , N , LenT > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut vec = Self :: new () ; for i in iter { vec . push (i) . ok () . expect ("Vec::from_iter overflow") ; } vec } }
    };
}

impl_302!();