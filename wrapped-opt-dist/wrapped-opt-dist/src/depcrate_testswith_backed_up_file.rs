// Generated macro for with_backed_up_file (function)
macro_rules! Depcrate_testswith_backed_up_file {
() => {
// Module: crate::tests
// Provides: {"with_backed_up_file"}
// Dependencies: {}
# [doc = " Backup `path` (if it exists), then write `contents` into it, and then restore the original"] # [doc = " contents of the file."] fn with_backed_up_file < F > (path : & Path , contents : & str , func : F) -> anyhow :: Result < () > where F : FnOnce () -> anyhow :: Result < () > , { let original_contents = if path . is_file () { Some (std :: fs :: read_to_string (path) ?) } else { None } ; std :: fs :: write (path , contents) ? ; let ret = func () ; if let Some (original_contents) = original_contents { std :: fs :: write (path , original_contents) ? ; } ret }
};
}
