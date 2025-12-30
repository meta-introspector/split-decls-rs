// Generated macro for impl_219 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_219 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'a , S > LookAheadArgument < 'a , S > { # [doc = " Returns the name of this [argument]."] # [doc = ""] # [doc = " [argument]: https://spec.graphql.org/October2021#sec-Language.Arguments"] # [must_use] pub fn name (& self) -> & 'a str { self . name . item } # [doc = " Returns the [`Span`] of this [argument]'s [`name`]."] # [doc = ""] # [doc = " [`name`]: LookAheadArgument::name()"] # [doc = " [argument]: https://spec.graphql.org/October2021#sec-Language.Arguments"] # [must_use] pub fn name_span (& self) -> & 'a Span { & self . name . span } # [doc = " Evaluates and returns the value of this [argument]."] # [doc = ""] # [doc = " [argument]: https://spec.graphql.org/October2021#sec-Language.Arguments"] pub fn value (& self) -> LookAheadValue < 'a , S > where S : ScalarValue , { LookAheadValue :: from_input_value (self . input_value . as_ref () , Some (self . vars)) . item } # [doc = " Returns the [`Span`] of this [argument]'s [`value`]."] # [doc = ""] # [doc = " [`value`]: LookAheadArgument::value()"] # [doc = " [argument]: https://spec.graphql.org/October2021#sec-Language.Arguments"] # [must_use] pub fn value_span (& self) -> & 'a Span { & self . input_value . span } }
};
}
