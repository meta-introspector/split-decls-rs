// Generated macro for shared_import_string (function)
macro_rules! Depcrate_encodeshared_import_string {
() => {
// Module: crate::encode
// Provides: {"shared_import_string"}
// Dependencies: {}
fn shared_import_string < 'a > (i : & 'a ast :: ImportString , intern : & 'a Interner) -> ImportString < 'a > { ImportString { shim : intern . intern (& i . shim) , string : & i . string , } }
};
}
