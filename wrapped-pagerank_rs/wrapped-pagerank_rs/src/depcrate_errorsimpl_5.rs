// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorsimpl_5 {
() => {
// Module: crate::errors
// Provides: {"impl_5"}
// Dependencies: {}
impl Display for PagerankError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { PagerankError :: CapacityError (msg) => write ! (f , "{}" , msg) , } } }
};
}
