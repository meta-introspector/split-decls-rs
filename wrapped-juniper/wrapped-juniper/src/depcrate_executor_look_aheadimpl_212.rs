// Generated macro for impl_212 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_212 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_212"}
// Dependencies: {}
impl < S : ScalarValue > LookAheadObject < '_ , S > { # [doc = " Returns an [`Iterator`] over this [input object]'s fields."] # [doc = ""] # [doc = " [input object]: https://spec.graphql.org/October2021#sec-Input-Objects"] pub fn iter (& self) -> < & Self as IntoIterator > :: IntoIter { self . into_iter () } }
};
}
