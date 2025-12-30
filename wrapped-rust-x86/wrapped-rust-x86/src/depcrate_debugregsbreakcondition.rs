// Generated macro for BreakCondition (enum)
macro_rules! Depcrate_debugregsBreakCondition {
() => {
// Module: crate::debugregs
// Provides: {"BreakCondition"}
// Dependencies: {}
# [doc = " Specifies the  breakpoint condition for a corresponding breakpoint."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum BreakCondition { # [doc = " 00 — Break on instruction execution only."] Instructions = 0b00 , # [doc = " 01 — Break on data writes only."] DataWrites = 0b01 , # [doc = " 10 — Break on I/O reads or writes."] # [doc = ""] # [doc = " # Notes"] # [doc = " For this type to be available, the DE (debug extensions) flag in control"] # [doc = " register CR4 must be set."] IoReadsWrites = 0b10 , # [doc = " 11 — Break on data reads or writes but not instruction fetches."] DataReadsWrites = 0b11 , }
};
}
