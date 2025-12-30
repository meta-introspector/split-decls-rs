// Generated macro for impl_1358 (impl)
macro_rules! Depcrate_unusedimpl_1358 {
() => {
// Module: crate::unused
// Provides: {"impl_1358"}
// Dependencies: {}
impl EarlyLintPass for UnusedImportBraces { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let ast :: ItemKind :: Use (ref use_tree) = item . kind { self . check_use_tree (cx , use_tree , item) ; } } }
};
}
