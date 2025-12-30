// Generated macro for test_from_char_iter (function)
macro_rules! Depcrate_teststest_from_char_iter {
() => {
// Module: crate::tests
// Provides: {"test_from_char_iter"}
// Dependencies: {}
# [test] # [cfg_attr (target_pointer_width = "32" , ignore)] fn test_from_char_iter () { let s = "\u{0} 0 \u{0}a𐀀𐀀 𐀀a𐀀" ; let compact : CompactString = s . chars () . collect () ; assert ! (! compact . is_heap_allocated ()) ; assert_eq ! (s , compact) ; }
};
}
