// Generated macro for impl_205 (impl)
macro_rules! Depcrate_parser_stateimpl_205 {
() => {
// Module: crate::parser_state
// Provides: {"impl_205"}
// Dependencies: {}
# [doc = " A holder for a literal string, for use in `push_literal`. This is typically a `&'static str`, but is an owned"] # [doc = " `Rc<String>` for the pest vm."] impl < 'i > BorrowedOrArc < 'i > { fn as_str < 'a : 'i > (& 'a self) -> & 'a str { match self { BorrowedOrArc :: Borrowed (s) => s , BorrowedOrArc :: Owned (s) => s . deref () , } } }
};
}
