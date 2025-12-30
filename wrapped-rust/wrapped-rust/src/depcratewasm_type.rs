// Generated macro for wasm_type (function)
macro_rules! Depcratewasm_type {
() => {
// Module: crate
// Provides: {"wasm_type"}
// Dependencies: {}
fn wasm_type (ty : WasmType) -> & 'static str { match ty { WasmType :: I32 => "i32" , WasmType :: I64 => "i64" , WasmType :: F32 => "f32" , WasmType :: F64 => "f64" , WasmType :: Pointer => "*mut u8" , WasmType :: Length => "usize" , WasmType :: PointerOrI64 => "::core::mem::MaybeUninit::<u64>" , } }
};
}
