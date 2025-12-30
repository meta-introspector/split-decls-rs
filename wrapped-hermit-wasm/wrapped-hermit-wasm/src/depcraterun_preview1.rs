// Generated macro for run_preview1 (function)
macro_rules! Depcraterun_preview1 {
() => {
// Module: crate
// Provides: {"run_preview1"}
// Dependencies: {}
pub fn run_preview1 (module_bytes : & [u8] , config : & wasmtime :: Config , # [allow (unused_variables)] module_and_args : & 'static [OsString] ,) -> Result < () > { let engine = Engine :: new (config) ? ; debug ! ("Wasmtime engine is configured as followed: {config:?}") ; debug ! ("Create Module") ; let now = Instant :: now () ; let module = Module :: new (& engine , module_bytes) ? ; let elapsed = now . elapsed () ; debug ! ("Time to create module: {} msec" , elapsed . as_millis ()) ; debug ! ("Create Linker") ; # [allow (unused_mut)] let mut linker = Linker :: new (& engine) ; { let mut imports = module . imports () ; if imports . any (| i | i . module () == "wasi_snapshot_preview1") { preview1 :: init (& mut linker , module_and_args) ? ; } } let mut store = Store :: new (& engine , 4) ; let instance = linker . instantiate (& mut store , & module) ? ; debug ! ("Try to find symbol _start") ; let func = instance . get_func (& mut store , "_start") . unwrap () ; let ty = func . ty (& store) ; if ty . params () . len () > 0 { panic ! ("Currently, _start should not receive arguments") ; } let mut results = vec ! [Val :: null_func_ref () ; ty . results () . len ()] ; let values = Vec :: new () ; let invoke_res = func . call (& mut store , & values , & mut results) . with_context (| | "failed to invoke command default" . to_string ()) ; debug ! ("Return value of entry point: {invoke_res:?}") ; invoke_res }
};
}
