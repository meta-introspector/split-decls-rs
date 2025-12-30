// Generated macro for int_ty_to_string (function)
macro_rules! Depcrate_primitiveint_ty_to_string {
() => {
// Module: crate::primitive
// Provides: {"int_ty_to_string"}
// Dependencies: {}
pub fn int_ty_to_string (ty : IntTy) -> & 'static str { match ty { IntTy :: Isize => "isize" , IntTy :: I8 => "i8" , IntTy :: I16 => "i16" , IntTy :: I32 => "i32" , IntTy :: I64 => "i64" , IntTy :: I128 => "i128" , } }
};
}
