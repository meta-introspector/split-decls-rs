// Generated macro for impl_308 (impl)
macro_rules! Depcrate_matchers_contains_matcherimpl_308 {
() => {
// Module: crate::matchers::contains_matcher
// Provides: {"impl_308"}
// Dependencies: {}
impl < InnerMatcherT > ContainsMatcher < InnerMatcherT > { # [doc = " Configures this instance to match containers which contain a number of"] # [doc = " matching items matched by `count`."] # [doc = ""] # [doc = " For example, to assert that exactly three matching items must be"] # [doc = " present, use:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " contains(...).times(eq(3))"] # [doc = " ```"] # [doc = ""] # [doc = " One can also use `times(eq(0))` to test for the *absence* of an item"] # [doc = " matching the expected value."] pub fn times (mut self , count : impl Matcher < usize > + 'static) -> Self { self . count = Some (Box :: new (count)) ; self } }
};
}
