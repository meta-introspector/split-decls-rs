// Generated macro for redirect (function)
macro_rules! Depcrate_errorredirect {
() => {
// Module: crate::error
// Provides: {"redirect"}
// Dependencies: {}
pub (crate) fn redirect < E : Into < BoxError > > (e : E , url : Url) -> Error { Error :: new (Kind :: Redirect , Some (e)) . with_url (url) }
};
}
