// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let ParseError :: AbruptEnd { location } = * self { write ! (f , "Parsing ended abruptly while parsing {location}") } else { f . write_str (self . to_static_string ()) } } }
};
}
