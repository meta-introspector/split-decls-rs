// Generated macro for impl_98 (impl)
macro_rules! Depcrate_walkimpl_98 {
() => {
// Module: crate::walk
// Provides: {"impl_98"}
// Dependencies: {}
impl Walk { # [doc = " Creates a new recursive directory iterator for the file path given."] # [doc = ""] # [doc = " Note that this uses default settings, which include respecting"] # [doc = " `.gitignore` files. To configure the iterator, use `WalkBuilder`"] # [doc = " instead."] pub fn new < P : AsRef < Path > > (path : P) -> Walk { WalkBuilder :: new (path) . build () } fn skip_entry (& self , ent : & DirEntry) -> Result < bool , Error > { if ent . depth () == 0 { return Ok (false) ; } if should_skip_entry (& self . ig , ent) { return Ok (true) ; } if let Some (ref stdout) = self . skip { if path_equals (ent , stdout) ? { return Ok (true) ; } } if self . max_filesize . is_some () && ! ent . is_dir () { return Ok (skip_filesize (self . max_filesize . unwrap () , ent . path () , & ent . metadata () . ok () ,)) ; } if let Some (Filter (filter)) = & self . filter { if ! filter (ent) { return Ok (true) ; } } Ok (false) } }
};
}
