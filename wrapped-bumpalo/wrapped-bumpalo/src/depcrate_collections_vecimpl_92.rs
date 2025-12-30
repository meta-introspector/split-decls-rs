// Generated macro for impl_92 (impl)
macro_rules! Depcrate_collections_vecimpl_92 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'bump , T : 'bump > Vec < 'bump , T > { # [doc = " Extend the vector by `n` values, using the given generator."] fn extend_with < E : ExtendWith < T > > (& mut self , n : usize , mut value : E) { self . reserve (n) ; unsafe { let mut ptr = self . as_mut_ptr () . add (self . len ()) ; let mut local_len = SetLenOnDrop :: new (& mut self . len) ; for _ in 1 .. n { ptr :: write (ptr , value . next ()) ; ptr = ptr . offset (1) ; local_len . increment_len (1) ; } if n > 0 { ptr :: write (ptr , value . last ()) ; local_len . increment_len (1) ; } } } }
};
}
