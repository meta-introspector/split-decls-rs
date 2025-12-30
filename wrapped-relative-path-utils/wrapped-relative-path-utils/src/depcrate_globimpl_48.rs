// Generated macro for impl_48 (impl)
macro_rules! Depcrate_globimpl_48 {
() => {
// Module: crate::glob
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > Matcher < 'a > { # [doc = " Perform an expansion in the filesystem."] fn expand_filesystem < M > (& mut self , current : & RelativePath , rest : & 'a [Component < 'a >] , mut m : M ,) -> Result < () > where M : FnMut (& str) -> bool , { if let Ok (m) = self . root . metadata (current) { if ! m . is_dir () { return Ok (()) ; } } else { return Ok (()) ; } for e in self . root . read_dir (current) . map_err (ErrorKind :: ReadDir) ? { let e = e . map_err (ErrorKind :: DirEntry) ? ; let c = e . file_name () ; let c = c . to_string_lossy () ; if ! m (c . as_ref ()) { continue ; } let mut new = current . to_owned () ; new . push (c . as_ref ()) ; self . queue . push_back ((new , rest)) ; } Ok (()) } # [doc = " Perform star star expansion."] fn walk (& mut self , current : & RelativePathBuf , rest : & 'a [Component < 'a >]) -> Result < () > { self . queue . push_back ((current . clone () , rest)) ; let mut queue = VecDeque :: new () ; queue . push_back (current . clone ()) ; while let Some (current) = queue . pop_front () { let Ok (m) = self . root . metadata (& current) else { continue ; } ; if ! m . is_dir () { continue ; } for e in self . root . read_dir (& current) . map_err (ErrorKind :: ReadDir) ? { let e = e . map_err (ErrorKind :: DirEntry) ? ; let c = e . file_name () ; let c = c . to_string_lossy () ; let next = current . join (c . as_ref ()) ; self . queue . push_back ((next . clone () , rest)) ; queue . push_back (next) ; } } Ok (()) } }
};
}
