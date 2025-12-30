// Generated macro for eb (function)
macro_rules! Depcrateeb {
() => {
// Module: crate
// Provides: {"eb"}
// Dependencies: {}
# [doc = " Converts a quantity of exabytes to bytes."] pub fn eb < V : Into < u64 > > (size : V) -> u64 { size . into () * EB }
};
}
