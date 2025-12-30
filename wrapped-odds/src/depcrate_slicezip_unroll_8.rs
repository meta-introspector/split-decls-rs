// Generated macro for zip_unroll_8 (function)
macro_rules! Depcrate_slicezip_unroll_8 {
() => {
// Module: crate::slice
// Provides: {"zip_unroll_8"}
// Dependencies: {}
# [cfg (test)] pub fn zip_unroll_8 < 'a , 'b , A , B , F > (a : & 'a [A] , b : & 'b [B] , mut f : F) where F : FnMut (usize , & 'a A , & 'b B) , { let len = min (a . len () , b . len ()) ; let mut a = & a [.. len] ; let mut b = & b [.. len] ; while a . len () >= 8 { f (0 , & a [0] , & b [0]) ; f (1 , & a [1] , & b [1]) ; f (2 , & a [2] , & b [2]) ; f (3 , & a [3] , & b [3]) ; f (4 , & a [4] , & b [4]) ; f (5 , & a [5] , & b [5]) ; f (6 , & a [6] , & b [6]) ; f (7 , & a [7] , & b [7]) ; a = & a [8 ..] ; b = & b [8 ..] ; } for i in 0 .. 7 { if i < a . len () { f (0 , & a [i] , & b [i]) ; } } }
};
}
