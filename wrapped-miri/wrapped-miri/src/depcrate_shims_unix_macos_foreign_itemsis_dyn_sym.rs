// Generated macro for is_dyn_sym (function)
macro_rules! Depcrate_shims_unix_macos_foreign_itemsis_dyn_sym {
() => {
// Module: crate::shims::unix::macos::foreign_items
// Provides: {"is_dyn_sym"}
// Dependencies: {}
pub fn is_dyn_sym (name : & str) -> bool { match name { "os_sync_wait_on_address" | "os_sync_wait_on_address_with_deadline" | "os_sync_wait_on_address_with_timeout" | "os_sync_wake_by_address_any" | "os_sync_wake_by_address_all" => true , _ => false , } }
};
}
