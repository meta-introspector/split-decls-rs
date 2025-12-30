// Generated macro for val_type_to_ts (function)
macro_rules! Depcrate_wasm2es6jsval_type_to_ts {
() => {
// Module: crate::wasm2es6js
// Provides: {"val_type_to_ts"}
// Dependencies: {}
fn val_type_to_ts (ty : walrus :: ValType) -> & 'static str { match ty { walrus :: ValType :: I32 | walrus :: ValType :: F32 | walrus :: ValType :: F64 => "number" , walrus :: ValType :: I64 => "bigint" , walrus :: ValType :: Ref (_) => "any" , walrus :: ValType :: V128 => "any" , } }
};
}
