// Generated macro for get_or_insert_start_builder (function)
macro_rules! Depcrate_wasm_conventionsget_or_insert_start_builder {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_or_insert_start_builder"}
// Dependencies: {}
pub fn get_or_insert_start_builder (module : & mut Module) -> & mut FunctionBuilder { let prev_start = get_start (module) ; let id = match prev_start { Ok (id) => id , Err (prev_start) => { let mut builder = FunctionBuilder :: new (& mut module . types , & [] , & []) ; if let Some (prev_start) = prev_start { builder . func_body () . call (prev_start) ; } let id = builder . finish (Vec :: new () , & mut module . funcs) ; module . start = Some (id) ; id } } ; module . funcs . get_mut (id) . kind . unwrap_local_mut () . builder_mut () }
};
}
