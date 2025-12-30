// Generated macro for impl_377 (impl)
macro_rules! Depcrateimpl_377 {
() => {
// Module: crate
// Provides: {"impl_377"}
// Dependencies: {}
impl Stamp { # [doc = " Creates a timestamp holding the last-modified time of the specified file."] fn from_path (path : & Utf8Path) -> Self { let mut stamp = Stamp { time : SystemTime :: UNIX_EPOCH } ; stamp . add_path (path) ; stamp } # [doc = " Updates this timestamp to the last-modified time of the specified file,"] # [doc = " if it is later than the currently-stored timestamp."] fn add_path (& mut self , path : & Utf8Path) { let modified = fs :: metadata (path . as_std_path ()) . and_then (| metadata | metadata . modified ()) . unwrap_or (SystemTime :: UNIX_EPOCH) ; self . time = self . time . max (modified) ; } # [doc = " Updates this timestamp to the most recent last-modified time of all files"] # [doc = " recursively contained in the given directory, if it is later than the"] # [doc = " currently-stored timestamp."] fn add_dir (& mut self , path : & Utf8Path) { let path = path . as_std_path () ; for entry in WalkDir :: new (path) { let entry = entry . unwrap () ; if entry . file_type () . is_file () { let modified = entry . metadata () . ok () . and_then (| metadata | metadata . modified () . ok ()) . unwrap_or (SystemTime :: UNIX_EPOCH) ; self . time = self . time . max (modified) ; } } } }
};
}
