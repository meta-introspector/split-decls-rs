// Generated macro for TimeBuf (struct)
macro_rules! Depcrate_parseTimeBuf {
() => {
// Module: crate::parse
// Provides: {"TimeBuf"}
// Dependencies: {}
# [doc = " A container for just enough bytes to hold the largest-possible [`time`](Time) instance."] # [doc = " It's used in conjunction with"] # [derive (Default , Clone)] pub struct TimeBuf { buf : SmallVec < u8 , { Time :: MAX . size () } > , }
};
}
