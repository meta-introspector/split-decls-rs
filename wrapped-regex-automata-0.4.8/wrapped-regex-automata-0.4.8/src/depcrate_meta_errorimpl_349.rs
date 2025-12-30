// Generated macro for impl_349 (impl)
macro_rules! Depcrate_meta_errorimpl_349 {
() => {
// Module: crate::meta::error
// Provides: {"impl_349"}
// Dependencies: {}
impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind { BuildErrorKind :: Syntax { pid , .. } => { write ! (f , "error parsing pattern {}" , pid . as_usize ()) } BuildErrorKind :: NFA (_) => write ! (f , "error building NFA") , } } }
};
}
