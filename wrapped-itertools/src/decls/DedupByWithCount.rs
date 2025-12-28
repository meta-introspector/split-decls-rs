macro_rules! deps {
    () => {
        CoalesceBy!();
        DedupPredWithCount2CoalescePred!();
        WithCount!();
    };
}

macro_rules! DedupByWithCount {
    () => {
        deps!();
        # [doc = " An iterator adaptor that removes repeated duplicates, while keeping a count of how many"] # [doc = " repeated elements were present. This will determine equality using a comparison function."] # [doc = ""] # [doc = " See [`.dedup_by_with_count()`](crate::Itertools::dedup_by_with_count) or"] # [doc = " [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information."] pub type DedupByWithCount < I , Pred > = CoalesceBy < I , DedupPredWithCount2CoalescePred < Pred > , WithCount > ;
    };
}

DedupByWithCount!();