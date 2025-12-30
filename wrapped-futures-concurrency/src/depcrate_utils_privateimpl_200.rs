// Generated macro for impl_200 (impl)
macro_rules! Depcrate_utils_privateimpl_200 {
() => {
// Module: crate::utils::private
// Provides: {"impl_200"}
// Dependencies: {}
impl < T > Try for Option < T > { private_impl ! { } type Output = T ; type Residual = Option < Infallible > ; fn from_output (output : Self :: Output) -> Self { Some (output) } fn from_residual (residual : Self :: Residual) -> Self { match residual { None => None , Some (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Some (c) => Continue (c) , None => Break (None) , } } }
};
}
