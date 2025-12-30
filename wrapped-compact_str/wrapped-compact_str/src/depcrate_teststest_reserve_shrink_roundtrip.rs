// Generated macro for test_reserve_shrink_roundtrip (function)
macro_rules! Depcrate_teststest_reserve_shrink_roundtrip {
() => {
// Module: crate::tests
// Provides: {"test_reserve_shrink_roundtrip"}
// Dependencies: {}
# [test] fn test_reserve_shrink_roundtrip () { const TEXT : & str = "Hello." ; let mut s = CompactString :: new (TEXT) ; assert ! (! s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , MAX_SIZE) ; assert_eq ! (s , TEXT) ; s . reserve (128) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 128 + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to (64) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 64) ; assert_eq ! (s , TEXT) ; s . shrink_to_fit () ; assert ! (! s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , MAX_SIZE) ; assert_eq ! (s , TEXT) ; s . reserve (SIXTEEN_MB) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= SIXTEEN_MB + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to (64) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 64) ; assert_eq ! (s , TEXT) ; s . reserve (SIXTEEN_MB) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= SIXTEEN_MB + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to_fit () ; assert ! (! s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , MAX_SIZE) ; assert_eq ! (s , TEXT) ; }
};
}
