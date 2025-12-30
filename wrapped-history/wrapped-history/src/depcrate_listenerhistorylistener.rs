// Generated macro for HistoryListener (struct)
macro_rules! Depcrate_listenerHistoryListener {
() => {
// Module: crate::listener
// Provides: {"HistoryListener"}
// Dependencies: {}
# [doc = " A History Listener to manage callbacks registered on a [`History`][crate::History]."] # [doc = ""] # [doc = " This Listener has the same behaviour as the [`EventListener`][gloo_events::EventListener] from"] # [doc = " `gloo` that the underlying callback will be unregistered when the listener is dropped."] # [must_use = "the listener is removed when `HistoryListener` is dropped"] pub struct HistoryListener { pub (crate) _listener : Rc < dyn Fn () > , }
};
}
