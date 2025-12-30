// Generated macro for impl_157 (impl)
macro_rules! Depcrate_errorimpl_157 {
() => {
// Module: crate::error
// Provides: {"impl_157"}
// Dependencies: {}
impl fmt :: Display for DataError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ICU4X data error") ? ; if self . kind != DataErrorKind :: Custom { write ! (f , ": {}" , self . kind) ? ; } if let Some (marker) = self . marker { write ! (f , " (marker: {marker:?})") ? ; } if let Some (str_context) = self . str_context { write ! (f , ": {str_context}") ? ; } Ok (()) } }
};
}
