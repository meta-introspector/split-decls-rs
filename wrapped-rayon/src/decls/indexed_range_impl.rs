macro_rules! deps {
    () => {
        ProducerCallback!();
        Consumer!();
        Iter!();
    };
}

macro_rules! indexed_range_impl {
    () => {
        deps!();
        macro_rules ! indexed_range_impl { ($ t : ty) => { parallel_range_impl ! { $ t } impl IndexedRangeInteger for $ t { private_impl ! { } fn drive < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : Consumer <$ t >, { convert ! (iter . drive (consumer)) } fn len (iter : & Iter <$ t >) -> usize { iter . range . len () } fn with_producer < CB > (iter : Iter <$ t >, callback : CB) -> CB :: Output where CB : ProducerCallback <$ t >, { convert ! (iter . with_producer (callback)) } } } ; }
    };
}

indexed_range_impl!()