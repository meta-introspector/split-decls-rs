macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < 'a , LenT : LenType , const N : usize > iter :: FromIterator < & 'a str > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push_str (c) . unwrap () ; } new } }
    };
}

impl_237!()