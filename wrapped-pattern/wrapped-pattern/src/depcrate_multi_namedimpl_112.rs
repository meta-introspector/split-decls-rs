// Generated macro for impl_112 (impl)
macro_rules! Depcrate_multi_namedimpl_112 {
() => {
// Module: crate::multi_named
// Provides: {"impl_112"}
// Dependencies: {}
impl Writeable for MissingNamedPlaceholderError < '_ > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . write_char ('{') ? ; sink . write_str (self . name) ? ; sink . write_char ('}') ? ; Ok (()) } }
};
}
