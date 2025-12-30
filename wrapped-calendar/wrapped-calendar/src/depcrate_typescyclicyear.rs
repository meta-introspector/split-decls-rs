// Generated macro for CyclicYear (struct)
macro_rules! Depcrate_typesCyclicYear {
() => {
// Module: crate::types
// Provides: {"CyclicYear"}
// Dependencies: {}
# [doc = " Year information for a year that is specified as a cyclic year"] # [derive (Copy , Clone , Debug , PartialEq)] # [non_exhaustive] pub struct CyclicYear { # [doc = " The year in the cycle, 1-based"] pub year : u8 , # [doc = " The ISO year corresponding to this year"] pub related_iso : i32 , }
};
}
