macro_rules! PROGRESS_INDICATOR_START {
    () => {
        # [doc = " After this many interpreted terminators, we start emitting progress indicators at every"] # [doc = " power of two of interpreted terminators."] const PROGRESS_INDICATOR_START : usize = 4_000_000 ;
    };
}

PROGRESS_INDICATOR_START!()