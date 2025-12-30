// Generated macro for zst_sanity (function)
macro_rules! Depcrate_raw_vec_testszst_sanity {
() => {
// Module: crate::raw_vec::tests
// Provides: {"zst_sanity"}
// Dependencies: {}
fn zst_sanity < T > (v : & RawVec < T >) { assert_eq ! (v . capacity () , usize :: MAX) ; assert_eq ! (v . ptr () , core :: ptr :: Unique ::< T >:: dangling () . as_ptr ()) ; assert_eq ! (v . inner . current_memory (T :: LAYOUT) , None) ; }
};
}
