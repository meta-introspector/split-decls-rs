// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < T > fmt :: Debug for Idx < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut type_name = std :: any :: type_name :: < T > () ; if let Some (idx) = type_name . rfind (':') { type_name = & type_name [idx + 1 ..] ; } write ! (f , "Idx::<{}>({})" , type_name , self . raw) } }
};
}
