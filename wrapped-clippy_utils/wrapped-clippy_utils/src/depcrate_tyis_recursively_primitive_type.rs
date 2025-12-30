// Generated macro for is_recursively_primitive_type (function)
macro_rules! Depcrate_tyis_recursively_primitive_type {
() => {
// Module: crate::ty
// Provides: {"is_recursively_primitive_type"}
// Dependencies: {}
# [doc = " Returns `true` if the given type is a primitive (a `bool` or `char`, any integer or"] # [doc = " floating-point number type, a `str`, or an array, slice, or tuple of those types)."] pub fn is_recursively_primitive_type (ty : Ty < '_ >) -> bool { match * ty . kind () { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str => true , ty :: Ref (_ , inner , _) if inner . is_str () => true , ty :: Array (inner_type , _) | ty :: Slice (inner_type) => is_recursively_primitive_type (inner_type) , ty :: Tuple (inner_types) => inner_types . iter () . all (is_recursively_primitive_type) , _ => false , } }
};
}
