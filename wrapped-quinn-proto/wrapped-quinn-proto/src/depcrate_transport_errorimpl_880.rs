// Generated macro for impl_880 (impl)
macro_rules! Depcrate_transport_errorimpl_880 {
() => {
// Module: crate::transport_error
// Provides: {"impl_880"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . code . fmt (f) ? ; if let Some (frame) = self . frame { write ! (f , " in {frame}") ? ; } if ! self . reason . is_empty () { write ! (f , ": {}" , self . reason) ? ; } Ok (()) } }
};
}
