// Generated macro for impl_63 (impl)
macro_rules! Depcrate_paramsimpl_63 {
() => {
// Module: crate::params
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Pair < 'a > ; fn next (& mut self) -> Option < Pair < 'a > > { let mut param = self . inner . as_mut () ? . next () ? . split (PAIR_DELIMITER) ; let name = param . next () . and_then (| id | Ident :: try_from (id) . ok ()) . expect (INVARIANT_VIOLATED_MSG) ; let value = param . next () . and_then (| value | Value :: try_from (value) . ok ()) . expect (INVARIANT_VIOLATED_MSG) ; debug_assert_eq ! (param . next () , None) ; Some ((name , value)) } }
};
}
