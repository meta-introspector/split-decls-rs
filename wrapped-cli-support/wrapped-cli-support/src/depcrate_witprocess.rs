// Generated macro for process (function)
macro_rules! Depcrate_witprocess {
() => {
// Module: crate::wit
// Provides: {"process"}
// Dependencies: {}
pub fn process (bindgen : & mut Bindgen , module : & mut Module , programs : Vec < decode :: Program > , thread_count : Option < ThreadCount > ,) -> Result < (NonstandardWitSectionId , WasmBindgenAuxId) , Error > { let mut cx = Context { adapters : Default :: default () , aux : Default :: default () , function_exports : Default :: default () , function_imports : Default :: default () , vendor_prefixes : Default :: default () , descriptors : Default :: default () , unique_crate_identifier : "" , memory : wasm_conventions :: get_memory (module) . ok () , module , start_found : false , externref_enabled : bindgen . externref , thread_count , support_start : bindgen . emit_start , linked_modules : bindgen . split_linked_modules , } ; cx . init () ? ; for program in programs { cx . program (program) ? ; } if ! cx . start_found { cx . discover_main () ? ; } cx . verify () ? ; cx . unexport_intrinsics () ; let adapters = cx . module . customs . add (cx . adapters) ; let aux = cx . module . customs . add (cx . aux) ; Ok ((adapters , aux)) }
};
}
