// Generated macro for impl_32 (impl)
macro_rules! Depcrate_browserimpl_32 {
() => {
// Module: crate::browser
// Provides: {"impl_32"}
// Dependencies: {}
impl BrowserHistory { # [doc = " Creates a new [`BrowserHistory`]"] pub fn new () -> Self { Self :: default () } fn notify_callbacks (& self) { crate :: utils :: notify_callbacks (self . callbacks . clone ()) ; } fn create_history_state () -> (u32 , JsValue) { let history_state = HistoryState :: new () ; (history_state . id () , serde_wasm_bindgen :: to_value (& history_state) . expect_throw ("fails to create history state.") ,) } pub (crate) fn combine_url (route : & str , query : & str) -> String { let href = window () . location () . href () . expect_throw ("Failed to read location href") ; let url = Url :: new_with_base (route , & href) . expect_throw ("current url is not valid.") ; url . set_search (query) ; url . href () } }
};
}
