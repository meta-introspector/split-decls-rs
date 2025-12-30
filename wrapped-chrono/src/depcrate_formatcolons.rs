// Generated macro for Colons (enum)
macro_rules! Depcrate_formatColons {
() => {
// Module: crate::format
// Provides: {"Colons"}
// Dependencies: {}
# [doc = " The separator between hours and minutes in an offset."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum Colons { # [doc = " No separator"] None , # [doc = " Colon (`:`) as separator"] Colon , # [doc = " No separator when formatting, colon allowed when parsing."] Maybe , }
};
}
