// Generated macro for impl_244 (impl)
macro_rules! Depcrate_vecimpl_244 {
() => {
// Module: crate::vec
// Provides: {"impl_244"}
// Dependencies: {}
impl < T , A : Allocator > Vec < T , A > { # [cfg (not (no_global_oom_handling))] # [inline (always)] # [doc = " Extend the vector by `n` values, using the given generator."] fn extend_with < E : ExtendWith < T > > (& mut self , n : usize , mut value : E) { self . reserve (n) ; unsafe { let mut ptr = self . as_mut_ptr () . add (self . len ()) ; let mut local_len = SetLenOnDrop :: new (& mut self . len) ; for _ in 1 .. n { ptr :: write (ptr , value . next ()) ; ptr = ptr . add (1) ; local_len . increment_len (1) ; } if n > 0 { ptr :: write (ptr , value . last ()) ; local_len . increment_len (1) ; } } } }
};
}
