// Generated macro for MethodSelf (enum)
macro_rules! Depcrate_astMethodSelf {
() => {
// Module: crate::ast
// Provides: {"MethodSelf"}
// Dependencies: {}
# [doc = " The 3 types variations of `self`."] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Copy , Clone)] pub enum MethodSelf { # [doc = " `self`"] ByValue , # [doc = " `&mut self`"] RefMutable , # [doc = " `&self`"] RefShared , }
};
}
