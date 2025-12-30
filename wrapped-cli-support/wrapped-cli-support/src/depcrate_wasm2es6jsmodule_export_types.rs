// Generated macro for module_export_types (function)
macro_rules! Depcrate_wasm2es6jsmodule_export_types {
() => {
// Module: crate::wasm2es6js
// Provides: {"module_export_types"}
// Dependencies: {}
# [doc = " Iterates over all the exports in a module and generates TypeScript types. All"] # [doc = " name-type pairs are passed to the `export` function."] fn module_export_types (module : & Module , mut export : impl FnMut (& str , & str)) { for entry in module . exports . iter () { match entry . item { walrus :: ExportItem :: Function (id) => { let func = module . funcs . get (id) ; let ty = module . types . get (func . ty ()) ; let ts_type = function_type_to_ts (ty , args_are_optional (& entry . name)) ; export (& entry . name , & ts_type) ; } walrus :: ExportItem :: Memory (_) => export (& entry . name , "WebAssembly.Memory") , walrus :: ExportItem :: Table (_) => export (& entry . name , "WebAssembly.Table") , walrus :: ExportItem :: Global (_) => continue , walrus :: ExportItem :: Tag (_) => export (& entry . name , "WebAssembly.Tag") , } ; } }
};
}
