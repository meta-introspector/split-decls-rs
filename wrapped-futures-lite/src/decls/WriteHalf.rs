macro_rules! WriteHalf {
    () => {
        # [doc = " The write half returned by [`split()`]."] # [derive (Debug)] pub struct WriteHalf < T > (Arc < Mutex < T > >) ;
    };
}

WriteHalf!()