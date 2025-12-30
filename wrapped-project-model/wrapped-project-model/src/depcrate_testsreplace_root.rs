// Generated macro for replace_root (function)
macro_rules! Depcrate_testsreplace_root {
() => {
// Module: crate::tests
// Provides: {"replace_root"}
// Dependencies: {}
fn replace_root (s : & mut String , direction : bool) { if direction { let root = if cfg ! (windows) { r#"C:\\ROOT\"# } else { "/ROOT/" } ; * s = s . replace ("$ROOT$" , root) } else { let root = if cfg ! (windows) { r#"C:\\\\ROOT\\"# } else { "/ROOT/" } ; * s = s . replace (root , "$ROOT$") } }
};
}
