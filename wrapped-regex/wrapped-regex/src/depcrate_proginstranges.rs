// Generated macro for InstRanges (struct)
macro_rules! Depcrate_progInstRanges {
() => {
// Module: crate::prog
// Provides: {"InstRanges"}
// Dependencies: {}
# [doc = " Representation of the Ranges instruction."] # [derive (Clone , Debug)] pub struct InstRanges { # [doc = " The next location to execute in the program if this instruction"] # [doc = " succeeds."] pub goto : InstPtr , # [doc = " The set of Unicode scalar value ranges to test."] pub ranges : Vec < (char , char) > , }
};
}
