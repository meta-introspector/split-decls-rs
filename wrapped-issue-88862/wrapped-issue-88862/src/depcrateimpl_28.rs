// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < S , Req > Service < Req > for Box < S > where S : Service < Req > + ? Sized { type Error = S :: Error ; type Future = S :: Future ; fn poll (& self , _ : & mut std :: task :: Context) -> std :: task :: Poll < Result < () , S :: Error > > { todo ! () } fn call (& self , _ : Req) -> S :: Future { todo ! () } }
};
}
