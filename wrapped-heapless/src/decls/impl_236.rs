macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < 'a , LenT : LenType , const N : usize > iter :: FromIterator < & 'a char > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = & 'a char > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push (* c) . unwrap () ; } new } }
    };
}

impl_236!();