// Generated macro for slice (function)
macro_rules! Depcrateslice {
() => {
// Module: crate
// Provides: {"slice"}
// Dependencies: {}
# [doc = " Computes the diff between two slices."] pub fn slice < 'a , T : PartialEq > (left : & 'a [T] , right : & 'a [T]) -> Vec < Result < & 'a T > > { do_diff (left , right , | t | t) }
};
}
