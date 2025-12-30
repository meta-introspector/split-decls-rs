// Generated macro for Glob (struct)
macro_rules! Depcrate_gitignoreGlob {
() => {
// Module: crate::gitignore
// Provides: {"Glob"}
// Dependencies: {}
# [doc = " Glob represents a single glob in a gitignore file."] # [doc = ""] # [doc = " This is used to report information about the highest precedent glob that"] # [doc = " matched in one or more gitignore files."] # [derive (Clone , Debug)] pub struct Glob { # [doc = " The file path that this glob was extracted from."] from : Option < PathBuf > , # [doc = " The original glob string."] original : String , # [doc = " The actual glob string used to convert to a regex."] actual : String , # [doc = " Whether this is a whitelisted glob or not."] is_whitelist : bool , # [doc = " Whether this glob should only match directories or not."] is_only_dir : bool , }
};
}
