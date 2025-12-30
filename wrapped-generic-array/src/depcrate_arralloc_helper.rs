// Generated macro for alloc_helper (module)
macro_rules! Depcrate_arralloc_helper {
() => {
// Module: crate::arr
// Provides: {"alloc_helper"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_helper { use crate :: { ArrayLength , GenericArray , IntoArrayLength } ; impl < T , N : ArrayLength > GenericArray < T , N > { # [doc (hidden)] # [inline (always)] pub fn __from_vec_helper < const U : usize > (_empty : [() ; U] , vec : alloc :: vec :: Vec < T > ,) -> alloc :: boxed :: Box < GenericArray < T , N > > where typenum :: Const < U > : IntoArrayLength < ArrayLength = N > , { unsafe { GenericArray :: try_from_vec (vec) . unwrap_unchecked () } } } }
};
}
