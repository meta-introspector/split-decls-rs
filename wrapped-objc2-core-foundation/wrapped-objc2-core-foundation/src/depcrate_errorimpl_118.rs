// Generated macro for impl_118 (impl)
macro_rules! Depcrate_errorimpl_118 {
() => {
// Module: crate::error
// Provides: {"impl_118"}
// Dependencies: {}
impl fmt :: Display for CFError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let desc = self . description () . unwrap () ; write ! (f , "{desc}") } }
};
}
