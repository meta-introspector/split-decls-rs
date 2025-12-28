macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! HashSet {
    () => {
        deps!();
        # [doc = " Type alias for [`std::collections::HashSet<T, foldhash::fast::RandomState>`]."] pub type HashSet < T > = std :: collections :: HashSet < T , RandomState > ;
    };
}

HashSet!();