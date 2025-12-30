// Generated macro for impl_201 (impl)
macro_rules! Depcrate_utils_privateimpl_201 {
() => {
// Module: crate::utils::private
// Provides: {"impl_201"}
// Dependencies: {}
impl < T , E > Try for Result < T , E > { private_impl ! { } type Output = T ; type Residual = Result < Infallible , E > ; fn from_output (output : Self :: Output) -> Self { Ok (output) } fn from_residual (residual : Self :: Residual) -> Self { match residual { Err (e) => Err (e) , Ok (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Ok (c) => Continue (c) , Err (e) => Break (Err (e)) , } } }
};
}
