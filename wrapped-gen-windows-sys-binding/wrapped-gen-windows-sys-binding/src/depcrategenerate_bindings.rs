// Generated macro for generate_bindings (function)
macro_rules! Depcrategenerate_bindings {
() => {
// Module: crate
// Provides: {"generate_bindings"}
// Dependencies: {}
pub fn generate_bindings () { let filter : String = format ! ("{MANIFEST_DIR}/windows_sys.list") ; let temp_file = tempfile :: Builder :: new () . suffix (".rs") . tempfile () . expect ("failed to create temp file") ; windows_bindgen :: bindgen (["--flat" , "--sys" , "--no-deps" , "--out" , temp_file . path () . to_str () . unwrap () , "--filter" , "--etc" , & filter ,]) . unwrap () ; let bindings = fs :: read_to_string (temp_file . path ()) . expect ("failed to read temp windows_sys.rs") ; let mut f : BufWriter < fs :: File > = fs :: File :: create (format ! ("{MANIFEST_DIR}/../../find-msvc-tools/src/windows_sys.rs")) . map (BufWriter :: new) . expect ("failed to create windows_sys.rs") ; write ! (& mut f , "{PRELUDE}\n{bindings}\n") . unwrap () ; let mut dll_names : Vec < & str > = Regex :: new (r#"link!\("(.*)\.dll""#) . unwrap () . captures_iter (& bindings) . map (| caps | caps . extract () . 1) . map (| [dll_name] | dll_name) . filter (| dll_name | * dll_name != "kernel32") . collect () ; if ! dll_names . is_empty () { dll_names . sort_unstable () ; dll_names . dedup () ; for dll_name in dll_names { write ! (& mut f , r#"#[link(name = "{dll_name}")]"#) . unwrap () ; f . write_all ("\n" . as_bytes ()) . unwrap () ; } f . write_all (r#"extern "C" {}"# . as_bytes ()) . unwrap () ; f . write_all ("\n" . as_bytes ()) . unwrap () ; } f . write_all (r#"use super::windows_link;"# . as_bytes ()) . unwrap () ; f . write_all ("\n" . as_bytes ()) . unwrap () ; f . into_inner () . unwrap () . sync_all () . unwrap () ; }
};
}
