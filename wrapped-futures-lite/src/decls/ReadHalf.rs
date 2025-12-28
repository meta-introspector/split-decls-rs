macro_rules! ReadHalf {
    () => {
        # [doc = " The read half returned by [`split()`]."] # [derive (Debug)] pub struct ReadHalf < T > (Arc < Mutex < T > >) ;
    };
}

ReadHalf!()