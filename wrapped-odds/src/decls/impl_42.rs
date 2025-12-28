macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a , T > Iterator for SliceCopyIter < 'a , T > where T : Copy , { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . ptr != self . end { unsafe { let elt = Some (* self . ptr) ; self . ptr = self . ptr . offset (1) ; elt } } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = (self . end as usize - self . ptr as usize) / size_of :: < T > () ; (len , Some (len)) } fn count (self) -> usize { self . len () } fn last (mut self) -> Option < Self :: Item > { self . next_back () } }
    };
}

impl_42!()