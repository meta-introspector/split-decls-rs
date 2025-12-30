// Generated macro for IOUSBBitRange (macro)
macro_rules! DepcrateIOUSBBitRange {
() => {
// Module: crate
// Provides: {"IOUSBBitRange"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! IOUSBBitRange { ($ start : expr , $ end : expr) => { ! ((1 << $ start) - 1) & ((1 << $ end) | ((1 << $ end) - 1)) } ; }
};
}
