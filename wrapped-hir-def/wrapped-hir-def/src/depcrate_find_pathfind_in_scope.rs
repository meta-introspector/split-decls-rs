// Generated macro for find_in_scope (function)
macro_rules! Depcrate_find_pathfind_in_scope {
() => {
// Module: crate::find_path
// Provides: {"find_in_scope"}
// Dependencies: {}
fn find_in_scope (db : & dyn DefDatabase , def_map : & DefMap , from : ModuleId , item : ItemInNs , ignore_local_imports : bool ,) -> Option < Name > { def_map . with_ancestor_maps (db , from . local_id , & mut | def_map , local_id | { def_map [local_id] . scope . names_of (item , | name , _ , declared | { (declared || ! ignore_local_imports) . then (| | name . clone ()) }) }) }
};
}
