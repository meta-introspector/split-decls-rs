// Generated macro for container_name (function)
macro_rules! Depcrate_navigation_targetcontainer_name {
() => {
// Module: crate::navigation_target
// Provides: {"container_name"}
// Dependencies: {}
fn container_name (db : & RootDatabase , t : impl HasContainer) -> Option < Symbol > { match t . container (db) { hir :: ItemContainer :: Trait (it) => Some (it . name (db) . symbol () . clone ()) , hir :: ItemContainer :: Module (it) => it . name (db) . map (| name | name . symbol () . clone ()) , _ => None , } }
};
}
