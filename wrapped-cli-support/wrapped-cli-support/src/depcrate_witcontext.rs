// Generated macro for Context (struct)
macro_rules! Depcrate_witContext {
() => {
// Module: crate::wit
// Provides: {"Context"}
// Dependencies: {}
struct Context < 'a > { start_found : bool , module : & 'a mut Module , adapters : NonstandardWitSection , aux : WasmBindgenAux , # [doc = " All of the Wasm module's exported functions."] function_exports : HashMap < String , (ExportId , FunctionId) > , # [doc = " All of the Wasm module's imported functions."] function_imports : HashMap < String , (ImportId , FunctionId) > , memory : Option < MemoryId > , vendor_prefixes : HashMap < String , Vec < String > > , unique_crate_identifier : & 'a str , descriptors : HashMap < String , Descriptor > , externref_enabled : bool , thread_count : Option < ThreadCount > , support_start : bool , linked_modules : bool , }
};
}
