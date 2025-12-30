// Generated macro for find_path_entry_in_commit (function)
macro_rules! Depcrate_file_functionfind_path_entry_in_commit {
() => {
// Module: crate::file::function
// Provides: {"find_path_entry_in_commit"}
// Dependencies: {}
fn find_path_entry_in_commit (odb : & impl gix_object :: Find , commit : & gix_hash :: oid , file_path : & BStr , cache : Option < & gix_commitgraph :: Graph > , buf : & mut Vec < u8 > , buf2 : & mut Vec < u8 > , stats : & mut Statistics ,) -> Result < Option < ObjectId > , Error > { let tree_id = find_commit (cache , odb , commit , buf) ? . tree_id () ? ; let tree_iter = odb . find_tree_iter (& tree_id , buf) ? ; stats . trees_decoded += 1 ; let res = tree_iter . lookup_entry (odb , buf2 , file_path . split (| b | * b == b'/') . inspect (| _ | stats . trees_decoded += 1) ,) ? ; stats . trees_decoded -= 1 ; Ok (res . map (| e | e . oid)) }
};
}
