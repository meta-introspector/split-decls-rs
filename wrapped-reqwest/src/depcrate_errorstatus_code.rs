// Generated macro for status_code (function)
macro_rules! Depcrate_errorstatus_code {
() => {
// Module: crate::error
// Provides: {"status_code"}
// Dependencies: {}
pub (crate) fn status_code (url : Url , status : StatusCode , # [cfg (not (target_arch = "wasm32"))] reason : Option < hyper :: ext :: ReasonPhrase > ,) -> Error { Error :: new (Kind :: Status (status , # [cfg (not (target_arch = "wasm32"))] reason ,) , None :: < Error > ,) . with_url (url) }
};
}
