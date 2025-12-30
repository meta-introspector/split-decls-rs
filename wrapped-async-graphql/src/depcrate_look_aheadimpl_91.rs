// Generated macro for impl_91 (impl)
macro_rules! Depcrate_look_aheadimpl_91 {
() => {
// Module: crate::look_ahead
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'a > From < SelectionField < 'a > > for Lookahead < 'a > { fn from (selection_field : SelectionField < 'a >) -> Self { Lookahead { fragments : selection_field . fragments , fields : vec ! [selection_field . field] , context : selection_field . context , } } }
};
}
