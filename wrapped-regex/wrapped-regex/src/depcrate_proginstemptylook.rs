// Generated macro for InstEmptyLook (struct)
macro_rules! Depcrate_progInstEmptyLook {
() => {
// Module: crate::prog
// Provides: {"InstEmptyLook"}
// Dependencies: {}
# [doc = " Representation of the EmptyLook instruction."] # [derive (Clone , Debug)] pub struct InstEmptyLook { # [doc = " The next location to execute in the program if this instruction"] # [doc = " succeeds."] pub goto : InstPtr , # [doc = " The type of zero-width assertion to check."] pub look : EmptyLook , }
};
}
