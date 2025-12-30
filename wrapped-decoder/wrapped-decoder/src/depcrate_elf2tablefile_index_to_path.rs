// Generated macro for file_index_to_path (function)
macro_rules! Depcrate_elf2tablefile_index_to_path {
() => {
// Module: crate::elf2table
// Provides: {"file_index_to_path"}
// Dependencies: {}
fn file_index_to_path < R > (index : u64 , unit : & gimli :: Unit < R > , dwarf : & gimli :: Dwarf < R > ,) -> Result < PathBuf , anyhow :: Error > where R : gimli :: read :: Reader , { ensure ! (index != 0 , "`FileIndex` was zero") ; let header = if let Some (program) = & unit . line_program { program . header () } else { bail ! ("no `LineProgram`") ; } ; let file = if let Some (file) = header . file (index) { file } else { bail ! ("no `FileEntry` for index {}" , index) } ; let mut p = PathBuf :: new () ; if let Some (dir) = file . directory (header) { let dir = dwarf . attr_string (unit , dir) ? ; let dir_s = dir . to_string_lossy () ? ; let dir = Path :: new (& dir_s [..]) ; if ! dir . is_absolute () { if let Some (ref comp_dir) = unit . comp_dir { p . push (& comp_dir . to_string_lossy () ? [..]) ; } } p . push (dir) ; } p . push (& dwarf . attr_string (unit , file . path_name ()) ? . to_string_lossy () ? [..] ,) ; Ok (p) }
};
}
