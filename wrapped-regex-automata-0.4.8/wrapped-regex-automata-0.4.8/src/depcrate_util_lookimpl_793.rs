// Generated macro for impl_793 (impl)
macro_rules! Depcrate_util_lookimpl_793 {
() => {
// Module: crate::util::look
// Provides: {"impl_793"}
// Dependencies: {}
impl core :: fmt :: Debug for LookSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . is_empty () { return write ! (f , "∅") ; } for look in self . iter () { write ! (f , "{}" , look . as_char ()) ? ; } Ok (()) } }
};
}
