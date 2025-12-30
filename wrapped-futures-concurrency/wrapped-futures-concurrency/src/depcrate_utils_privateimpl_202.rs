// Generated macro for impl_202 (impl)
macro_rules! Depcrate_utils_privateimpl_202 {
() => {
// Module: crate::utils::private
// Provides: {"impl_202"}
// Dependencies: {}
impl < T , E > Try for Poll < Result < T , E > > { private_impl ! { } type Output = Poll < T > ; type Residual = Result < Infallible , E > ; fn from_output (output : Self :: Output) -> Self { output . map (Ok) } fn from_residual (residual : Self :: Residual) -> Self { match residual { Err (e) => Poll :: Ready (Err (e)) , Ok (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Poll :: Pending => Continue (Poll :: Pending) , Poll :: Ready (Ok (c)) => Continue (Poll :: Ready (c)) , Poll :: Ready (Err (e)) => Break (Err (e)) , } } }
};
}
