// Generated macro for acquire_shallow_lock (function)
macro_rules! Depcrate_fetch_functionacquire_shallow_lock {
() => {
// Module: crate::fetch::function
// Provides: {"acquire_shallow_lock"}
// Dependencies: {}
fn acquire_shallow_lock (shallow_file : & Path) -> Result < gix_lock :: File , Error > { gix_lock :: File :: acquire_to_update_resource (shallow_file , gix_lock :: acquire :: Fail :: Immediately , None) . map_err (Into :: into) }
};
}
