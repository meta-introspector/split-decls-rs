// Generated macro for compare (function)
macro_rules! Depcrate_bigintcompare {
() => {
// Module: crate::bigint
// Provides: {"compare"}
// Dependencies: {}
# [doc = " Compare `x` to `y`, in little-endian order."] # [inline] pub fn compare (x : & [Limb] , y : & [Limb]) -> cmp :: Ordering { match x . len () . cmp (& y . len ()) { cmp :: Ordering :: Equal => { let iter = x . iter () . rev () . zip (y . iter () . rev ()) ; for (& xi , yi) in iter { match xi . cmp (yi) { cmp :: Ordering :: Equal => () , ord => return ord , } } cmp :: Ordering :: Equal } , ord => ord , } }
};
}
