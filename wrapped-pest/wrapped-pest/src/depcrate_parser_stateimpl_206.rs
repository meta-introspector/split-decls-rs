// Generated macro for impl_206 (impl)
macro_rules! Depcrate_parser_stateimpl_206 {
() => {
// Module: crate::parser_state
// Provides: {"impl_206"}
// Dependencies: {}
impl From < Cow < 'static , str > > for BorrowedOrArc < '_ > { fn from (value : Cow < 'static , str >) -> Self { match value { Cow :: Borrowed (s) => Self :: Borrowed (s) , Cow :: Owned (s) => Self :: Owned (Arc :: new (s)) , } } }
};
}
