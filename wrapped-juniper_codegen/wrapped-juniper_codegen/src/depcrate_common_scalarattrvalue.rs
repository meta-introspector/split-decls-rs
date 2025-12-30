// Generated macro for AttrValue (enum)
macro_rules! Depcrate_common_scalarAttrValue {
() => {
// Module: crate::common::scalar
// Provides: {"AttrValue"}
// Dependencies: {}
# [doc = " Possible values of `#[graphql(scalar = ...)]` attribute."] # [derive (Clone , Debug)] pub (crate) enum AttrValue { # [doc = " Concrete Rust type (like `DefaultScalarValue`)."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] Concrete (syn :: Type) , # [doc = " Generic Rust type parameter with a bound predicate"] # [doc = " (like `S: ScalarValue + Send + Sync`)."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] Generic (syn :: PredicateType) , }
};
}
