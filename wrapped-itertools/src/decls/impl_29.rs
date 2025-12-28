macro_rules! deps {
    () => {
        CoalescePredicate!();
        DedupPredicate!();
        DedupPred2CoalescePred!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < DP , T > CoalescePredicate < T , T > for DedupPred2CoalescePred < DP > where DP : DedupPredicate < T > , { fn coalesce_pair (& mut self , t : T , item : T) -> Result < T , (T , T) > { if self . 0 . dedup_pair (& t , & item) { Ok (t) } else { Err ((t , item)) } } }
    };
}

impl_29!();