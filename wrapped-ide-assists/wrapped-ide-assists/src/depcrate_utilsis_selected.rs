// Generated macro for is_selected (function)
macro_rules! Depcrate_utilsis_selected {
() => {
// Module: crate::utils
// Provides: {"is_selected"}
// Dependencies: {}
pub (crate) fn is_selected (it : & impl AstNode , selection : syntax :: TextRange , allow_empty : bool ,) -> bool { selection . intersect (it . syntax () . text_range ()) . is_some_and (| it | ! it . is_empty ()) || allow_empty && it . syntax () . text_range () . contains_range (selection) }
};
}
