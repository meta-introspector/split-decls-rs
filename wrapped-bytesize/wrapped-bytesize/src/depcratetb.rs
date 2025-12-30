// Generated macro for tb (function)
macro_rules! Depcratetb {
() => {
// Module: crate
// Provides: {"tb"}
// Dependencies: {}
# [doc = " Converts a quantity of terabytes to bytes."] pub fn tb < V : Into < u64 > > (size : V) -> u64 { size . into () * TB }
};
}
