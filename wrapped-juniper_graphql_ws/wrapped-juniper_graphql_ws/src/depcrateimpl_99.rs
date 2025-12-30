// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < S : ScalarValue , CtxT : Unpin + Send + 'static > Init < S , CtxT > for ConnectionConfig < CtxT > { type Error = Infallible ; type Future = future :: Ready < Result < Self , Self :: Error > > ; fn init (self , _params : Variables < S >) -> Self :: Future { future :: ready (Ok (self)) } }
};
}
