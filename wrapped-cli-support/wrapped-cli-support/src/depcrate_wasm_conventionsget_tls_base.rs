// Generated macro for get_tls_base (function)
macro_rules! Depcrate_wasm_conventionsget_tls_base {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_tls_base"}
// Dependencies: {}
# [doc = " Get the `__tls_base`."] pub fn get_tls_base (module : & Module) -> Option < GlobalId > { let candidates = module . exports . iter () . filter (| ex | ex . name == "__tls_base") . filter_map (| ex | match ex . item { walrus :: ExportItem :: Global (id) => Some (id) , _ => None , }) . filter (| id | { let global = module . globals . get (* id) ; global . ty == ValType :: I32 }) . collect :: < Vec < _ > > () ; match candidates . len () { 1 => Some (candidates [0]) , _ => None , } }
};
}
