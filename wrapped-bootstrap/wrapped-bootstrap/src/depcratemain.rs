// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let pest = Path :: new (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/../meta/src/grammar.pest")) ; let pest_ref = pest . to_string_lossy () ; let normalized_path = pest_ref . strip_prefix (r#"\\?\"#) . unwrap_or_else (| | & pest_ref) ; let pest = Path :: new (& normalized_path) ; let rs : PathBuf = if should_bootstrap_in_src () { Path :: new (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/../meta/src/grammar.rs")) . to_owned () } else { let path = env :: args () . nth (1) . expect ("path to grammar.rs") ; PathBuf :: from (path) } ; let derived = { let path = pest . to_string_lossy () ; let pest = quote ! { # [grammar = # path] pub struct PestParser ; } ; derive_parser (pest , false) } ; let mut file = File :: create (rs) . unwrap () ; writeln ! (file , "pub struct PestParser;\n{}" , derived ,) . unwrap () ; }
};
}
