macro_rules! deps {
    () => {
        DedupEq!();
        DedupByWithCount!();
    };
}

macro_rules! DedupWithCount {
    () => {
        deps!();
        # [doc = " An iterator adaptor that removes repeated duplicates, while keeping a count of how many"] # [doc = " repeated elements were present."] # [doc = ""] # [doc = " See [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information."] pub type DedupWithCount < I > = DedupByWithCount < I , DedupEq > ;
    };
}

DedupWithCount!()