// Generated macro for impl_1949 (impl)
macro_rules! Depcrate_vecimpl_1949 {
() => {
// Module: crate::vec
// Provides: {"impl_1949"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for Vec < T , A > { # [track_caller] fn clone (& self) -> Self { let alloc = self . allocator () . clone () ; < [T] > :: to_vec_in (& * * self , alloc) } # [doc = " Overwrites the contents of `self` with a clone of the contents of `source`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible. Additionally, if the element type"] # [doc = " `T` overrides `clone_from()`, this will reuse the resources of `self`'s"] # [doc = " elements as well."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = vec![5, 6, 7];"] # [doc = " let mut y = vec![8, 9, 10];"] # [doc = " let yp: *const i32 = y.as_ptr();"] # [doc = ""] # [doc = " y.clone_from(&x);"] # [doc = ""] # [doc = " // The value is the same"] # [doc = " assert_eq!(x, y);"] # [doc = ""] # [doc = " // And no reallocation occurred"] # [doc = " assert_eq!(yp, y.as_ptr());"] # [doc = " ```"] # [track_caller] fn clone_from (& mut self , source : & Self) { crate :: slice :: SpecCloneIntoVec :: clone_into (source . as_slice () , self) ; } }
};
}
