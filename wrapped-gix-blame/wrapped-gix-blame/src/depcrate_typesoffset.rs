// Generated macro for Offset (enum)
macro_rules! Depcrate_typesOffset {
() => {
// Module: crate::types
// Provides: {"Offset"}
// Dependencies: {}
# [doc = " Describes the offset of a particular hunk relative to the *Blamed File*."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Offset { # [doc = " The amount of lines to add."] Added (u32) , # [doc = " The amount of lines to remove."] Deleted (u32) , }
};
}
