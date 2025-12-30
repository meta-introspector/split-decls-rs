// Generated macro for proptest_extend_chars_allocated_properly (function)
macro_rules! Depcrate_testsproptest_extend_chars_allocated_properly {
() => {
// Module: crate::tests
// Provides: {"proptest_extend_chars_allocated_properly"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_extend_chars_allocated_properly (# [strategy (rand_unicode ())] start : String , # [strategy (rand_unicode ())] extend : String ,) { let mut compact = CompactString :: new (& start) ; compact . extend (extend . chars ()) ; let mut control = start . clone () ; # [allow (clippy :: string_extend_chars)] control . extend (extend . chars ()) ; prop_assert_eq ! (& compact , & control) ; assert_allocated_properly (& compact) ; }
};
}
