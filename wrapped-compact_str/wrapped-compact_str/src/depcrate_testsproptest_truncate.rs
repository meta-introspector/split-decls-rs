// Generated macro for proptest_truncate (function)
macro_rules! Depcrate_testsproptest_truncate {
() => {
// Module: crate::tests
// Provides: {"proptest_truncate"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_truncate (# [strategy (rand_unicode ())] mut control : String , val : u8) { let initial_len = control . len () ; let mut compact = CompactString :: new (& control) ; let new_len = control . char_indices () . cycle () . nth (val as usize) . unwrap_or_default () . 0 ; control . truncate (new_len) ; compact . truncate (new_len) ; prop_assert_eq ! (& control , & compact) ; prop_assert_eq ! (control . len () , compact . len ()) ; if initial_len > MAX_SIZE { prop_assert ! (compact . is_heap_allocated ()) ; } else { prop_assert ! (! compact . is_heap_allocated ()) ; } }
};
}
