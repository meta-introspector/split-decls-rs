// Generated macro for impl_347 (impl)
macro_rules! Depcrate_wit_standardimpl_347 {
() => {
// Module: crate::wit::standard
// Provides: {"impl_347"}
// Dependencies: {}
impl AdapterType { pub fn from_wasm (wasm : walrus :: ValType) -> Option < AdapterType > { Some (match wasm { walrus :: ValType :: I32 => AdapterType :: I32 , walrus :: ValType :: I64 => AdapterType :: I64 , walrus :: ValType :: F32 => AdapterType :: F32 , walrus :: ValType :: F64 => AdapterType :: F64 , walrus :: ValType :: Ref (RefType :: Externref) => AdapterType :: Externref , walrus :: ValType :: Ref (_) | walrus :: ValType :: V128 => return None , }) } pub fn to_wasm (& self) -> Option < walrus :: ValType > { Some (match self { AdapterType :: I32 => walrus :: ValType :: I32 , AdapterType :: I64 => walrus :: ValType :: I64 , AdapterType :: F32 => walrus :: ValType :: F32 , AdapterType :: F64 => walrus :: ValType :: F64 , AdapterType :: Enum (_) => walrus :: ValType :: I32 , AdapterType :: Externref | AdapterType :: NamedExternref (_) => { walrus :: ValType :: Ref (RefType :: Externref) } _ => return None , }) } pub fn option (self) -> AdapterType { AdapterType :: Option (Box :: new (self)) } }
};
}
