// Generated macro for zip_unroll_4 (function)
macro_rules! Depcrate_slicezip_unroll_4 {
() => {
// Module: crate::slice
// Provides: {"zip_unroll_4"}
// Dependencies: {}
# [cfg (test)] pub fn zip_unroll_4 < 'a , 'b , A , B , F > (a : & 'a [A] , b : & 'b [B] , mut f : F) where F : FnMut (usize , & 'a A , & 'b B) , { let len = min (a . len () , b . len ()) ; let mut a = & a [.. len] ; let mut b = & b [.. len] ; while a . len () >= 4 { f (0 , & a [0] , & b [0]) ; f (1 , & a [1] , & b [1]) ; f (2 , & a [2] , & b [2]) ; f (3 , & a [3] , & b [3]) ; a = & a [4 ..] ; b = & b [4 ..] ; } for i in 0 .. 3 { if i < a . len () { f (0 , & a [i] , & b [i]) ; } } }
};
}
