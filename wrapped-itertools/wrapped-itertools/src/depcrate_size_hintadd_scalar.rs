// Generated macro for add_scalar (function)
macro_rules! Depcrate_size_hintadd_scalar {
() => {
// Module: crate::size_hint
// Provides: {"add_scalar"}
// Dependencies: {}
# [doc = " Add `x` correctly to a `SizeHint`."] # [inline] pub fn add_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_add (x) ; hi = hi . and_then (| elt | elt . checked_add (x)) ; (low , hi) }
};
}
