// Generated macro for Access (enum)
macro_rules! Depcrate_leavesAccess {
() => {
// Module: crate::leaves
// Provides: {"Access"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , Ord , PartialOrd , Debug)] # [doc = " The possible access characteristics a Property can have."] pub enum Access { # [doc = " The Property can only be read (Get)."] Read , # [doc = " The Property can be read or written."] ReadWrite , # [doc = " The Property can only be written (Set)."] Write , }
};
}
