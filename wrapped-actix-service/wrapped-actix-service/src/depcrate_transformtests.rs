// Generated macro for tests (module)
macro_rules! Depcrate_transformtests {
() => {
// Module: crate::transform
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: time :: Duration ; use actix_utils :: future :: { ready , Ready } ; use super :: * ; # [allow (unused)] pub struct TimeoutTransform { timeout : Duration , } impl < S : Service < Req > , Req > Transform < S , Req > for TimeoutTransform { type Response = S :: Response ; type Error = S :: Error ; type InitError = S :: Error ; type Transform = Timeout < S > ; type Future = Ready < Result < Self :: Transform , Self :: InitError > > ; fn new_transform (& self , service : S) -> Self :: Future { ready (Ok (Timeout { service , _timeout : self . timeout , })) } } # [allow (unused)] pub struct Timeout < S > { service : S , _timeout : Duration , } impl < S : Service < Req > , Req > Service < Req > for Timeout < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; crate :: forward_ready ! (service) ; fn call (& self , req : Req) -> Self :: Future { self . service . call (req) } } }
};
}
