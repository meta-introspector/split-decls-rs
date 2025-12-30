// Generated macro for impl_8 (impl)
macro_rules! Depcrate_accessimpl_8 {
() => {
// Module: crate::access
// Provides: {"impl_8"}
// Dependencies: {}
# [doc = " Access fundamentals"] impl Graph { fn lookup_by_id (& self , id : & gix_hash :: oid) -> Option < LookupByIdResult < '_ > > { let mut current_file_start = 0 ; for file in & self . files { if let Some (lex_pos) = file . lookup (id) { return Some (LookupByIdResult { file , file_pos : lex_pos , graph_pos : Position (current_file_start + lex_pos . 0) , }) ; } current_file_start += file . num_commits () ; } None } fn lookup_by_pos (& self , pos : Position) -> LookupByPositionResult < '_ > { let mut remaining = pos . 0 ; for (file_index , file) in self . files . iter () . enumerate () { match remaining . checked_sub (file . num_commits ()) { Some (v) => remaining = v , None => { return LookupByPositionResult { file , _file_index : file_index , pos : file :: Position (remaining) , } } } } panic ! ("graph position too large: {}" , pos . 0) ; } }
};
}
