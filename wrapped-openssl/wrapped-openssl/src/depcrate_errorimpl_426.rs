// Generated macro for impl_426 (impl)
macro_rules! Depcrate_errorimpl_426 {
() => {
// Module: crate::error
// Provides: {"impl_426"}
// Dependencies: {}
impl fmt :: Display for ErrorStack { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . is_empty () { return fmt . write_str ("OpenSSL error") ; } let mut first = true ; for err in & self . 0 { if ! first { fmt . write_str (", ") ? ; } write ! (fmt , "{}" , err) ? ; first = false ; } Ok (()) } }
};
}
