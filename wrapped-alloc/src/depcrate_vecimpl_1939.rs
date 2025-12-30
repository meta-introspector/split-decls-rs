// Generated macro for impl_1939 (impl)
macro_rules! Depcrate_vecimpl_1939 {
() => {
// Module: crate::vec
// Provides: {"impl_1939"}
// Dependencies: {}
impl < T : Clone , A : Allocator > Vec < T , A > { # [cfg (not (no_global_oom_handling))] # [track_caller] # [doc = " Extend the vector by `n` clones of value."] fn extend_with (& mut self , n : usize , value : T) { self . reserve (n) ; unsafe { let mut ptr = self . as_mut_ptr () . add (self . len ()) ; let mut local_len = SetLenOnDrop :: new (& mut self . len) ; for _ in 1 .. n { ptr :: write (ptr , value . clone ()) ; ptr = ptr . add (1) ; local_len . increment_len (1) ; } if n > 0 { ptr :: write (ptr , value) ; local_len . increment_len (1) ; } } } }
};
}
