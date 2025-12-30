// Generated macro for InstBytes (struct)
macro_rules! Depcrate_progInstBytes {
() => {
// Module: crate::prog
// Provides: {"InstBytes"}
// Dependencies: {}
# [doc = " Representation of the Bytes instruction."] # [derive (Clone , Debug)] pub struct InstBytes { # [doc = " The next location to execute in the program if this instruction"] # [doc = " succeeds."] pub goto : InstPtr , # [doc = " The start (inclusive) of this byte range."] pub start : u8 , # [doc = " The end (inclusive) of this byte range."] pub end : u8 , }
};
}
