// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl fmt :: Display for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let len = self . arcs () . count () ; for (i , arc) in self . arcs () . enumerate () { write ! (f , "{arc}") ? ; if let Some (j) = i . checked_add (1) { if j < len { write ! (f , ".") ? ; } } } Ok (()) } }
};
}
