// Generated macro for mib (function)
macro_rules! Depcratemib {
() => {
// Module: crate
// Provides: {"mib"}
// Dependencies: {}
# [doc = " Converts a quantity of mebibytes to bytes."] pub fn mib < V : Into < u64 > > (size : V) -> u64 { size . into () * MIB }
};
}
