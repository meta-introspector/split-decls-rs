// Generated macro for EmptyLook (enum)
macro_rules! Depcrate_progEmptyLook {
() => {
// Module: crate::prog
// Provides: {"EmptyLook"}
// Dependencies: {}
# [doc = " The set of zero-width match instructions."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum EmptyLook { # [doc = " Start of line or input."] StartLine , # [doc = " End of line or input."] EndLine , # [doc = " Start of input."] StartText , # [doc = " End of input."] EndText , # [doc = " Word character on one side and non-word character on other."] WordBoundary , # [doc = " Word character on both sides or non-word character on both sides."] NotWordBoundary , # [doc = " ASCII word boundary."] WordBoundaryAscii , # [doc = " Not ASCII word boundary."] NotWordBoundaryAscii , }
};
}
