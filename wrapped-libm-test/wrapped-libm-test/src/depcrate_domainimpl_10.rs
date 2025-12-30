// Generated macro for impl_10 (impl)
macro_rules! Depcrate_domainimpl_10 {
() => {
// Module: crate::domain
// Provides: {"impl_10"}
// Dependencies: {}
impl < F : fmt :: Debug , I : fmt :: Debug > EitherPrim < F , I > { pub fn unwrap_float (self) -> F { match self { EitherPrim :: Float (f) => f , EitherPrim :: Int (_) => panic ! ("expected float; got {self:?}") , } } pub fn unwrap_int (self) -> I { match self { EitherPrim :: Float (_) => panic ! ("expected int; got {self:?}") , EitherPrim :: Int (i) => i , } } }
};
}
