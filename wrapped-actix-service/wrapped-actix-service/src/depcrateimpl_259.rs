// Generated macro for impl_259 (impl)
macro_rules! Depcrateimpl_259 {
() => {
// Module: crate
// Provides: {"impl_259"}
// Dependencies: {}
# [doc = " This impl is deprecated since v2 because the `Service` trait now receives shared reference."] impl < S , Req > Service < Req > for RefCell < S > where S : Service < Req > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . borrow () . poll_ready (ctx) } fn call (& self , request : Req) -> S :: Future { self . borrow () . call (request) } }
};
}
