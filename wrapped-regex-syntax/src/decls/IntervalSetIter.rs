macro_rules! IntervalSetIter {
    () => {
        # [doc = " An iterator over intervals."] # [derive (Debug)] pub struct IntervalSetIter < 'a , I > (slice :: Iter < 'a , I >) ;
    };
}

IntervalSetIter!()