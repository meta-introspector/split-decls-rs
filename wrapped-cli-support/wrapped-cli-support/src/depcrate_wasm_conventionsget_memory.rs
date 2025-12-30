// Generated macro for get_memory (function)
macro_rules! Depcrate_wasm_conventionsget_memory {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_memory"}
// Dependencies: {}
# [doc = " Get a Wasm module's canonical linear memory."] pub fn get_memory (module : & Module) -> Result < MemoryId > { let mut memories = module . memories . iter () . map (| m | m . id ()) ; let memory = memories . next () ; if memories . next () . is_some () { bail ! ("expected a single memory, found multiple; multiple memories \
             currently not supported") ; } memory . ok_or_else (| | { anyhow ! ("module does not have a memory; must have a memory \
             to transform return pointers into Wasm multi-value") }) }
};
}
