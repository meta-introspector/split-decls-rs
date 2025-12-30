// Generated macro for rmain (function)
macro_rules! Depcrate_wasm2es6jsrmain {
() => {
// Module: crate::wasm2es6js
// Provides: {"rmain"}
// Dependencies: {}
fn rmain (args : Args) -> anyhow :: Result < () > { let wasm = fs :: read (& args . input) . with_context (| | format ! ("failed to read `{}`" , args . input . display ())) ? ; let object = wasm_bindgen_cli_support :: wasm2es6js :: Config :: new () . base64 (args . base64) . fetch (args . fetch . clone ()) . generate (& wasm) ? ; if args . typescript { let ts = object . typescript () ? ; write (& args , "d.ts" , ts . as_bytes () , false) ? ; } let (js , wasm) = object . js_and_wasm () ? ; write (& args , "js" , js . as_bytes () , false) ? ; if let Some (wasm) = wasm { write (& args , "wasm" , & wasm , false) ? ; } Ok (()) }
};
}
