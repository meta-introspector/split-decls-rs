// Generated macro for shared_import_static (function)
macro_rules! Depcrate_encodeshared_import_static {
() => {
// Module: crate::encode
// Provides: {"shared_import_static"}
// Dependencies: {}
fn shared_import_static < 'a > (i : & 'a ast :: ImportStatic , intern : & 'a Interner) -> ImportStatic < 'a > { ImportStatic { name : & i . js_name , shim : intern . intern (& i . shim) , } }
};
}
