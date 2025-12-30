// Generated macro for impl_40 (impl)
macro_rules! Depcrate_descriptorimpl_40 {
() => {
// Module: crate::descriptor
// Provides: {"impl_40"}
// Dependencies: {}
impl VectorKind { pub fn js_ty (& self) -> String { match * self { VectorKind :: String => "string" . to_string () , VectorKind :: I8 => "Int8Array" . to_string () , VectorKind :: U8 => "Uint8Array" . to_string () , VectorKind :: ClampedU8 => "Uint8ClampedArray" . to_string () , VectorKind :: I16 => "Int16Array" . to_string () , VectorKind :: U16 => "Uint16Array" . to_string () , VectorKind :: I32 => "Int32Array" . to_string () , VectorKind :: U32 => "Uint32Array" . to_string () , VectorKind :: I64 => "BigInt64Array" . to_string () , VectorKind :: U64 => "BigUint64Array" . to_string () , VectorKind :: F32 => "Float32Array" . to_string () , VectorKind :: F64 => "Float64Array" . to_string () , VectorKind :: Externref => "any[]" . to_string () , VectorKind :: NamedExternref (ref name) => { if is_valid_ident (name . as_str ()) { format ! ("{name}[]") } else { format ! ("({name})[]") } } } } pub fn size (& self) -> usize { match * self { VectorKind :: String => 1 , VectorKind :: I8 => 1 , VectorKind :: U8 => 1 , VectorKind :: ClampedU8 => 1 , VectorKind :: I16 => 2 , VectorKind :: U16 => 2 , VectorKind :: I32 => 4 , VectorKind :: U32 => 4 , VectorKind :: I64 => 8 , VectorKind :: U64 => 8 , VectorKind :: F32 => 4 , VectorKind :: F64 => 8 , VectorKind :: Externref => 4 , VectorKind :: NamedExternref (_) => 4 , } } }
};
}
