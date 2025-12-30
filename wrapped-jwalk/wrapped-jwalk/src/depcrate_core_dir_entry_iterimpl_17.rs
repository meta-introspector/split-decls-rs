// Generated macro for impl_17 (impl)
macro_rules! Depcrate_core_dir_entry_iterimpl_17 {
() => {
// Module: crate::core::dir_entry_iter
// Provides: {"impl_17"}
// Dependencies: {}
impl < C : ClientState > DirEntryIter < C > { pub (crate) fn new (root_entry_results : Vec < Result < DirEntry < C > > > , parallelism : Parallelism , min_depth : usize , root_read_dir_state : C :: ReadDirState , core_read_dir_callback : Arc < ReadDirCallback < C > > ,) -> DirEntryIter < C > { let read_dir_specs : Vec < _ > = root_entry_results . iter () . flat_map (| dir_entry_result | { dir_entry_result . as_ref () . ok () ? . read_children_spec (root_read_dir_state . clone ()) }) . collect () ; let read_dir_iter = ReadDirIter :: try_new (read_dir_specs , parallelism , core_read_dir_callback) . map (| iter | iter . peekable ()) ; DirEntryIter { min_depth , read_dir_iter , read_dir_results_stack : vec ! [root_entry_results . into_iter ()] , } } fn push_next_read_dir_results (iter : & mut Peekable < ReadDirIter < C > > , results : & mut Vec < vec :: IntoIter < Result < DirEntry < C > > > > ,) -> Result < () > { let read_dir_result = iter . next () . unwrap () ; let read_dir = match read_dir_result { Ok (read_dir) => read_dir , Err (err) => return Err (err) , } ; let ReadDir { results_list , .. } = read_dir ; results . push (results_list . into_iter ()) ; Ok (()) } }
};
}
