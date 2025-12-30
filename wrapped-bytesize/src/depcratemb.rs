// Generated macro for mb (function)
macro_rules! Depcratemb {
() => {
// Module: crate
// Provides: {"mb"}
// Dependencies: {}
# [doc = " Converts a quantity of megabytes to bytes."] pub fn mb < V : Into < u64 > > (size : V) -> u64 { size . into () * MB }
};
}
