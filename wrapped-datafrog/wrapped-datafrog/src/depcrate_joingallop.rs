// Generated macro for gallop (function)
macro_rules! Depcrate_joingallop {
() => {
// Module: crate::join
// Provides: {"gallop"}
// Dependencies: {}
pub (crate) fn gallop < T > (mut slice : & [T] , mut cmp : impl FnMut (& T) -> bool) -> & [T] { if ! slice . is_empty () && cmp (& slice [0]) { let mut step = 1 ; while step < slice . len () && cmp (& slice [step]) { slice = & slice [step ..] ; step <<= 1 ; } step >>= 1 ; while step > 0 { if step < slice . len () && cmp (& slice [step]) { slice = & slice [step ..] ; } step >>= 1 ; } slice = & slice [1 ..] ; } slice }
};
}
