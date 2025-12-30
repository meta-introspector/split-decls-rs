// Generated macro for BrowserHistory (struct)
macro_rules! Depcrate_browserBrowserHistory {
() => {
// Module: crate::browser
// Provides: {"BrowserHistory"}
// Dependencies: {}
# [doc = " A [`History`] that is implemented with [`web_sys::History`] that provides native browser"] # [doc = " history and state access."] # [derive (Clone)] pub struct BrowserHistory { inner : web_sys :: History , states : Rc < RefCell < StateMap > > , callbacks : Rc < RefCell < Vec < WeakCallback > > > , }
};
}
