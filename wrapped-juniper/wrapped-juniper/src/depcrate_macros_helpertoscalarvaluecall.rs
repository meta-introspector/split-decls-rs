// Generated macro for ToScalarValueCall (trait)
macro_rules! Depcrate_macros_helperToScalarValueCall {
() => {
// Module: crate::macros::helper
// Provides: {"ToScalarValueCall"}
// Dependencies: {}
# [doc = " [Autoref-based specialized][0] coercion into a [`ScalarValue`] for a function call for providing"] # [doc = " a return-type polymorphism in macros."] # [doc = ""] # [doc = " # Priority"] # [doc = ""] # [doc = " 1. Functions returning a [`ScalarValue`] are propagated \"as is\"."] # [doc = ""] # [doc = " 2. Functions returning a [`String`] are followed by [`From<String>`] conversion."] # [doc = ""] # [doc = " 3. Functions returning anything implementing [`ToScalarValue`] conversion are followed by this"] # [doc = "    conversion."] # [doc = ""] # [doc = " 4. Functions returning anything implementing [`Display`] are followed by the"] # [doc = "    [`ScalarValue::from_displayable_non_static()`] method call."] # [doc = ""] # [doc = " [0]: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html"] pub trait ToScalarValueCall < S : ScalarValue > { # [doc = " Input of this function."] type Input ; # [doc = " Calls this function, coercing its output into a [`ScalarValue`]."] fn __to_scalar_value_call (& self , input : Self :: Input) -> S ; }
};
}
