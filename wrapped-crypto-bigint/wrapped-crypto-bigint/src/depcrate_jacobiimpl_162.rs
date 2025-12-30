// Generated macro for impl_162 (impl)
macro_rules! Depcrate_jacobiimpl_162 {
() => {
// Module: crate::jacobi
// Provides: {"impl_162"}
// Dependencies: {}
impl Neg for JacobiSymbol { type Output = Self ; fn neg (self) -> Self { match self { Self :: Zero => Self :: Zero , Self :: One => Self :: MinusOne , Self :: MinusOne => Self :: One , } } }
};
}
