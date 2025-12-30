// Generated macro for builder (module)
macro_rules! Depcrate_client_proxy_matcherbuilder {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"builder"}
// Dependencies: {}
mod builder { # [doc = " A type that can used as a `Builder` value."] # [doc = ""] # [doc = " Private and sealed, only visible in docs."] pub trait IntoValue { # [doc (hidden)] fn into_value (self) -> String ; } impl IntoValue for String { # [doc (hidden)] fn into_value (self) -> String { self } } impl IntoValue for & String { # [doc (hidden)] fn into_value (self) -> String { self . into () } } impl IntoValue for & str { # [doc (hidden)] fn into_value (self) -> String { self . into () } } }
};
}
