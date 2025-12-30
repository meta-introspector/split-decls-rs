// Generated macro for proptest_string_iterator_roundtrips (function)
macro_rules! Depcrate_testsproptest_string_iterator_roundtrips {
() => {
// Module: crate::tests
// Provides: {"proptest_string_iterator_roundtrips"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_string_iterator_roundtrips (# [strategy (rand_unicode_collection ())] collection : Vec < String > ,) { let compact : CompactString = collection . clone () . into_iter () . collect () ; let word : String = collection . into_iter () . collect () ; prop_assert_eq ! (& word , & compact) ; }
};
}
