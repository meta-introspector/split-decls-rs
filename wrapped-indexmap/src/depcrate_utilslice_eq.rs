// Generated macro for slice_eq (function)
macro_rules! Depcrate_utilslice_eq {
() => {
// Module: crate::util
// Provides: {"slice_eq"}
// Dependencies: {}
pub (crate) fn slice_eq < T , U > (left : & [T] , right : & [U] , eq : impl Fn (& T , & U) -> bool) -> bool { if left . len () != right . len () { return false ; } for i in 0 .. left . len () { if ! eq (& left [i] , & right [i]) { return false ; } } true }
};
}
