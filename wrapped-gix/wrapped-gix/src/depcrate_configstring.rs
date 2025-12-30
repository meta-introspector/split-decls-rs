// Generated macro for string (module)
macro_rules! Depcrate_configstring {
() => {
// Module: crate::config
// Provides: {"string"}
// Dependencies: {}
# [doc = ""] pub mod string { # [doc = " The error produced when failing to interpret configuration as UTF-8 encoded string."] pub type Error = super :: key :: Error < crate :: bstr :: Utf8Error , 'w' , 'd' > ; }
};
}
