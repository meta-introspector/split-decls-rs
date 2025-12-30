// Generated macro for impl_203 (impl)
macro_rules! Depcrate_utils_privateimpl_203 {
() => {
// Module: crate::utils::private
// Provides: {"impl_203"}
// Dependencies: {}
impl < T , E > Try for Poll < Option < Result < T , E > > > { private_impl ! { } type Output = Poll < Option < T > > ; type Residual = Result < Infallible , E > ; fn from_output (output : Self :: Output) -> Self { match output { Poll :: Ready (o) => Poll :: Ready (o . map (Ok)) , Poll :: Pending => Poll :: Pending , } } fn from_residual (residual : Self :: Residual) -> Self { match residual { Err (e) => Poll :: Ready (Some (Err (e))) , Ok (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Poll :: Pending => Continue (Poll :: Pending) , Poll :: Ready (None) => Continue (Poll :: Ready (None)) , Poll :: Ready (Some (Ok (c))) => Continue (Poll :: Ready (Some (c))) , Poll :: Ready (Some (Err (e))) => Break (Err (e)) , } } }
};
}
