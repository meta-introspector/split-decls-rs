macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < K , V , S , A > FromIterator < (K , V) > for HashMap < K , V , S , A > where K : Eq + Hash , S : BuildHasher + Default , A : Default + Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let iter = iter . into_iter () ; let mut map = Self :: with_capacity_and_hasher_in (iter . size_hint () . 0 , S :: default () , A :: default ()) ; iter . for_each (| (k , v) | { map . insert (k , v) ; }) ; map } }
    };
}

impl_335!();