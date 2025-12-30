// Generated macro for IOUSBBitRange (macro)
macro_rules! Depcrate_macrosIOUSBBitRange {
() => {
// Module: crate::macros
// Provides: {"IOUSBBitRange"}
// Dependencies: {}
macro_rules ! IOUSBBitRange { ($ start : expr , $ end : expr) => { ! ((1 << $ start) - 1) & ((1 << $ end) | ((1 << $ end) - 1)) } ; }
};
}
