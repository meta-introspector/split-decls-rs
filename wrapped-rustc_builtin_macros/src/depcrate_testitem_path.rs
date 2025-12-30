// Generated macro for item_path (function)
macro_rules! Depcrate_testitem_path {
() => {
// Module: crate::test
// Provides: {"item_path"}
// Dependencies: {}
fn item_path (mod_path : & [Ident] , item_ident : & Ident) -> String { join_path_idents (mod_path . iter () . chain (iter :: once (item_ident))) }
};
}
