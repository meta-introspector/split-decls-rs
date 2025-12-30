// Generated macro for interpret (function)
macro_rules! Depcrate_interpreter_smoke_testsinterpret {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"interpret"}
// Dependencies: {}
fn interpret (wat : & str , name : & str , result : & [u32]) { let wasm = wat :: parse_str (wat) . unwrap () ; let module = ModuleConfig :: new () . generate_producers_section (false) . parse (& wasm) . unwrap () ; let mut i = Interpreter :: new (& module) . unwrap () ; let id = module . exports . iter () . filter (| e | e . name == name) . find_map (| e | match e . item { walrus :: ExportItem :: Function (f) => Some (f) , _ => None , }) . unwrap () ; assert_eq ! (i . interpret_descriptor (id , & module) , result) ; }
};
}
