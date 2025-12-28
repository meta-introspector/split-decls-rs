macro_rules! deps {
    () => {
        RawIntoIter!();
        RawTable!();
        IntoIter!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T , A : Allocator > IntoIterator for RawTable < T , A > { type Item = T ; type IntoIter = RawIntoIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> RawIntoIter < T , A > { unsafe { let iter = self . iter () ; self . into_iter_from (iter) } } }
    };
}

impl_67!()