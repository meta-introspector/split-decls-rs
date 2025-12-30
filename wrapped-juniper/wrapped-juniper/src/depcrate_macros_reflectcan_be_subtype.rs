// Generated macro for can_be_subtype (function)
macro_rules! Depcrate_macros_reflectcan_be_subtype {
() => {
// Module: crate::macros::reflect
// Provides: {"can_be_subtype"}
// Dependencies: {}
# [doc = " Checks whether the given GraphQL [object][1] represents a `subtype` of the"] # [doc = " given GraphQL `ty`pe, basing on the [`WrappedType`] encoding."] # [doc = ""] # [doc = " To fully determine the sub-typing relation the [`Type`] should be one of the"] # [doc = " [`BaseSubTypes::NAMES`]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] # [must_use] pub const fn can_be_subtype (ty : WrappedValue , subtype : WrappedValue) -> bool { let ty_curr = ty % 10 ; let sub_curr = subtype % 10 ; if ty_curr == sub_curr { if ty_curr == 1 { true } else { can_be_subtype (ty / 10 , subtype / 10) } } else if ty_curr == 2 { can_be_subtype (ty / 10 , subtype) } else { false } }
};
}
