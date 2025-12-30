// Generated macro for assert_allocated_properly (function)
macro_rules! Depcrate_testsassert_allocated_properly {
() => {
// Module: crate::tests
// Provides: {"assert_allocated_properly"}
// Dependencies: {}
# [doc = " Asserts a [`CompactString`] is allocated properly"] fn assert_allocated_properly (compact : & CompactString) { if compact . len () <= MAX_SIZE { assert ! (! compact . is_heap_allocated ()) } else { assert ! (compact . is_heap_allocated ()) } }
};
}
