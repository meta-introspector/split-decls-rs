// Generated macro for Type (struct)
macro_rules! Depcrate_ir_typesType {
() => {
// Module: crate::ir::types
// Provides: {"Type"}
// Dependencies: {}
# [doc = " The type of an SSA value."] # [doc = ""] # [doc = " The `INVALID` type isn't a real type, and is used as a placeholder in the IR where a type"] # [doc = " field is present put no type is needed, such as the controlling type variable for a"] # [doc = " non-polymorphic instruction."] # [doc = ""] # [doc = " Basic integer types: `I8`, `I16`, `I32`, `I64`, and `I128`. These types are sign-agnostic."] # [doc = ""] # [doc = " Basic floating point types: `F16`, `F32`, `F64`, and `F128`. IEEE half, single, double, and quadruple precision."] # [doc = ""] # [doc = " SIMD vector types have power-of-two lanes, up to 256. Lanes can be any int/float type."] # [doc = ""] # [doc = " Note that this is encoded in a `u16` currently for extensibility,"] # [doc = " but allows only 14 bits to be used due to some bitpacking tricks"] # [doc = " in the CLIF data structures."] # [derive (Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Type (u16) ;
};
}
