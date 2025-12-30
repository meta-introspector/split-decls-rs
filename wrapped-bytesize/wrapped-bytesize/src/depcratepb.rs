// Generated macro for pb (function)
macro_rules! Depcratepb {
() => {
// Module: crate
// Provides: {"pb"}
// Dependencies: {}
# [doc = " Converts a quantity of petabytes to bytes."] pub fn pb < V : Into < u64 > > (size : V) -> u64 { size . into () * PB }
};
}
