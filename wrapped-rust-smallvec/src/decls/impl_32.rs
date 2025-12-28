macro_rules! deps {
    () => {
        SmallVec!();
        Splice!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < I : Iterator , const N : usize > Drop for Splice < '_ , I , N > { fn drop (& mut self) { self . drain . by_ref () . for_each (drop) ; self . drain . iter = [] . iter () ; unsafe { if self . drain . tail_len == 0 { self . drain . vec . as_mut () . extend (self . replace_with . by_ref ()) ; return ; } if ! self . drain . fill (& mut self . replace_with) { return ; } let (lower_bound , _upper_bound) = self . replace_with . size_hint () ; if lower_bound > 0 { self . drain . move_tail (lower_bound) ; if ! self . drain . fill (& mut self . replace_with) { return ; } } let mut collected = self . replace_with . by_ref () . collect :: < SmallVec < I :: Item , N > > () . into_iter () ; if collected . len () > 0 { self . drain . move_tail (collected . len ()) ; let filled = self . drain . fill (& mut collected) ; debug_assert ! (filled) ; debug_assert_eq ! (collected . len () , 0) ; } } } }
    };
}

impl_32!()