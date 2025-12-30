// Generated macro for remove_items (function)
macro_rules! Depcrateremove_items {
() => {
// Module: crate
// Provides: {"remove_items"}
// Dependencies: {}
# [doc = " Removes a list of files or directories."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = "  let mut from_paths = Vec::new();"] # [doc = "  from_paths.push(\"source/dir1\");"] # [doc = "  from_paths.push(\"source/file.txt\");"] # [doc = ""] # [doc = "  remove_items(&from_paths).unwrap();"] # [doc = " ```"] # [doc = ""] pub fn remove_items < P > (from_items : & [P]) -> Result < () > where P : AsRef < Path > , { for item in from_items { let item = item . as_ref () ; if item . is_dir () { dir :: remove (item) ? ; } else { file :: remove (item) ? } } Ok (()) }
};
}
