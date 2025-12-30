// Generated macro for parse_bound (function)
macro_rules! Depcrate_common_rangeparse_bound {
() => {
// Module: crate::common::range
// Provides: {"parse_bound"}
// Dependencies: {}
fn parse_bound (s : & str) -> Option < Bound < u64 > > { if s . is_empty () { return Some (Bound :: Unbounded) ; } s . parse () . ok () . map (Bound :: Included) }
};
}
