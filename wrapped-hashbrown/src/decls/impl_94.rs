macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T , A : Allocator > Iterator for RawIntoIter < T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < T > { unsafe { Some (self . iter . next () ? . read ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_94!()