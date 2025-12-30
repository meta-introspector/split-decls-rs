// Generated macro for impl_31 (impl)
macro_rules! Depcrate_browserimpl_31 {
() => {
// Module: crate::browser
// Provides: {"impl_31"}
// Dependencies: {}
impl Default for BrowserHistory { fn default () -> Self { thread_local ! { static BROWSER_HISTORY : (BrowserHistory , EventListener) = { let window = window () ; let inner = window . history () . expect_throw ("Failed to create browser history. Are you using a browser?") ; let callbacks = Rc :: default () ; let history = BrowserHistory { inner , callbacks , states : Rc :: default () , } ; let listener = { let history = history . clone () ; EventListener :: new (& window , "popstate" , move | _ | { history . notify_callbacks () ; }) } ; (history , listener) } ; } BROWSER_HISTORY . with (| (history , _) | history . clone ()) } }
};
}
