// Generated macro for FromInputValueVecError (enum)
macro_rules! Depcrate_types_containersFromInputValueVecError {
() => {
// Module: crate::types::containers
// Provides: {"FromInputValueVecError"}
// Dependencies: {}
# [doc = " Possible errors of converting [`InputValue`] into [`Vec`]."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum FromInputValueVecError < T , S > where T : FromInputValue < S > , S : ScalarValue , { # [doc = " [`InputValue`] cannot be [`Null`]."] # [doc = ""] # [doc = " See [\"Combining List and Non-Null\" section of spec][1]."] # [doc = ""] # [doc = " [`Null`]: [`InputValue::Null`]"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Combining-List-and-Non-Null"] Null , # [doc = " Error of converting [`InputValue::List`]'s item."] Item (T :: Error) , }
};
}
