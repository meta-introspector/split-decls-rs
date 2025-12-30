// Generated macro for impl_74 (impl)
macro_rules! Depcrate_nttimpl_74 {
() => {
// Module: crate::ntt
// Provides: {"impl_74"}
// Dependencies: {}
impl Mul < & NttPolynomial > for & NttPolynomial { type Output = NttPolynomial ; fn mul (self , rhs : & NttPolynomial) -> NttPolynomial { NttPolynomial :: new (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (& x , & y) | x * y) . collect () ,) } }
};
}
