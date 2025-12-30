// Generated macro for impl_104 (impl)
macro_rules! Depcrate_targetimpl_104 {
() => {
// Module: crate::target
// Provides: {"impl_104"}
// Dependencies: {}
impl fmt :: Display for Target { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Target :: Object (oid) => oid . fmt (f) , Target :: Symbolic (name) => write ! (f , "ref: {}" , name . as_bstr ()) , } } }
};
}
