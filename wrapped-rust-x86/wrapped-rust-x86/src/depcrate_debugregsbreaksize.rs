// Generated macro for BreakSize (enum)
macro_rules! Depcrate_debugregsBreakSize {
() => {
// Module: crate::debugregs
// Provides: {"BreakSize"}
// Dependencies: {}
# [doc = " Specify the size of the memory location at the address specified in the"] # [doc = " corresponding breakpoint address register (DR0 through DR3)."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum BreakSize { # [doc = " 00 — 1-byte length."] Bytes1 = 0b00 , # [doc = " 01 — 2-byte length."] Bytes2 = 0b01 , # [doc = " 10 — 8 byte length (or undefined, on older processors)."] Bytes8 = 0b10 , # [doc = " 11 — 4-byte length."] Bytes4 = 0b11 , }
};
}
