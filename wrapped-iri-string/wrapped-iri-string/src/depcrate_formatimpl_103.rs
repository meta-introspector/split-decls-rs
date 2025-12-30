// Generated macro for impl_103 (impl)
macro_rules! Depcrate_formatimpl_103 {
() => {
// Module: crate::format
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Write for StringWriter < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { if self . error . is_some () { return Err (fmt :: Error) ; } if let Err (e) = self . buffer . try_reserve (s . len ()) { self . error = Some (e) ; return Err (fmt :: Error) ; } self . buffer . push_str (s) ; Ok (()) } }
};
}
