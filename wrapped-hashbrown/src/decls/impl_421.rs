macro_rules! deps {
    () => {
        Iter!();
        HashSet!();
        IntoIter!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < 'a , T , S , A : Allocator > IntoIterator for & 'a HashSet < T , S , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
    };
}

impl_421!()