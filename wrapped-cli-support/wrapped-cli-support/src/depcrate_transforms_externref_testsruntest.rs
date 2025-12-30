// Generated macro for runtest (function)
macro_rules! Depcrate_transforms_externref_testsruntest {
() => {
// Module: crate::transforms::externref::tests
// Provides: {"runtest"}
// Dependencies: {}
fn runtest (test : & Test) -> Result < String > { let wasm = wat :: parse_file (& test . file) ? ; let mut walrus = ModuleConfig :: new () . generate_producers_section (false) . parse (& wasm) ? ; let mut cx = super :: Context :: new (& mut walrus) ? ; for directive in test . directives . iter () { match & directive . kind { DirectiveKind :: Export (name) => { let export = walrus . exports . iter () . find (| e | e . name == * name) . ok_or_else (| | anyhow ! ("failed to find export")) ? ; cx . export_xform (export . id () , & directive . args , directive . ret_externref) ; } DirectiveKind :: Import (module , field) => { let import = walrus . imports . iter () . find (| e | e . module == * module && e . name == * field) . ok_or_else (| | anyhow ! ("failed to find export")) ? ; cx . import_xform (import . id () , & directive . args , directive . ret_externref) ; } } } cx . run (& mut walrus) ? ; walrus :: passes :: gc :: run (& mut walrus) ; let printed = wasmprinter :: print_bytes (walrus . emit_wasm ()) ? ; Ok (printed) }
};
}
