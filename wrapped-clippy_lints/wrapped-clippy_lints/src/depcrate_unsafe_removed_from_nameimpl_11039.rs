// Generated macro for impl_11039 (impl)
macro_rules! Depcrate_unsafe_removed_from_nameimpl_11039 {
() => {
// Module: crate::unsafe_removed_from_name
// Provides: {"impl_11039"}
// Dependencies: {}
impl EarlyLintPass for UnsafeNameRemoval { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: Use (ref use_tree) = item . kind { check_use_tree (use_tree , cx , item . span) ; } } }
};
}
