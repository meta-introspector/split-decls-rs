// Generated macro for proptest_char_iterator_roundtrips (function)
macro_rules! Depcrate_testsproptest_char_iterator_roundtrips {
() => {
// Module: crate::tests
// Provides: {"proptest_char_iterator_roundtrips"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_char_iterator_roundtrips (# [strategy (rand_unicode ())] word : String) { let compact : CompactString = word . clone () . chars () . collect () ; prop_assert_eq ! (& word , & compact) }
};
}
