// Generated macro for impl_175 (impl)
macro_rules! Depcrate_valueimpl_175 {
() => {
// Module: crate::value
// Provides: {"impl_175"}
// Dependencies: {}
impl Value { pub fn ty (& self) -> Type { match self { Self :: Bool (..) => Type :: Bool , Self :: U8 (..) => Type :: U8 , Self :: I8 (..) => Type :: I8 , Self :: U16 (..) => Type :: U16 , Self :: I16 (..) => Type :: I16 , Self :: U32 (..) => Type :: U32 , Self :: I32 (..) => Type :: I32 , Self :: U64 (..) => Type :: U64 , Self :: I64 (..) => Type :: I64 , Self :: F32 (..) => Type :: F32 , Self :: F64 (..) => Type :: F64 , Self :: Utf8 (..) => Type :: String , Self :: Utf16 (..) => Type :: String , Self :: AttributeEnum (..) => Type :: AttributeEnum , } } }
};
}
