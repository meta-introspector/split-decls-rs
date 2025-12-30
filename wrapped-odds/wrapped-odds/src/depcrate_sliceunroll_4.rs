// Generated macro for unroll_4 (function)
macro_rules! Depcrate_sliceunroll_4 {
() => {
// Module: crate::slice
// Provides: {"unroll_4"}
// Dependencies: {}
# [cfg (test)] pub fn unroll_4 < 'a , T , F > (data : & 'a [T] , mut f : F) where F : FnMut (& 'a T) , { let mut data = data ; while data . len () >= 4 { f (& data [0]) ; f (& data [1]) ; f (& data [2]) ; f (& data [3]) ; data = & data [4 ..] ; } for i in 0 .. 3 { if i < data . len () { f (& data [i]) ; } } }
};
}
