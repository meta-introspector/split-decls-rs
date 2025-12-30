// Generated macro for HelperThread (struct)
macro_rules! DepcrateHelperThread {
() => {
// Module: crate
// Provides: {"HelperThread"}
// Dependencies: {}
# [doc = " Structure returned from [`Client::into_helper_thread`] to manage the lifetime"] # [doc = " of the helper thread returned, see those associated docs for more info."] # [derive (Debug)] pub struct HelperThread { inner : Option < imp :: Helper > , state : Arc < HelperState > , }
};
}
