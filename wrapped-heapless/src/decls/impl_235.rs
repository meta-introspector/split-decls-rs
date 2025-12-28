macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < LenT : LenType , const N : usize > iter :: FromIterator < char > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push (c) . unwrap () ; } new } }
    };
}

impl_235!()