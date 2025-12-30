// Generated macro for AnyDebug (trait)
macro_rules! Depcrate_core_builderAnyDebug {
() => {
// Module: crate::core::builder
// Provides: {"AnyDebug"}
// Dependencies: {}
# [doc = " This trait is similar to `Any`, except that it also exposes the underlying"] # [doc = " type's [`Debug`] implementation."] # [doc = ""] # [doc = " (Trying to debug-print `dyn Any` results in the unhelpful `\"Any { .. }\"`.)"] pub trait AnyDebug : Any + Debug { }
};
}
