// Generated macro for alloc_error_handler_name (function)
macro_rules! Depcrate_expand_allocatoralloc_error_handler_name {
() => {
// Module: crate::expand::allocator
// Provides: {"alloc_error_handler_name"}
// Dependencies: {}
pub fn alloc_error_handler_name (alloc_error_handler_kind : AllocatorKind) -> & 'static str { match alloc_error_handler_kind { AllocatorKind :: Global => "__rg_oom" , AllocatorKind :: Default => "__rdl_oom" , } }
};
}
