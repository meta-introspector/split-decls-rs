// Generated macro for iter_cmp (function)
macro_rules! Depcrate_orditer_cmp {
() => {
// Module: crate::ord
// Provides: {"iter_cmp"}
// Dependencies: {}
# [doc = " Compare the order of two iterators using [`DerCmp`] on the values."] pub (crate) fn iter_cmp < 'a , I , T > (a : I , b : I) -> Result < Ordering > where I : Iterator < Item = & 'a T > + ExactSizeIterator , T : 'a + DerOrd , { let length_ord = a . len () . cmp (& b . len ()) ; for (value1 , value2) in a . zip (b) { match value1 . der_cmp (value2) ? { Ordering :: Equal => () , other => return Ok (other) , } } Ok (length_ord) }
};
}
