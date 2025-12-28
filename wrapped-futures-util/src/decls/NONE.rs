macro_rules! NONE {
    () => {
        # [doc = " There is nothing to poll and stream isn't being polled/waking/woken at the moment."] const NONE : u8 = 0 ;
    };
}

NONE!()