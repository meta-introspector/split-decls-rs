// Generated macro for dynamic_to_fixed (function)
macro_rules! Depcrate_ir_dynamic_typedynamic_to_fixed {
() => {
// Module: crate::ir::dynamic_type
// Provides: {"dynamic_to_fixed"}
// Dependencies: {}
# [doc = " Convert a dynamic-vector type to a fixed-vector type."] pub fn dynamic_to_fixed (ty : Type) -> Type { match ty { I8X8XN => I8X8 , I8X16XN => I8X16 , I16X4XN => I16X4 , I16X8XN => I16X8 , I32X2XN => I32X2 , I32X4XN => I32X4 , I64X2XN => I64X2 , F32X4XN => F32X4 , F64X2XN => F64X2 , _ => unreachable ! ("unhandled type: {}" , ty) , } }
};
}
