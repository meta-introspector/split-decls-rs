// Generated macro for run (function)
macro_rules! Depcrate_multivaluerun {
() => {
// Module: crate::multivalue
// Provides: {"run"}
// Dependencies: {}
pub fn run (module : & mut Module) -> Result < () , Error > { let mut adapters = module . customs . delete_typed :: < NonstandardWitSection > () . unwrap () ; let mut to_xform = Vec :: new () ; let mut exports = Vec :: new () ; for adapter in adapters . adapters . values_mut () { extract_xform (module , adapter , & mut to_xform , & mut exports) ; } if to_xform . is_empty () { module . customs . add (* adapters) ; return Ok (()) ; } let stack_pointer = module . customs . get_typed :: < WasmBindgenAux > () . expect ("aux section should be present") . stack_pointer . ok_or_else (| | anyhow ! ("failed to find stack pointer in Wasm module")) ? ; let memory = wasm_conventions :: get_memory (module) ? ; let wrappers = multi_value_xform :: run (module , memory , stack_pointer , & to_xform) ? ; for (export , id) in exports . into_iter () . zip (wrappers) { module . exports . get_mut (export) . item = id . into () ; } module . customs . add (* adapters) ; Ok (()) }
};
}
