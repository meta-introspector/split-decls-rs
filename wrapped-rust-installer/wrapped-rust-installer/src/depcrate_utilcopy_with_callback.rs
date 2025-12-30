// Generated macro for copy_with_callback (function)
macro_rules! Depcrate_utilcopy_with_callback {
() => {
// Module: crate::util
// Provides: {"copy_with_callback"}
// Dependencies: {}
# [doc = " Copies the `src` directory recursively to `dst`. Both are assumed to exist"] # [doc = " when this function is called. Invokes a callback for each path visited."] pub fn copy_with_callback < F > (src : & Path , dst : & Path , mut callback : F) -> Result < () > where F : FnMut (& Path , fs :: FileType) -> Result < () > , { for entry in WalkDir :: new (src) . min_depth (1) { let entry = entry ? ; let file_type = entry . file_type () ; let path = entry . path () . strip_prefix (src) ? ; let dst = dst . join (path) ; if file_type . is_dir () { create_dir (& dst) ? ; } else { copy (entry . path () , dst) ? ; } callback (path , file_type) ? ; } Ok (()) }
};
}
