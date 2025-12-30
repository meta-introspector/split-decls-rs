// Generated macro for LocalClocks (struct)
macro_rules! Depcrate_concurrency_data_raceLocalClocks {
() => {
// Module: crate::concurrency::data_race
// Provides: {"LocalClocks"}
// Dependencies: {}
# [doc = " Stripped-down version of [`MemoryCellClocks`] for the clocks we need to keep track"] # [doc = " of in a local that does not yet have addressable memory -- and hence can only"] # [doc = " be accessed from the thread its stack frame belongs to, and cannot be access atomically."] # [derive (Debug)] struct LocalClocks { write : VTimestamp , write_type : NaWriteType , read : VTimestamp , }
};
}
