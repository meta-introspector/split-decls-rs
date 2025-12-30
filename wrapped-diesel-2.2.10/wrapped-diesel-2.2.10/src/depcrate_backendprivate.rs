// Generated macro for private (module)
macro_rules! Depcrate_backendprivate {
() => {
// Module: crate::backend
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { # [doc = " This is a marker trait which indicates that"] # [doc = " diesel may specialize a certain [`QueryFragment`]"] # [doc = " impl in a later version. If you as a user encounter, where rustc"] # [doc = " suggests adding this a bound to a type implementing `Backend`"] # [doc = " consider adding the following bound instead"] # [doc = " `YourQueryType: QueryFragment<DB>` (the concrete bound"] # [doc = " is likely mentioned by rustc as part of a `note: …`)"] # [doc = ""] # [doc = " For any user implementing a custom backend: You likely want to implement"] # [doc = " this trait for your custom backend type to opt in the existing [`QueryFragment`] impls in diesel."] # [doc = " As indicated by the `i-implement-a-third-party-backend-and-opt-into-breaking-changes` feature"] # [doc = " diesel reserves the right to specialize any generic [`QueryFragment`](crate::query_builder::QueryFragment)"] # [doc = " impl via [`SqlDialect`](super::SqlDialect) in a later minor version release"] # [doc = ""] # [doc = " [`QueryFragment`]: crate::query_builder::QueryFragment"] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub trait DieselReserveSpecialization { } # [doc = " This trait just indicates that none implements"] # [doc = " [`SqlDialect`](super::SqlDialect) without enabling the"] # [doc = " `i-implement-a-third-party-backend-and-opt-into-breaking-changes`"] # [doc = " feature flag."] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] pub trait TrustedBackend { } }
};
}
