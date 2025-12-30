// Generated macro for wasm_type (function)
macro_rules! Depcratewasm_type {
() => {
// Module: crate
// Provides: {"wasm_type"}
// Dependencies: {}
fn wasm_type (ty : WasmType) -> & 'static str { match ty { WasmType :: I32 => "Int" , WasmType :: I64 => "Int64" , WasmType :: F32 => "Float" , WasmType :: F64 => "Double" , WasmType :: Pointer => "Int" , WasmType :: PointerOrI64 => "Int64" , WasmType :: Length => "Int" , } }
};
}
