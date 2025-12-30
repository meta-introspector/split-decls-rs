// Generated macro for impl_18 (impl)
macro_rules! Depcrate_testsimpl_18 {
() => {
// Module: crate::tests
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for IntCode { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { IntCode :: Zero => write ! (f , "Z0") , IntCode :: Pos (ref inner) => write ! (f , "PInt<{}>" , inner) , IntCode :: Neg (ref inner) => write ! (f , "NInt<{}>" , inner) , } } }
};
}
