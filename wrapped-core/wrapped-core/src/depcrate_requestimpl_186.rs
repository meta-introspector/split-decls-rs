// Generated macro for impl_186 (impl)
macro_rules! Depcrate_requestimpl_186 {
() => {
// Module: crate::request
// Provides: {"impl_186"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Display for DataIdentifierCow < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . locale , f) ? ; if ! self . marker_attributes . is_empty () { write ! (f , "/{}" , self . marker_attributes . as_str ()) ? ; } Ok (()) } }
};
}
