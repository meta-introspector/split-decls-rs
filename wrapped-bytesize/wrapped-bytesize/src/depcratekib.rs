// Generated macro for kib (function)
macro_rules! Depcratekib {
() => {
// Module: crate
// Provides: {"kib"}
// Dependencies: {}
# [doc = " Converts a quantity of kibibytes to bytes."] pub fn kib < V : Into < u64 > > (size : V) -> u64 { size . into () * KIB }
};
}
