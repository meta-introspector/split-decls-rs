// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_alloctest {
() => {
// Module: crate::arbitrary::_alloc::alloc
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { multiplex_alloc ! (:: alloc :: alloc , :: std :: alloc) ; no_panic_test ! (layout => self :: alloc :: Layout , alloc_err => self :: alloc :: AllocError) ; }
};
}
