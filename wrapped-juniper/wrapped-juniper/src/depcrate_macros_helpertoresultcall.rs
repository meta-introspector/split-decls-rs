// Generated macro for ToResultCall (trait)
macro_rules! Depcrate_macros_helperToResultCall {
() => {
// Module: crate::macros::helper
// Provides: {"ToResultCall"}
// Dependencies: {}
# [doc = " [Autoref-based specialized][0] coercion into a [`Result`] for a function call for providing a"] # [doc = " return-type polymorphism in macros."] # [doc = ""] # [doc = " # Priority"] # [doc = ""] # [doc = " 1. Functions returning [`Result`] are propagated \"as is\"."] # [doc = ""] # [doc = " 2. Any other function's output is wrapped into [`Result`] with an [`Infallible`] [`Err`]."] # [doc = ""] # [doc = " [0]: https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html"] pub trait ToResultCall { # [doc = " Input of this function."] type Input ; # [doc = " Output of this function."] type Output ; # [doc = " Error of the [`Result`] coercion for this function."] type Error ; # [doc = " Calls this function, coercing its output into a [`Result`]."] fn __to_result_call (& self , input : Self :: Input) -> Result < Self :: Output , Self :: Error > ; }
};
}
