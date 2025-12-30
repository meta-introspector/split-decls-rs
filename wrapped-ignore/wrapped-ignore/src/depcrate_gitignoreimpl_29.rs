// Generated macro for impl_29 (impl)
macro_rules! Depcrate_gitignoreimpl_29 {
() => {
// Module: crate::gitignore
// Provides: {"impl_29"}
// Dependencies: {}
impl Glob { # [doc = " Returns the file path that defined this glob."] pub fn from (& self) -> Option < & Path > { self . from . as_ref () . map (| p | & * * p) } # [doc = " The original glob as it was defined in a gitignore file."] pub fn original (& self) -> & str { & self . original } # [doc = " The actual glob that was compiled to respect gitignore"] # [doc = " semantics."] pub fn actual (& self) -> & str { & self . actual } # [doc = " Whether this was a whitelisted glob or not."] pub fn is_whitelist (& self) -> bool { self . is_whitelist } # [doc = " Whether this glob must match a directory or not."] pub fn is_only_dir (& self) -> bool { self . is_only_dir } # [doc = " Returns true if and only if this glob has a `**/` prefix."] fn has_doublestar_prefix (& self) -> bool { self . actual . starts_with ("**/") || self . actual == "**" } }
};
}
