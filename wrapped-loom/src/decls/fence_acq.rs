macro_rules! deps {
    () => {
        Execution!();
        State!();
    };
}

macro_rules! fence_acq {
    () => {
        deps!();
        fn fence_acq (execution : & mut Execution) { for state in execution . objects . iter_mut :: < State > () { for store in state . stores_mut () { if ! store . first_seen . is_seen_by_current (& execution . threads) { continue ; } store . sync . sync_load (& mut execution . threads , Ordering :: Acquire) ; } } }
    };
}

fence_acq!()