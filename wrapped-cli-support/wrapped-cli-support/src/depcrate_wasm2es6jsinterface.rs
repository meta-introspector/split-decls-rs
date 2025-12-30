// Generated macro for interface (function)
macro_rules! Depcrate_wasm2es6jsinterface {
() => {
// Module: crate::wasm2es6js
// Provides: {"interface"}
// Dependencies: {}
pub fn interface (module : & Module) -> Result < String , Error > { let mut exports = String :: new () ; module_export_types (module , | name , ty | { if name . contains (':') { writeln ! (exports , "  readonly {name:?}: {ty};") . unwrap () ; } else { writeln ! (exports , "  readonly {name}: {ty};") . unwrap () ; } }) ; Ok (exports) }
};
}
