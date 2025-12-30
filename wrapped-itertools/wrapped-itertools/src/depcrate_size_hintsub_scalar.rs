// Generated macro for sub_scalar (function)
macro_rules! Depcrate_size_hintsub_scalar {
() => {
// Module: crate::size_hint
// Provides: {"sub_scalar"}
// Dependencies: {}
# [doc = " Subtract `x` correctly from a `SizeHint`."] # [inline] pub fn sub_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_sub (x) ; hi = hi . map (| elt | elt . saturating_sub (x)) ; (low , hi) }
};
}
