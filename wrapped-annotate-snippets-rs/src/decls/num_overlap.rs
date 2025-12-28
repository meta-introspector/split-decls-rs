macro_rules! num_overlap {
    () => {
        pub (crate) fn num_overlap (a_start : usize , a_end : usize , b_start : usize , b_end : usize , inclusive : bool ,) -> bool { let extra = usize :: from (inclusive) ; (b_start .. b_end + extra) . contains (& a_start) || (a_start .. a_end + extra) . contains (& b_start) }
    };
}

num_overlap!()