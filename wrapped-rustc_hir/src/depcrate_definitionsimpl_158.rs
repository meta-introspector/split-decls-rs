// Generated macro for impl_158 (impl)
macro_rules! Depcrate_definitionsimpl_158 {
() => {
// Module: crate::definitions
// Provides: {"impl_158"}
// Dependencies: {}
impl fmt :: Display for DefPathData { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . name () { DefPathDataName :: Named (name) => f . write_str (name . as_str ()) , DefPathDataName :: Anon { namespace } => write ! (f , "{{{{{namespace}}}}}") , } } }
};
}
