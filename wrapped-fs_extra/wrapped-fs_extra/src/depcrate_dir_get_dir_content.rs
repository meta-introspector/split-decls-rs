// Generated macro for _get_dir_content (function)
macro_rules! Depcrate_dir_get_dir_content {
() => {
// Module: crate::dir
// Provides: {"_get_dir_content"}
// Dependencies: {}
fn _get_dir_content < P > (path : P , mut depth : u64) -> Result < DirContent > where P : AsRef < Path > , { let mut directories = Vec :: new () ; let mut files = Vec :: new () ; let mut dir_size ; let item = path . as_ref () . to_str () ; if item . is_none () { err ! ("Invalid path" , ErrorKind :: InvalidPath) ; } let item = item . unwrap () . to_string () ; if path . as_ref () . is_dir () { dir_size = path . as_ref () . metadata () ? . len () ; directories . push (item) ; if depth == 0 || depth > 1 { if depth > 1 { depth -= 1 ; } for entry in read_dir (& path) ? { let _path = entry ? . path () ; match _get_dir_content (_path , depth) { Ok (items) => { let mut _files = items . files ; let mut _directories = items . directories ; dir_size += items . dir_size ; files . append (& mut _files) ; directories . append (& mut _directories) ; } Err (err) => return Err (err) , } } } } else { dir_size = path . as_ref () . metadata () ? . len () ; files . push (item) ; } Ok (DirContent { dir_size , files , directories , }) }
};
}
