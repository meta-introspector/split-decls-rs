macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [pinned_drop] impl < T > PinnedDrop for AssertUnmoved < T > { fn drop (self : Pin < & mut Self >) { if ! panicking () && self . this_addr != 0 { let cur_this = & * self as * const Self as usize ; assert_eq ! (self . this_addr , cur_this , "AssertUnmoved moved before drop") ; } } }
    };
}

impl_81!()