// Generated macro for Interpreter (struct)
macro_rules! Depcrate_interpreterInterpreter {
() => {
// Module: crate::interpreter
// Provides: {"Interpreter"}
// Dependencies: {}
# [doc = " A ready-to-go interpreter of a Wasm module."] # [doc = ""] # [doc = " An interpreter currently represents effectively cached state. It is reused"] # [doc = " between calls to `interpret` and is precomputed from a `Module`. It houses"] # [doc = " state like the Wasm stack, Wasm memory, etc."] # [derive (Default)] pub struct Interpreter { describe_id : Option < FunctionId > , describe_cast_id : Option < FunctionId > , sp : i32 , mem : Vec < i32 > , scratch : Vec < i32 > , descriptor : Vec < u32 > , # [doc = " The `__wbindgen_skip_interpret_calls`'s id."] skip_interpret : Option < ExportId > , # [doc = " Some functions that need to skip interpret, such as `__wasm_call_ctors`"] # [doc = " and `__wasm_call_dtors`."] skip_calls : HashSet < FunctionId > , }
};
}
