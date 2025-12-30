// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl Drop for EnvChange < '_ > { fn drop (& mut self) { for name in self . changed_vars . drain (..) { unsafe { match self . snap . vars . get :: < std :: ffi :: OsStr > (name . as_ref ()) { Some (prev_val) => env :: set_var (name , prev_val) , None => env :: remove_var (name) , } } } if let Some (dir) = & self . prev_working_dir && let Err (err) = std :: env :: set_current_dir (dir) { eprintln ! ("Failed to set the current working dir to {}. Error: {:?}" , dir . display () , err) } } }
};
}
