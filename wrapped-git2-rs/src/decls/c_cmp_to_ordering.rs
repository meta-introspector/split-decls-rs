macro_rules! c_cmp_to_ordering {
    () => {
        pub fn c_cmp_to_ordering (cmp : c_int) -> Ordering { match cmp { 0 => Ordering :: Equal , n if n < 0 => Ordering :: Less , _ => Ordering :: Greater , } }
    };
}

c_cmp_to_ordering!()