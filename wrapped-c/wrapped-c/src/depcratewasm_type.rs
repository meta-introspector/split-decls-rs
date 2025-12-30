// Generated macro for wasm_type (function)
macro_rules! Depcratewasm_type {
() => {
// Module: crate
// Provides: {"wasm_type"}
// Dependencies: {}
pub fn wasm_type (ty : WasmType) -> & 'static str { match ty { WasmType :: I32 => "int32_t" , WasmType :: I64 => "int64_t" , WasmType :: F32 => "float" , WasmType :: F64 => "double" , WasmType :: Pointer => "uint8_t *" , WasmType :: PointerOrI64 => "int64_t" , WasmType :: Length => "size_t" , } }
};
}
