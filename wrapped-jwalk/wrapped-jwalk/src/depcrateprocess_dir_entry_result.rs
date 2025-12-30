// Generated macro for process_dir_entry_result (function)
macro_rules! Depcrateprocess_dir_entry_result {
() => {
// Module: crate
// Provides: {"process_dir_entry_result"}
// Dependencies: {}
fn process_dir_entry_result < C : ClientState > (dir_entry_result : Result < DirEntry < C > > , follow_links : bool ,) -> Result < DirEntry < C > > { match dir_entry_result { Ok (mut dir_entry) => { if follow_links && dir_entry . file_type . is_symlink () { dir_entry = dir_entry . follow_symlink () ? ; } if dir_entry . depth == 0 && dir_entry . file_type . is_symlink () { let metadata = fs :: metadata (dir_entry . path ()) . map_err (| err | Error :: from_path (0 , dir_entry . path () , err)) ? ; if metadata . file_type () . is_dir () { dir_entry . read_children_path = Some (Arc :: from (dir_entry . path ())) ; } } Ok (dir_entry) } Err (err) => Err (err) , } }
};
}
