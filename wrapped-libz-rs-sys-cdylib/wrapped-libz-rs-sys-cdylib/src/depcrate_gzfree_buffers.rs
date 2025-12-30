// Generated macro for free_buffers (function)
macro_rules! Depcrate_gzfree_buffers {
() => {
// Module: crate::gz
// Provides: {"free_buffers"}
// Dependencies: {}
unsafe fn free_buffers (state : & mut GzState) { if ! state . input . is_null () { unsafe { ALLOCATOR . deallocate (state . input , state . in_capacity ()) } ; state . input = ptr :: null_mut () ; } state . in_size = 0 ; if ! state . output . is_null () { unsafe { ALLOCATOR . deallocate (state . output , state . out_capacity ()) } ; state . output = ptr :: null_mut () ; } state . out_size = 0 ; }
};
}
