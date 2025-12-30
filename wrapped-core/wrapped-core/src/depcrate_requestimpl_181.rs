// Generated macro for impl_181 (impl)
macro_rules! Depcrate_requestimpl_181 {
() => {
// Module: crate::request
// Provides: {"impl_181"}
// Dependencies: {}
impl fmt :: Display for DataIdentifierBorrowed < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . locale , f) ? ; if ! self . marker_attributes . is_empty () { write ! (f , "/{}" , self . marker_attributes . as_str ()) ? ; } Ok (()) } }
};
}
