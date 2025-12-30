// Generated macro for wasm_type (function)
macro_rules! Depcrate_world_generatorwasm_type {
() => {
// Module: crate::world_generator
// Provides: {"wasm_type"}
// Dependencies: {}
pub fn wasm_type (ty : WasmType) -> & 'static str { match ty { WasmType :: I32 => "int" , WasmType :: I64 => "long" , WasmType :: F32 => "float" , WasmType :: F64 => "double" , WasmType :: Pointer => "nint" , WasmType :: PointerOrI64 => "long" , WasmType :: Length => "int" , } }
};
}
