macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl < T , S , A > FromIterator < T > for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher + Default , A : Default + Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut set = Self :: with_hasher_in (Default :: default () , Default :: default ()) ; set . extend (iter) ; set } }
    };
}

impl_400!();