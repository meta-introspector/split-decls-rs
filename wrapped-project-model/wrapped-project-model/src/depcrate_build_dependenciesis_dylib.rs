// Generated macro for is_dylib (function)
macro_rules! Depcrate_build_dependenciesis_dylib {
() => {
// Module: crate::build_dependencies
// Provides: {"is_dylib"}
// Dependencies: {}
fn is_dylib (path : & Utf8Path) -> bool { match path . extension () . map (| e | e . to_owned () . to_lowercase ()) { None => false , Some (ext) => matches ! (ext . as_str () , "dll" | "dylib" | "so") , } }
};
}
