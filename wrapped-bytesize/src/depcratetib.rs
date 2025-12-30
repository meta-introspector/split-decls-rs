// Generated macro for tib (function)
macro_rules! Depcratetib {
() => {
// Module: crate
// Provides: {"tib"}
// Dependencies: {}
# [doc = " Converts a quantity of tebibytes to bytes."] pub fn tib < V : Into < u64 > > (size : V) -> u64 { size . into () * TIB }
};
}
