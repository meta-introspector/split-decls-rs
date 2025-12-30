// Generated macro for Parts (struct)
macro_rules! Depcrate_responseParts {
() => {
// Module: crate::response
// Provides: {"Parts"}
// Dependencies: {}
# [doc = " Component parts of an HTTP `Response`"] # [doc = ""] # [doc = " The HTTP response head consists of a status, version, and a set of"] # [doc = " header fields."] # [derive (Clone)] pub struct Parts { # [doc = " The response's status"] pub status : StatusCode , # [doc = " The response's version"] pub version : Version , # [doc = " The response's headers"] pub headers : HeaderMap < HeaderValue > , # [doc = " The response's extensions"] pub extensions : Extensions , _priv : () , }
};
}
