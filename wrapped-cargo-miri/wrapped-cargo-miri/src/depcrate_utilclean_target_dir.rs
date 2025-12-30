// Generated macro for clean_target_dir (function)
macro_rules! Depcrate_utilclean_target_dir {
() => {
// Module: crate::util
// Provides: {"clean_target_dir"}
// Dependencies: {}
# [doc = " Deletes the Miri target directory"] pub fn clean_target_dir (meta : & Metadata) { let target_dir = get_target_dir (meta) ; eprintln ! ("Cleaning target directory at {}" , target_dir . display ()) ; remove_dir_all_idem (& target_dir) . unwrap_or_else (| err | show_error ! ("{}" , err)) }
};
}
