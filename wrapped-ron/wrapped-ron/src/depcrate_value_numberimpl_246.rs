// Generated macro for impl_246 (impl)
macro_rules! Depcrate_value_numberimpl_246 {
() => {
// Module: crate::value::number
// Provides: {"impl_246"}
// Dependencies: {}
impl Serialize for Number { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { Self :: I8 (v) => serializer . serialize_i8 (* v) , Self :: I16 (v) => serializer . serialize_i16 (* v) , Self :: I32 (v) => serializer . serialize_i32 (* v) , Self :: I64 (v) => serializer . serialize_i64 (* v) , # [cfg (feature = "integer128")] Self :: I128 (v) => serializer . serialize_i128 (* v) , Self :: U8 (v) => serializer . serialize_u8 (* v) , Self :: U16 (v) => serializer . serialize_u16 (* v) , Self :: U32 (v) => serializer . serialize_u32 (* v) , Self :: U64 (v) => serializer . serialize_u64 (* v) , # [cfg (feature = "integer128")] Self :: U128 (v) => serializer . serialize_u128 (* v) , Self :: F32 (v) => serializer . serialize_f32 (v . get ()) , Self :: F64 (v) => serializer . serialize_f64 (v . get ()) , # [cfg (not (doc))] Self :: __NonExhaustive (never) => never . never () , } } }
};
}
