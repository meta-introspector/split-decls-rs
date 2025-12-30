// Generated macro for PreUpdateOldValueAccessor (struct)
macro_rules! Depcrate_hooks_preupdate_hookPreUpdateOldValueAccessor {
() => {
// Module: crate::hooks::preupdate_hook
// Provides: {"PreUpdateOldValueAccessor"}
// Dependencies: {}
# [doc = " An accessor to access the old values of the row being deleted/updated during the preupdate callback."] # [derive (Debug)] pub struct PreUpdateOldValueAccessor { db : * mut ffi :: sqlite3 , old_row_id : i64 , }
};
}
