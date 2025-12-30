// Generated macro for impl_631 (impl)
macro_rules! Depcrate_extensionsimpl_631 {
() => {
// Module: crate::extensions
// Provides: {"impl_631"}
// Dependencies: {}
impl fmt :: Debug for Extensions { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct TypeName (& 'static str) ; impl fmt :: Debug for TypeName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . 0) } } let mut set = f . debug_set () ; if let Some (map) = & self . map { set . entries (map . values () . map (| any_clone | TypeName (any_clone . as_ref () . type_name ())) ,) ; } set . finish () } }
};
}
