// Generated macro for AList (struct)
macro_rules! Depcrate_astAList {
() => {
// Module: crate::ast
// Provides: {"AList"}
// Dependencies: {}
# [doc = " A list of attributes. This corresponds to the `a_list` non-terminal of the"] # [doc = " grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Clone)] pub struct AList < A > { # [doc = " The attributes in the list."] pub elems : Vec < A > , }
};
}
