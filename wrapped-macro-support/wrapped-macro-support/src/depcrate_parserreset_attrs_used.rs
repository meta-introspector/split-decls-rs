// Generated macro for reset_attrs_used (function)
macro_rules! Depcrate_parserreset_attrs_used {
() => {
// Module: crate::parser
// Provides: {"reset_attrs_used"}
// Dependencies: {}
pub fn reset_attrs_used () { ATTRS . with (| state | { state . parsed . set (0) ; state . checks . set (0) ; state . unused_attrs . borrow_mut () . clear () ; }) }
};
}
