// Generated macro for DefaultScalarValue (enum)
macro_rules! Depcrate_value_scalarDefaultScalarValue {
() => {
// Module: crate::value::scalar
// Provides: {"DefaultScalarValue"}
// Dependencies: {}
# [doc = " The default [`ScalarValue`] representation in [`juniper`]."] # [doc = ""] # [doc = " These types closely follow the [GraphQL specification][0]."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021"] # [derive (Clone , Debug , Display , From , PartialEq , ScalarValue , Serialize , TryInto)] # [serde (untagged)] pub enum DefaultScalarValue { # [doc = " [`Int` scalar][0] as a signed 32‐bit numeric non‐fractional value."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec-Int"] # [from] # [value (to_float , to_int)] Int (i32) , # [doc = " [`Float` scalar][0] as a signed double‐precision fractional values as"] # [doc = " specified by [IEEE 754]."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec-Float"] # [doc = " [IEEE 754]: https://en.wikipedia.org/wiki/IEEE_floating_point"] # [from] # [value (to_float)] Float (f64) , # [doc = " [`String` scalar][0] as a textual data, represented as UTF‐8 character"] # [doc = " sequences."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec-String"] # [from (& str , Cow <'_ , str >, String)] # [value (as_str , to_string)] String (String) , # [doc = " [`Boolean` scalar][0] as a `true` or `false` value."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sec-Boolean"] # [from] # [value (to_bool)] Boolean (bool) , }
};
}
