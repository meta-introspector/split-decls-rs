// Generated macro for rmain (function)
macro_rules! Depcrate_wasm_bindgenrmain {
() => {
// Module: crate::wasm_bindgen
// Provides: {"rmain"}
// Dependencies: {}
fn rmain (args : & Args) -> Result < () , Error > { let mut b = Bindgen :: new () ; match & args . target { Target :: Bundler => b . bundler (true) ? , Target :: Web => b . web (true) ? , Target :: NoModules => b . no_modules (true) ? , Target :: Nodejs => b . nodejs (true) ? , Target :: Deno => b . deno (true) ? , Target :: ExperimentalNodejsModule => b . nodejs_module (true) ? , Target :: Module => b . module (true) ? , } ; # [allow (deprecated)] b . input_path (& args . input) . nodejs (args . nodejs) ? . web (args . web) ? . browser (args . browser) ? . no_modules (args . no_modules) ? . debug (args . debug) . demangle (! args . no_demangle) . keep_lld_exports (args . keep_lld_exports) . keep_debug (args . keep_debug) . remove_name_section (args . remove_name_section) . remove_producers_section (args . remove_producers_section) . typescript (! args . no_typescript) . omit_imports (args . omit_imports) . omit_default_module_path (args . omit_default_module_path) . split_linked_modules (args . split_linked_modules) . reference_types (args . reference_types) . reset_state_function (args . generate_reset_state) ; if let Some (ref name) = args . no_modules_global { b . no_modules_global (name) ? ; } if let Some (ref name) = args . out_name { b . out_name (name) ; } if let Some (mode) = & args . encode_into { let mode = match mode . as_str () { "test" => EncodeInto :: Test , "always" => EncodeInto :: Always , "never" => EncodeInto :: Never , _ => unreachable ! () , } ; b . encode_into (mode) ; } b . generate (& args . out_dir) }
};
}
