// Generated macro for remove (function)
macro_rules! Depcrate_symlinkremove {
() => {
// Module: crate::symlink
// Provides: {"remove"}
// Dependencies: {}
# [doc = " Remove a symlink."] # [cfg (windows)] pub fn remove (path : & Path) -> io :: Result < () > { if let Ok (meta) = std :: fs :: metadata (path) { if meta . is_file () { std :: fs :: remove_file (path) } else { std :: fs :: remove_dir (path) } } else { std :: fs :: remove_file (path) . or_else (| _ | std :: fs :: remove_dir (path)) } }
};
}
