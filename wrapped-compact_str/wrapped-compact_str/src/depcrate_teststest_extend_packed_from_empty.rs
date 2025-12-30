// Generated macro for test_extend_packed_from_empty (function)
macro_rules! Depcrate_teststest_extend_packed_from_empty {
() => {
// Module: crate::tests
// Provides: {"test_extend_packed_from_empty"}
// Dependencies: {}
# [test] # [cfg_attr (target_pointer_width = "32" , ignore)] fn test_extend_packed_from_empty () { let s = "  0\u{80}A\u{0}𐀀 𐀀¡a𐀀0" ; let mut compact = CompactString :: new (s) ; assert ! (! compact . is_heap_allocated ()) ; compact . extend ("" . chars ()) ; assert ! (! compact . is_heap_allocated ()) ; }
};
}
