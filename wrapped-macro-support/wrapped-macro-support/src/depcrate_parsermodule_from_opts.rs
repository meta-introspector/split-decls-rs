// Generated macro for module_from_opts (function)
macro_rules! Depcrate_parsermodule_from_opts {
() => {
// Module: crate::parser
// Provides: {"module_from_opts"}
// Dependencies: {}
pub fn module_from_opts (program : & mut ast :: Program , opts : & BindgenAttrs ,) -> Result < Option < ast :: ImportModule > , Diagnostic > { if let Some (path) = opts . wasm_bindgen () { program . wasm_bindgen = path . clone () ; } if let Some (path) = opts . js_sys () { program . js_sys = path . clone () ; } if let Some (path) = opts . wasm_bindgen_futures () { program . wasm_bindgen_futures = path . clone () ; } let mut errors = Vec :: new () ; let module = if let Some ((name , span)) = opts . module () { if opts . inline_js () . is_some () { let msg = "cannot specify both `module` and `inline_js`" ; errors . push (Diagnostic :: span_error (span , msg)) ; } if opts . raw_module () . is_some () { let msg = "cannot specify both `module` and `raw_module`" ; errors . push (Diagnostic :: span_error (span , msg)) ; } Some (ast :: ImportModule :: Named (name . to_string () , span)) } else if let Some ((name , span)) = opts . raw_module () { if opts . inline_js () . is_some () { let msg = "cannot specify both `raw_module` and `inline_js`" ; errors . push (Diagnostic :: span_error (span , msg)) ; } Some (ast :: ImportModule :: RawNamed (name . to_string () , span)) } else if let Some ((js , _span)) = opts . inline_js () { let i = program . inline_js . len () ; program . inline_js . push (js . to_string ()) ; Some (ast :: ImportModule :: Inline (i)) } else { None } ; Diagnostic :: from_vec (errors) ? ; Ok (module) }
};
}
