// Generated macro for is_dir (function)
macro_rules! Depcrate_fsis_dir {
() => {
// Module: crate::fs
// Provides: {"is_dir"}
// Dependencies: {}
pub fn is_dir < P > (path : & P) -> bool where P : AsRef < Path > , { let path : & Path = path . as_ref () ; path . is_dir () }
};
}
