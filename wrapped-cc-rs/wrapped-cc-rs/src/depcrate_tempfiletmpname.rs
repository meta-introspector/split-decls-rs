// Generated macro for tmpname (function)
macro_rules! Depcrate_tempfiletmpname {
() => {
// Module: crate::tempfile
// Provides: {"tmpname"}
// Dependencies: {}
fn tmpname (suffix : & str) -> String { format ! ("{}{}" , rand () , suffix) }
};
}
