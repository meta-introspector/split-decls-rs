// Generated macro for list_element_info (function)
macro_rules! Depcrate_functionlist_element_info {
() => {
// Module: crate::function
// Provides: {"list_element_info"}
// Dependencies: {}
fn list_element_info (ty : & Type) -> (usize , & 'static str) { match ty { Type :: S8 => (1 , "sbyte") , Type :: S16 => (2 , "short") , Type :: S32 => (4 , "int") , Type :: S64 => (8 , "long") , Type :: U8 => (1 , "byte") , Type :: U16 => (2 , "ushort") , Type :: U32 => (4 , "uint") , Type :: U64 => (8 , "ulong") , Type :: F32 => (4 , "float") , Type :: F64 => (8 , "double") , _ => unreachable ! () , } }
};
}
