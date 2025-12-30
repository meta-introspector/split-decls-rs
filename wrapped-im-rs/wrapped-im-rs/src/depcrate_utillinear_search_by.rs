// Generated macro for linear_search_by (function)
macro_rules! Depcrate_utillinear_search_by {
() => {
// Module: crate::util
// Provides: {"linear_search_by"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn linear_search_by < 'a , A , I , F > (iterable : I , mut cmp : F) -> Result < usize , usize > where A : 'a , I : IntoIterator < Item = & 'a A > , F : FnMut (& A) -> Ordering , { let mut pos = 0 ; for value in iterable { match cmp (value) { Ordering :: Equal => return Ok (pos) , Ordering :: Greater => return Err (pos) , Ordering :: Less => { } } pos += 1 ; } Err (pos) }
};
}
