// Generated macro for find (function)
macro_rules! Depcratefind {
() => {
// Module: crate
// Provides: {"find"}
// Dependencies: {}
fn find < P : AsRef < Path > > (path : P) -> Vec < Crate > { let mut crates = vec ! [] ; if let Ok (files) = std :: fs :: read_dir (path) { for file in files . filter_map (| file | file . ok ()) { if let Ok (file_type) = file . file_type () { if file_type . is_dir () { crates . append (& mut find (file . path ())) ; } else if file . file_name () == "Cargo.toml" { let text = std :: fs :: read_to_string (file . path ()) . expect ("Cargo.toml") ; let mut entry : Crate = toml :: from_str (& text) . expect ("toml") ; entry . path = Some (file . path ()) ; crates . push (entry) ; } } } } crates }
};
}
