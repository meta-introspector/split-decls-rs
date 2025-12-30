// Generated macro for impl_203 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_203 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_203"}
// Dependencies: {}
impl < S : ScalarValue > LookAheadList < '_ , S > { # [doc = " Returns an [`Iterator`] over the items of this [list]."] # [doc = ""] # [doc = " [list]: https://spec.graphql.org/October2021#sec-List"] pub fn iter (& self) -> < & Self as IntoIterator > :: IntoIter { self . into_iter () } }
};
}
