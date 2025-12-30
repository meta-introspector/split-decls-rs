// Generated macro for PreUpdateNewValueAccessor (struct)
macro_rules! Depcrate_hooks_preupdate_hookPreUpdateNewValueAccessor {
() => {
// Module: crate::hooks::preupdate_hook
// Provides: {"PreUpdateNewValueAccessor"}
// Dependencies: {}
# [doc = " An accessor to access the new values of the row being inserted/updated"] # [doc = " during the preupdate callback."] # [derive (Debug)] pub struct PreUpdateNewValueAccessor { db : * mut ffi :: sqlite3 , new_row_id : i64 , }
};
}
