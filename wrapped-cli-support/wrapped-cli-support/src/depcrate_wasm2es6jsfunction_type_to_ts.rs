// Generated macro for function_type_to_ts (function)
macro_rules! Depcrate_wasm2es6jsfunction_type_to_ts {
() => {
// Module: crate::wasm2es6js
// Provides: {"function_type_to_ts"}
// Dependencies: {}
fn function_type_to_ts (function : & walrus :: Type , all_args_optional : bool) -> String { let mut out = String :: new () ; out . push ('(') ; for (i , arg_type) in function . params () . iter () . enumerate () { if i > 0 { out . push_str (", ") ; } push_index_identifier (i , & mut out) ; if all_args_optional { out . push ('?') ; } out . push_str (": ") ; out . push_str (val_type_to_ts (* arg_type)) ; } out . push (')') ; out . push_str (" => ") ; let results = function . results () ; match results . len () { 0 => out . push_str ("void") , 1 => out . push_str (val_type_to_ts (results [0])) , _ => { out . push ('[') ; for (i , result) in results . iter () . enumerate () { if i > 0 { out . push_str (", ") ; } out . push_str (val_type_to_ts (* result)) ; } out . push (']') ; } } out }
};
}
