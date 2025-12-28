macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < T , A : Allocator > Iterator for RawDrain < '_ , T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < T > { unsafe { let item = self . iter . next () ? ; Some (item . read ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_102!()