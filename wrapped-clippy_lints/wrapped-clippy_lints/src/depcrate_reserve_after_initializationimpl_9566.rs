// Generated macro for impl_9566 (impl)
macro_rules! Depcrate_reserve_after_initializationimpl_9566 {
() => {
// Module: crate::reserve_after_initialization
// Provides: {"impl_9566"}
// Dependencies: {}
impl VecReserveSearcher { fn display_err (& self , cx : & LateContext < '_ >) { if self . space_hint . is_empty () { return ; } let s = format ! ("{}Vec::with_capacity({});" , self . init_part , self . space_hint) ; span_lint_and_sugg (cx , RESERVE_AFTER_INITIALIZATION , self . err_span , "call to `reserve` immediately after creation" , "consider using `Vec::with_capacity(/* Space hint */)`" , s , Applicability :: HasPlaceholders ,) ; } }
};
}
