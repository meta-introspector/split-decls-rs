// Generated macro for is_non_aggregate_primitive_type (function)
macro_rules! Depcrate_tyis_non_aggregate_primitive_type {
() => {
// Module: crate::ty
// Provides: {"is_non_aggregate_primitive_type"}
// Dependencies: {}
# [doc = " Returns `true` if the given type is a non aggregate primitive (a `bool` or `char`, any"] # [doc = " integer or floating-point number type)."] # [doc = ""] # [doc = " For checking aggregation of primitive types (e.g. tuples and slices of primitive type) see"] # [doc = " `is_recursively_primitive_type`"] pub fn is_non_aggregate_primitive_type (ty : Ty < '_ >) -> bool { matches ! (ty . kind () , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_)) }
};
}
