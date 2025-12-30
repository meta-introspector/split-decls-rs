// Generated macro for names_of_path (function)
macro_rules! Depcrate_snapshotnames_of_path {
() => {
// Module: crate::snapshot
// Provides: {"names_of_path"}
// Dependencies: {}
# [doc = " Extracts the module and snapshot name from a snapshot path"] fn names_of_path (path : & Path) -> (String , String) { let parts : Vec < & str > = path . file_stem () . unwrap () . to_str () . unwrap_or ("") . rsplitn (2 , "__") . collect () ; match parts . as_slice () { [snapshot_name , module_name] => (snapshot_name . to_string () , module_name . to_string ()) , [snapshot_name] => (snapshot_name . to_string () , String :: new ()) , _ => (String :: new () , "<unknown>" . to_string ()) , } }
};
}
