// Generated macro for test_reserve_shrink_roundtrip_static (function)
macro_rules! Depcrate_teststest_reserve_shrink_roundtrip_static {
() => {
// Module: crate::tests
// Provides: {"test_reserve_shrink_roundtrip_static"}
// Dependencies: {}
# [test] fn test_reserve_shrink_roundtrip_static () { const TEXT : & str = "Hello, world! How are you today?" ; let mut s = CompactString :: const_new (TEXT) ; assert ! (! s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . reserve (128) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 128 + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to (64) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 64) ; assert_eq ! (s , TEXT) ; s . shrink_to_fit () ; assert ! (s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , s . len ()) ; assert_eq ! (s , TEXT) ; s . reserve (SIXTEEN_MB) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= SIXTEEN_MB + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to (64) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= 64) ; assert_eq ! (s , TEXT) ; s . reserve (SIXTEEN_MB) ; assert ! (s . is_heap_allocated ()) ; assert ! (s . capacity () >= SIXTEEN_MB + TEXT . len ()) ; assert_eq ! (s , TEXT) ; s . shrink_to_fit () ; assert ! (s . is_heap_allocated ()) ; assert_eq ! (s . capacity () , s . len ()) ; assert_eq ! (s , TEXT) ; }
};
}
