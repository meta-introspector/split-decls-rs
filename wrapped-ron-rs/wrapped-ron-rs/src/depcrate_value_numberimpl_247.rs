// Generated macro for impl_247 (impl)
macro_rules! Depcrate_value_numberimpl_247 {
() => {
// Module: crate::value::number
// Provides: {"impl_247"}
// Dependencies: {}
impl Number { pub fn visit < 'de , V : Visitor < 'de > , E : serde :: de :: Error > (& self , visitor : V ,) -> Result < V :: Value , E > { match self { Self :: I8 (v) => visitor . visit_i8 (* v) , Self :: I16 (v) => visitor . visit_i16 (* v) , Self :: I32 (v) => visitor . visit_i32 (* v) , Self :: I64 (v) => visitor . visit_i64 (* v) , # [cfg (feature = "integer128")] Self :: I128 (v) => visitor . visit_i128 (* v) , Self :: U8 (v) => visitor . visit_u8 (* v) , Self :: U16 (v) => visitor . visit_u16 (* v) , Self :: U32 (v) => visitor . visit_u32 (* v) , Self :: U64 (v) => visitor . visit_u64 (* v) , # [cfg (feature = "integer128")] Self :: U128 (v) => visitor . visit_u128 (* v) , Self :: F32 (v) => visitor . visit_f32 (v . get ()) , Self :: F64 (v) => visitor . visit_f64 (v . get ()) , # [cfg (not (doc))] Self :: __NonExhaustive (never) => never . never () , } } }
};
}
