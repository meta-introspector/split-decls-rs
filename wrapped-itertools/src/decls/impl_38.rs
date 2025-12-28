macro_rules! deps {
    () => {
        DedupPredicate!();
        CoalescePredicate!();
        DedupPredWithCount2CoalescePred!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < DP , T > CoalescePredicate < T , (usize , T) > for DedupPredWithCount2CoalescePred < DP > where DP : DedupPredicate < T > , { fn coalesce_pair (& mut self , (c , t) : (usize , T) , item : T ,) -> Result < (usize , T) , ((usize , T) , (usize , T)) > { if self . 0 . dedup_pair (& t , & item) { Ok ((c + 1 , t)) } else { Err (((c , t) , (1 , item))) } } }
    };
}

impl_38!()