// Generated macro for InstChar (struct)
macro_rules! Depcrate_progInstChar {
() => {
// Module: crate::prog
// Provides: {"InstChar"}
// Dependencies: {}
# [doc = " Representation of the Char instruction."] # [derive (Clone , Debug)] pub struct InstChar { # [doc = " The next location to execute in the program if this instruction"] # [doc = " succeeds."] pub goto : InstPtr , # [doc = " The character to test."] pub c : char , }
};
}
