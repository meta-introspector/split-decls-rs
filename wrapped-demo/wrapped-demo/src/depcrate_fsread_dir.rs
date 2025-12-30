// Generated macro for read_dir (function)
macro_rules! Depcrate_fsread_dir {
() => {
// Module: crate::fs
// Provides: {"read_dir"}
// Dependencies: {}
pub fn read_dir (path : & str) -> io :: Result < () > { eprintln ! () ; assert ! (Path :: new (path) . is_dir ()) ; eprintln ! ("Reading {path:?} directory entries") ; let entries = fs :: read_dir (path) ? . collect :: < Result < Vec < _ > , _ > > () ? ; assert ! (! entries . is_empty ()) ; for entry in entries { let path = entry . path () ; eprintln ! ("Found {path:?}") ; } Ok (()) }
};
}
