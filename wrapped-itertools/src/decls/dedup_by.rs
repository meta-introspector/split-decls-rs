macro_rules! deps {
    () => {
        DedupPred2CoalescePred!();
        DedupBy!();
    };
}

macro_rules! dedup_by {
    () => {
        deps!();
        # [doc = " Create a new `DedupBy`."] pub fn dedup_by < I , Pred > (iter : I , dedup_pred : Pred) -> DedupBy < I , Pred > where I : Iterator , { DedupBy { last : None , iter , f : DedupPred2CoalescePred (dedup_pred) , } }
    };
}

dedup_by!();