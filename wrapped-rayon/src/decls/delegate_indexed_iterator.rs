macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
    };
}

macro_rules! delegate_indexed_iterator {
    () => {
        deps!();
        # [doc = " Creates an indexed parallel iterator implementation which simply wraps an"] # [doc = " inner type and delegates all methods inward.  The actual struct must already"] # [doc = " be declared with an `inner` field."] macro_rules ! delegate_indexed_iterator { ($ iter : ty => $ item : ty , impl $ ($ args : tt) *) => { delegate_iterator ! { $ iter => $ item , impl $ ($ args) * } impl $ ($ args) * IndexedParallelIterator for $ iter { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > { self . inner . drive (consumer) } fn len (& self) -> usize { self . inner . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > { self . inner . with_producer (callback) } } } }
    };
}

delegate_indexed_iterator!()