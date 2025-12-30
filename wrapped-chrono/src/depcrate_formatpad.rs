// Generated macro for Pad (enum)
macro_rules! Depcrate_formatPad {
() => {
// Module: crate::format
// Provides: {"Pad"}
// Dependencies: {}
# [doc = " Padding characters for numeric items."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum Pad { # [doc = " No padding."] None , # [doc = " Zero (`0`) padding."] Zero , # [doc = " Space padding."] Space , }
};
}
