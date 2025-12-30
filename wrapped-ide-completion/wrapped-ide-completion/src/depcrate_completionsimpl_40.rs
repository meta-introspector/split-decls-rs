// Generated macro for impl_40 (impl)
macro_rules! Depcrate_completionsimpl_40 {
() => {
// Module: crate::completions
// Provides: {"impl_40"}
// Dependencies: {}
impl Builder { # [doc = " Convenience method, which allows to add a freshly created completion into accumulator"] # [doc = " without binding it to the variable."] pub (crate) fn add_to (self , acc : & mut Completions , db : & RootDatabase) { acc . add (self . build (db)) } }
};
}
