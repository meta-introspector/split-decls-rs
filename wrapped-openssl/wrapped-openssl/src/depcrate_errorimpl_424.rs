// Generated macro for impl_424 (impl)
macro_rules! Depcrate_errorimpl_424 {
() => {
// Module: crate::error
// Provides: {"impl_424"}
// Dependencies: {}
impl ErrorStack { # [doc = " Returns the contents of the OpenSSL error stack."] pub fn get () -> ErrorStack { let mut vec = vec ! [] ; while let Some (err) = Error :: get () { vec . push (err) ; } ErrorStack (vec) } # [doc = " Pushes the errors back onto the OpenSSL error stack."] pub fn put (& self) { for error in self . errors () { error . put () ; } } }
};
}
