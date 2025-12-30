// Generated macro for impl_139 (impl)
macro_rules! Depcrate_client_extimpl_139 {
() => {
// Module: crate::client_ext
// Provides: {"impl_139"}
// Dependencies: {}
impl < B > SendRequestExt for SendRequest < B > where B : Buf , { fn get (& mut self , uri : & str) -> ResponseFuture { let req = Request :: builder () . uri (uri) . body (()) . expect ("valid uri") ; let (fut , _tx) = self . send_request (req , true) . expect ("send_request") ; fut } }
};
}
