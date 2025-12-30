// Generated macro for CounterWriter (struct)
macro_rules! Depcrate_counterCounterWriter {
() => {
// Module: crate::counter
// Provides: {"CounterWriter"}
// Dependencies: {}
# [doc = " A writer that counts the number of bytes that have been successfully"] # [doc = " written."] # [derive (Clone , Debug)] pub (crate) struct CounterWriter < W > { wtr : W , count : u64 , total_count : u64 , }
};
}
