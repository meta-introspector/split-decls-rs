// Generated macro for is_dyn_sym (function)
macro_rules! Depcrate_shims_unix_android_foreign_itemsis_dyn_sym {
() => {
// Module: crate::shims::unix::android::foreign_items
// Provides: {"is_dyn_sym"}
// Dependencies: {}
pub fn is_dyn_sym (name : & str) -> bool { matches ! (name , "gettid") }
};
}
