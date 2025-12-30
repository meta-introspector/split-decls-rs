// Generated macro for impl_356 (impl)
macro_rules! Depcrate_nameimpl_356 {
() => {
// Module: crate::name
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a > Debug for PrefixDeclaration < 'a > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Default => f . write_str ("PrefixDeclaration::Default") , Self :: Named (prefix) => { f . write_str ("PrefixDeclaration::Named(") ? ; write_byte_string (f , prefix) ? ; f . write_str (")") } } } }
};
}
