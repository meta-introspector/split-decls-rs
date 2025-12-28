macro_rules! deps {
    () => {
        DedupByWithCount!();
        DedupPredWithCount2CoalescePred!();
    };
}

macro_rules! dedup_by_with_count {
    () => {
        deps!();
        # [doc = " Create a new `DedupByWithCount`."] pub fn dedup_by_with_count < I , Pred > (iter : I , dedup_pred : Pred) -> DedupByWithCount < I , Pred > where I : Iterator , { DedupByWithCount { last : None , iter , f : DedupPredWithCount2CoalescePred (dedup_pred) , } }
    };
}

dedup_by_with_count!();