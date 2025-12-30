// Generated macro for Nullable (enum)
macro_rules! Depcrate_types_nullableNullable {
() => {
// Module: crate::types::nullable
// Provides: {"Nullable"}
// Dependencies: {}
# [doc = " `Nullable` can be used in situations where you need to distinguish between an implicitly and"] # [doc = " explicitly null input value."] # [doc = ""] # [doc = " The GraphQL spec states that these two field calls are similar, but are not identical:"] # [doc = ""] # [doc = " ```graphql"] # [doc = " {"] # [doc = "   field(arg: null)"] # [doc = "   field"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The first has explicitly provided null to the argument “arg”, while the second has implicitly"] # [doc = " not provided a value to the argument “arg”. These two forms may be interpreted differently. For"] # [doc = " example, a mutation representing deleting a field vs not altering a field, respectively."] # [doc = ""] # [doc = " In cases where you do not need to be able to distinguish between the two types of null, you"] # [doc = " should simply use `Option<T>`."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub enum Nullable < T > { # [doc = " No value"] # [default] ImplicitNull , # [doc = " No value, explicitly specified to be null"] ExplicitNull , # [doc = " Some value `T`"] Some (T) , }
};
}
