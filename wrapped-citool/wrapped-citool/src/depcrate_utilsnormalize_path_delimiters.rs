// Generated macro for normalize_path_delimiters (function)
macro_rules! Depcrate_utilsnormalize_path_delimiters {
() => {
// Module: crate::utils
// Provides: {"normalize_path_delimiters"}
// Dependencies: {}
# [doc = " Normalizes Windows-style path delimiters to Unix-style paths."] pub fn normalize_path_delimiters (name : & str) -> Cow < '_ , str > { if name . contains ("\\") { name . replace ('\\' , "/") . into () } else { name . into () } }
};
}
