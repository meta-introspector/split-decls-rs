// Generated macro for clean_sysroot (function)
macro_rules! Depcrate_utilclean_sysroot {
() => {
// Module: crate::util
// Provides: {"clean_sysroot"}
// Dependencies: {}
# [doc = " Deletes the Miri sysroot cache"] # [doc = " Returns an error if the MIRI_SYSROOT env var is set."] pub fn clean_sysroot () { if std :: env :: var_os ("MIRI_SYSROOT") . is_some () { show_error ! ("MIRI_SYSROOT is set. Please clean your custom sysroot cache directory manually.") } let sysroot_dir = get_sysroot_dir () ; eprintln ! ("Cleaning sysroot cache at {}" , sysroot_dir . display ()) ; remove_dir_all_idem (& sysroot_dir) . unwrap_or_else (| err | show_error ! ("{}" , err)) ; }
};
}
