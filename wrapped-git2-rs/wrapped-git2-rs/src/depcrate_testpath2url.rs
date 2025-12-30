// Generated macro for path2url (function)
macro_rules! Depcrate_testpath2url {
() => {
// Module: crate::test
// Provides: {"path2url"}
// Dependencies: {}
pub fn path2url (path : & Path) -> String { Url :: from_file_path (path) . unwrap () . to_string () }
};
}
