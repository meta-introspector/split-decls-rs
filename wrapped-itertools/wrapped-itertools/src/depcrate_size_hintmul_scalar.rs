// Generated macro for mul_scalar (function)
macro_rules! Depcrate_size_hintmul_scalar {
() => {
// Module: crate::size_hint
// Provides: {"mul_scalar"}
// Dependencies: {}
# [doc = " Multiply `x` correctly with a `SizeHint`."] # [inline] pub fn mul_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_mul (x) ; hi = hi . and_then (| elt | elt . checked_mul (x)) ; (low , hi) }
};
}
