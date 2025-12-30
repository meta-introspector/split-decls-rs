// Generated macro for free_state (function)
macro_rules! Depcrate_gzfree_state {
() => {
// Module: crate::gz
// Provides: {"free_state"}
// Dependencies: {}
unsafe fn free_state (state : * mut GzState) { if state . is_null () { return ; } unsafe { match (* state) . source { Source :: Path (path) => deallocate_cstr (path . cast_mut ()) , Source :: Fd (_) => { } } deallocate_cstr ((* state) . msg . cast_mut ()) ; } unsafe { free_buffers (state . as_mut () . unwrap ()) } ; unsafe { ALLOCATOR . deallocate (state , 1) } ; }
};
}
