// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
impl < F , S , CtxT , Fut , E > Init < S , CtxT > for F where S : ScalarValue , F : FnOnce (Variables < S >) -> Fut + Unpin + 'static , Fut : Future < Output = Result < ConnectionConfig < CtxT > , E > > + Send + 'static , E : Error , { type Error = E ; type Future = Fut ; fn init (self , params : Variables < S >) -> Fut { self (params) } }
};
}
