// Generated macro for wasm_import_module_map (function)
macro_rules! Depcrate_back_symbol_exportwasm_import_module_map {
() => {
// Module: crate::back::symbol_export
// Provides: {"wasm_import_module_map"}
// Dependencies: {}
fn wasm_import_module_map (tcx : TyCtxt < '_ > , cnum : CrateNum) -> DefIdMap < String > { let native_libs = tcx . native_libraries (cnum) ; let def_id_to_native_lib = native_libs . iter () . filter_map (| lib | lib . foreign_module . map (| id | (id , lib))) . collect :: < DefIdMap < _ > > () ; let mut ret = DefIdMap :: default () ; for (def_id , lib) in tcx . foreign_modules (cnum) . iter () { let module = def_id_to_native_lib . get (def_id) . and_then (| s | s . wasm_import_module ()) ; let Some (module) = module else { continue } ; ret . extend (lib . foreign_items . iter () . map (| id | { assert_eq ! (id . krate , cnum) ; (* id , module . to_string ()) })) ; } ret }
};
}
