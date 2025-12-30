// Generated macro for impl_198 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_198 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_198"}
// Dependencies: {}
impl AccessCause { fn print_as_access (self , is_foreign : bool) -> String { let rel = if is_foreign { "foreign" } else { "child" } ; match self { Self :: Explicit (kind) => format ! ("{rel} {kind}") , Self :: Reborrow => format ! ("reborrow (acting as a {rel} read access)") , Self :: Dealloc => format ! ("deallocation (acting as a {rel} write access)") , Self :: FnExit (kind) => format ! ("protector release (acting as a {rel} {kind})") , } } }
};
}
