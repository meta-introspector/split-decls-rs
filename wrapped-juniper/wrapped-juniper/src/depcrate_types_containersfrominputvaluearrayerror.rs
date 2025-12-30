// Generated macro for FromInputValueArrayError (enum)
macro_rules! Depcrate_types_containersFromInputValueArrayError {
() => {
// Module: crate::types::containers
// Provides: {"FromInputValueArrayError"}
// Dependencies: {}
# [doc = " Error converting [`InputValue`] into exact-size [`array`](prim@array)."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum FromInputValueArrayError < T , S > where T : FromInputValue < S > , S : ScalarValue , { # [doc = " [`InputValue`] cannot be [`Null`]."] # [doc = ""] # [doc = " See [\"Combining List and Non-Null\" section of spec][1]."] # [doc = ""] # [doc = " [`Null`]: [`InputValue::Null`]"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Combining-List-and-Non-Null"] Null , # [doc = " Wrong count of items."] WrongCount { # [doc = " Actual count of items."] actual : usize , # [doc = " Expected count of items."] expected : usize , } , # [doc = " Error of converting [`InputValue::List`]'s item."] Item (T :: Error) , }
};
}
