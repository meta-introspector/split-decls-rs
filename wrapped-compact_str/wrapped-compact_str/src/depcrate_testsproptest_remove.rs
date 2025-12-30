// Generated macro for proptest_remove (function)
macro_rules! Depcrate_testsproptest_remove {
() => {
// Module: crate::tests
// Provides: {"proptest_remove"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_remove (# [strategy (rand_unicode_with_range (1 .. 80))] mut control : String , val : u8) { let initial_len = control . len () ; let mut compact = CompactString :: new (& control) ; let idx = control . char_indices () . cycle () . nth (val as usize) . unwrap_or_default () . 0 ; let control_char = control . remove (idx) ; let compact_char = compact . remove (idx) ; prop_assert_eq ! (control_char , compact_char) ; prop_assert_eq ! (control_char , compact_char) ; prop_assert_eq ! (control . len () , compact . len ()) ; if initial_len > MAX_SIZE { prop_assert ! (compact . is_heap_allocated ()) ; } else { prop_assert ! (! compact . is_heap_allocated ()) ; } }
};
}
