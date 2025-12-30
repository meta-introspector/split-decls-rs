// Generated macro for wmul_impl_usize (macro)
macro_rules! Depcrate_distr_utilswmul_impl_usize {
() => {
// Module: crate::distr::utils
// Provides: {"wmul_impl_usize"}
// Dependencies: {}
macro_rules ! wmul_impl_usize { ($ ty : ty) => { impl WideningMultiply for usize { type Output = (usize , usize) ; # [inline (always)] fn wmul (self , x : usize) -> Self :: Output { let (high , low) = (self as $ ty) . wmul (x as $ ty) ; (high as usize , low as usize) } } } ; }
};
}
