// Generated macro for macro_67 (macro)
macro_rules! Depcrate_arcmacro_67 {
() => {
// Module: crate::arc
// Provides: {"macro_67"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_min_const_generics))] items ! { impl < T , const N : usize > From < [T ; N] > for Arc < [T] > { # [doc = " Converts a [`[T; N]`](prim@array) into an `Arc<[T]>`."] # [doc = ""] # [doc = " The conversion moves the array into a newly allocated `Arc`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let original: [i32; 3] = [1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : [T ; N]) -> Self { let v : Box < [T] > = Box ::< [T ; N] >:: from (v) ; v . into () } } }
};
}
