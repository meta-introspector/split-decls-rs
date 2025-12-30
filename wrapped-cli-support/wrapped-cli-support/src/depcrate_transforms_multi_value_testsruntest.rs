// Generated macro for runtest (function)
macro_rules! Depcrate_transforms_multi_value_testsruntest {
() => {
// Module: crate::transforms::multi_value::tests
// Provides: {"runtest"}
// Dependencies: {}
fn runtest (test : & Test) -> Result < String > { let wasm = wat :: parse_file (& test . file) ? ; let mut walrus = ModuleConfig :: new () . generate_producers_section (false) . parse (& wasm) ? ; let mut exports = Vec :: new () ; let mut xforms = Vec :: new () ; for directive in test . directives . iter () { let export = walrus . exports . iter () . find (| e | e . name == directive . name) . unwrap () ; let id = match export . item { walrus :: ExportItem :: Function (id) => id , _ => panic ! ("must be function export") , } ; exports . push (export . id ()) ; xforms . push ((id , 0 , directive . tys . clone ())) ; } let memory = walrus . memories . iter () . next () . unwrap () . id () ; let stack_pointer = walrus . globals . iter () . next () . unwrap () . id () ; let ret = super :: run (& mut walrus , memory , stack_pointer , & xforms) ? ; for (export , id) in exports . into_iter () . zip (ret) { walrus . exports . get_mut (export) . item = walrus :: ExportItem :: Function (id) ; } walrus :: passes :: gc :: run (& mut walrus) ; let printed = wasmprinter :: print_bytes (walrus . emit_wasm ()) ? ; Ok (printed) }
};
}
