macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! TimeBuf {
    () => {
        deps!();
        # [doc = " A container for just enough bytes to hold the largest-possible [`time`](Time) instance."] # [doc = " It's used in conjunction with"] # [derive (Default , Clone)] pub struct TimeBuf { buf : SmallVec < u8 , { Time :: MAX . size () } > , }
    };
}

TimeBuf!();