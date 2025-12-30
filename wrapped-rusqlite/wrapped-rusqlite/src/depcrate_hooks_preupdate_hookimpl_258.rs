// Generated macro for impl_258 (impl)
macro_rules! Depcrate_hooks_preupdate_hookimpl_258 {
() => {
// Module: crate::hooks::preupdate_hook
// Provides: {"impl_258"}
// Dependencies: {}
impl PreUpdateOldValueAccessor { # [doc = " Get the amount of columns in the row being deleted/updated."] pub fn get_column_count (& self) -> i32 { unsafe { ffi :: sqlite3_preupdate_count (self . db) } } # [doc = " Get the depth of the query that triggered the preupdate hook."] # [doc = " Returns 0 if the preupdate callback was invoked as a result of"] # [doc = " a direct insert, update, or delete operation;"] # [doc = " 1 for inserts, updates, or deletes invoked by top-level triggers;"] # [doc = " 2 for changes resulting from triggers called by top-level triggers; and so forth."] pub fn get_query_depth (& self) -> i32 { unsafe { ffi :: sqlite3_preupdate_depth (self . db) } } # [doc = " Get the row id of the row being updated/deleted."] pub fn get_old_row_id (& self) -> i64 { self . old_row_id } # [doc = " Get the value of the row being updated/deleted at the specified index."] pub fn get_old_column_value (& self , i : i32) -> Result < ValueRef < '_ > > { let mut p_value : * mut ffi :: sqlite3_value = ptr :: null_mut () ; unsafe { check (ffi :: sqlite3_preupdate_old (self . db , i , & mut p_value)) ? ; Ok (ValueRef :: from_value (p_value)) } } }
};
}
