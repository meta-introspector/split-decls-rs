// Generated macro for proptest_reserve_and_write_bytes_allocated_properly (function)
macro_rules! Depcrate_testsproptest_reserve_and_write_bytes_allocated_properly {
() => {
// Module: crate::tests
// Provides: {"proptest_reserve_and_write_bytes_allocated_properly"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_reserve_and_write_bytes_allocated_properly (# [strategy (rand_unicode ())] word : String) { let mut compact = CompactString :: default () ; prop_assert ! (compact . is_empty ()) ; compact . reserve (word . len ()) ; let slice = unsafe { compact . as_mut_bytes () } ; slice [.. word . len ()] . copy_from_slice (word . as_bytes ()) ; unsafe { compact . set_len (word . len ()) } prop_assert_eq ! (compact . len () , word . len ()) ; prop_assert_eq ! (compact . is_heap_allocated () , word . len () > MAX_SIZE) ; }
};
}
