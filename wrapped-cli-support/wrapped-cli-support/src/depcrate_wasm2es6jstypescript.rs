// Generated macro for typescript (function)
macro_rules! Depcrate_wasm2es6jstypescript {
() => {
// Module: crate::wasm2es6js
// Provides: {"typescript"}
// Dependencies: {}
pub fn typescript (module : & Module) -> Result < String , Error > { let mut exports = "/* tslint:disable */\n/* eslint-disable */\n" . to_string () ; module_export_types (module , | name , ty | { writeln ! (exports , "export const {name}: {ty};") . unwrap () ; }) ; Ok (exports) }
};
}
