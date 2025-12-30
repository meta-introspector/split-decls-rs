// Generated macro for impl_72 (impl)
macro_rules! Depcrate_serviceimpl_72 {
() => {
// Module: crate::service
// Provides: {"impl_72"}
// Dependencies: {}
impl < S , I > Service < (WorkerCounterGuard , MioStream) > for StreamService < S , I > where S : Service < I > , S :: Future : 'static , S :: Error : 'static , I : FromStream , { type Response = () ; type Error = () ; type Future = Ready < Result < () , () > > ; fn poll_ready (& self , ctx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . service . poll_ready (ctx) . map_err (| _ | ()) } fn call (& self , (guard , req) : (WorkerCounterGuard , MioStream)) -> Self :: Future { ready (match FromStream :: from_mio (req) { Ok (stream) => { let f = self . service . call (stream) ; actix_rt :: spawn (async move { let _ = f . await ; drop (guard) ; }) ; Ok (()) } Err (err) => { error ! ("can not convert to an async TCP stream: {err}") ; Err (()) } }) } }
};
}
