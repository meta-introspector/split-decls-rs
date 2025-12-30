// Generated macro for unroll_8 (function)
macro_rules! Depcrate_sliceunroll_8 {
() => {
// Module: crate::slice
// Provides: {"unroll_8"}
// Dependencies: {}
# [cfg (test)] pub fn unroll_8 < 'a , T , F > (data : & 'a [T] , mut f : F) where F : FnMut (& 'a T) , { let mut data = data ; while data . len () >= 8 { f (& data [0]) ; f (& data [1]) ; f (& data [2]) ; f (& data [3]) ; f (& data [4]) ; f (& data [5]) ; f (& data [6]) ; f (& data [7]) ; data = & data [8 ..] ; } for i in 0 .. 7 { if i < data . len () { f (& data [i]) ; } } }
};
}
