// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < S , Req , F > Service < Req > for TracingService < S , F > where S : Service < Req > , F : Fn (& Req) -> Option < tracing :: Span > , { type Response = S :: Response ; type Error = S :: Error ; type Future = Either < S :: Future , Instrumented < S :: Future > > ; actix_service :: forward_ready ! (inner) ; fn call (& self , req : Req) -> Self :: Future { let span = (self . make_span) (& req) ; let _enter = span . as_ref () . map (| s | s . enter ()) ; let fut = self . inner . call (req) ; if let Some (span) = span . clone () . map (| span | tracing :: span ! (parent : & span , tracing :: Level :: INFO , "future")) { Either :: right (fut . instrument (span)) } else { Either :: left (fut) } } }
};
}
