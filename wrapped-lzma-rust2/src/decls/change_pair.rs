macro_rules! change_pair {
    () => {
        fn change_pair (small_dist : u32 , big_dist : u32) -> bool { small_dist < (big_dist >> 7) }
    };
}

change_pair!();