macro_rules! deps {
    () => {
        ParallelIterator!();
        IntoParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! delegate_iterator {
    () => {
        deps!();
        # [doc = " Creates a parallel iterator implementation which simply wraps an inner type"] # [doc = " and delegates all methods inward.  The actual struct must already be"] # [doc = " declared with an `inner` field."] # [doc = ""] # [doc = " The implementation of `IntoParallelIterator` should be added separately."] macro_rules ! delegate_iterator { ($ iter : ty => $ item : ty , impl $ ($ args : tt) *) => { impl $ ($ args) * ParallelIterator for $ iter { type Item = $ item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > { self . inner . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { self . inner . opt_len () } } } }
    };
}

delegate_iterator!();