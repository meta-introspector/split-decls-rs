// Generated macro for File (struct)
macro_rules! Depcrate_utilsFile {
() => {
// Module: crate::utils
// Provides: {"File"}
// Dependencies: {}
# [doc = " Wrapper around `std::fs::File` which panics with a path on failure."] pub struct File < 'a > { pub inner : fs :: File , pub path : & 'a Path , }
};
}
