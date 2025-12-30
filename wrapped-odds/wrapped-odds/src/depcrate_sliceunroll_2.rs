// Generated macro for unroll_2 (function)
macro_rules! Depcrate_sliceunroll_2 {
() => {
// Module: crate::slice
// Provides: {"unroll_2"}
// Dependencies: {}
# [cfg (test)] pub fn unroll_2 < 'a , T , F > (data : & 'a [T] , mut f : F) where F : FnMut (& 'a T) , { let mut data = data ; while data . len () >= 2 { f (& data [0]) ; f (& data [1]) ; data = & data [2 ..] ; } if 0 < data . len () { f (& data [0]) ; } }
};
}
