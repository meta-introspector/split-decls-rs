// Generated macro for impl_340 (impl)
macro_rules! Depcrate_nameimpl_340 {
() => {
// Module: crate::name
// Provides: {"impl_340"}
// Dependencies: {}
impl fmt :: Display for NamespaceError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: UnknownPrefix (prefix) => { f . write_str ("unknown namespace prefix '") ? ; write_byte_string (f , prefix) ? ; f . write_str ("'") } Self :: InvalidXmlPrefixBind (namespace) => { f . write_str ("the namespace prefix 'xml' cannot be bound to '") ? ; write_byte_string (f , namespace) ? ; f . write_str ("'") } Self :: InvalidXmlnsPrefixBind (namespace) => { f . write_str ("the namespace prefix 'xmlns' cannot be bound to '") ? ; write_byte_string (f , namespace) ? ; f . write_str ("'") } Self :: InvalidPrefixForXml (prefix) => { f . write_str ("the namespace prefix '") ? ; write_byte_string (f , prefix) ? ; f . write_str ("' cannot be bound to 'http://www.w3.org/XML/1998/namespace'") } Self :: InvalidPrefixForXmlns (prefix) => { f . write_str ("the namespace prefix '") ? ; write_byte_string (f , prefix) ? ; f . write_str ("' cannot be bound to 'http://www.w3.org/2000/xmlns/'") } } } }
};
}
