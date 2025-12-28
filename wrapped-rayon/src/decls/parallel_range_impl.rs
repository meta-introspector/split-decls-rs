macro_rules! deps {
    () => {
        Iter!();
        UnindexedConsumer!();
    };
}

macro_rules! parallel_range_impl {
    () => {
        deps!();
        macro_rules ! parallel_range_impl { ($ t : ty) => { impl RangeInteger for $ t { private_impl ! { } fn drive_unindexed < C > (iter : Iter <$ t >, consumer : C) -> C :: Result where C : UnindexedConsumer <$ t >, { convert ! (iter . drive_unindexed (consumer)) } fn opt_len (iter : & Iter <$ t >) -> Option < usize > { convert ! (iter . opt_len ()) } } } ; }
    };
}

parallel_range_impl!()