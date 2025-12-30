// Generated macro for Instruction (enum)
macro_rules! Depcrate_typesInstruction {
() => {
// Module: crate::types
// Provides: {"Instruction"}
// Dependencies: {}
# [doc = " Tells what to do and is derived from a [`RefSpec`][crate::RefSpecRef]."] # [derive (PartialOrd , Ord , PartialEq , Eq , Copy , Clone , Hash , Debug)] pub enum Instruction < 'a > { # [doc = " An instruction for pushing."] Push (instruction :: Push < 'a >) , # [doc = " An instruction for fetching."] Fetch (instruction :: Fetch < 'a >) , }
};
}
