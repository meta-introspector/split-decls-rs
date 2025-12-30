// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < S , Req , U , F > Transform < S , Req > for TracingTransform < S , U , F > where S : Service < Req > , U : ServiceFactory < Req , Response = S :: Response , Error = S :: Error , Service = S > , F : Fn (& Req) -> Option < tracing :: Span > + Clone , { type Response = S :: Response ; type Error = S :: Error ; type Transform = TracingService < S , F > ; type InitError = U :: InitError ; type Future = Ready < Result < Self :: Transform , Self :: InitError > > ; fn new_transform (& self , service : S) -> Self :: Future { ok (TracingService :: new (service , self . make_span . clone ())) } }
};
}
