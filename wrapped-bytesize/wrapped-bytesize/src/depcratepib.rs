// Generated macro for pib (function)
macro_rules! Depcratepib {
() => {
// Module: crate
// Provides: {"pib"}
// Dependencies: {}
# [doc = " Converts a quantity of pebibytes to bytes."] pub fn pib < V : Into < u64 > > (size : V) -> u64 { size . into () * PIB }
};
}
