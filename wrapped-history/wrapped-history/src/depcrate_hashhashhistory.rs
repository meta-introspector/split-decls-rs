// Generated macro for HashHistory (struct)
macro_rules! Depcrate_hashHashHistory {
() => {
// Module: crate::hash
// Provides: {"HashHistory"}
// Dependencies: {}
# [doc = " A [`History`] that is implemented with [`web_sys::History`] and stores path in `#`(fragment)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " HashHistory does not support relative paths and will panic if routes are not starting with `/`."] # [derive (Clone , PartialEq)] pub struct HashHistory { inner : BrowserHistory , }
};
}
