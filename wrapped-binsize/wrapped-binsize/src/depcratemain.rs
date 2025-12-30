// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args : Vec < String > = env :: args () . collect () ; if args . len () != 3 { eprintln ! ("Usage: cargo run --package icu_benchmark_binsize -- <PATH> <wasm | gz | file>") ; process :: exit (1) ; } let path = & args [1] ; let path_type = & args [2] ; if path_type != "wasm" && path_type != "gz" && path_type != "file" { eprintln ! ("Invalid path type {path_type}, use wasm or gz") ; process :: exit (1) ; } if path_type == "file" { let rc = any_file_size (path) ; if ! rc . unwrap () { eprintln ! ("File {path} not found") ; } } else { let count = wasm_filesize (path , path_type) ; if count . unwrap () == 0 { eprintln ! ("No wasm binaries found in directory {path}") ; process :: exit (1) ; } } }
};
}
