// Generated macro for shared_linked_module (function)
macro_rules! Depcrate_encodeshared_linked_module {
() => {
// Module: crate::encode
// Provides: {"shared_linked_module"}
// Dependencies: {}
fn shared_linked_module < 'a > (name : & str , i : & 'a ast :: ImportModule , intern : & 'a Interner ,) -> Result < LinkedModule < 'a > , Diagnostic > { Ok (LinkedModule { module : shared_module (i , intern , true) ? , link_function_name : intern . intern_str (name) , }) }
};
}
