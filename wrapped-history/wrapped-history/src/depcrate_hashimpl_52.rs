// Generated macro for impl_52 (impl)
macro_rules! Depcrate_hashimpl_52 {
() => {
// Module: crate::hash
// Provides: {"impl_52"}
// Dependencies: {}
impl Default for HashHistory { fn default () -> Self { thread_local ! { static HASH_HISTORY : HashHistory = { let browser_history = BrowserHistory :: new () ; let browser_location = browser_history . location () ; let current_hash = browser_location . hash () ; if current_hash . is_empty () || ! current_hash . starts_with ("#/") { let url = HashHistory :: get_url () ; url . set_hash ("#/") ; browser_history . replace (url . href ()) ; } HashHistory { inner : browser_history , } } ; } HASH_HISTORY . with (| s | s . clone ()) } }
};
}
