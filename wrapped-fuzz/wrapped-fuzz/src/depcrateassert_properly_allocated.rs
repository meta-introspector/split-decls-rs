// Generated macro for assert_properly_allocated (function)
macro_rules! Depcrateassert_properly_allocated {
() => {
// Module: crate
// Provides: {"assert_properly_allocated"}
// Dependencies: {}
# [doc = " Asserts the provided CompactString is allocated properly either on the stack or on the heap,"] # [doc = " using a \"control\" `&str` for a reference length."] fn assert_properly_allocated (compact : & CompactString , control : & str) { assert_eq ! (compact . len () , control . len ()) ; if control . len () <= MAX_INLINE_LENGTH { assert ! (! compact . is_heap_allocated ()) ; } else { let is_static = compact . as_static_str () . is_some () ; let is_heap = compact . is_heap_allocated () ; assert ! (is_static || is_heap) ; } }
};
}
