// Generated macro for impl_261 (impl)
macro_rules! Depcrateimpl_261 {
() => {
// Module: crate
// Provides: {"impl_261"}
// Dependencies: {}
impl < S , Req > ServiceFactory < Req > for Arc < S > where S : ServiceFactory < Req > , { type Response = S :: Response ; type Error = S :: Error ; type Config = S :: Config ; type Service = S :: Service ; type InitError = S :: InitError ; type Future = S :: Future ; fn new_service (& self , cfg : S :: Config) -> S :: Future { self . as_ref () . new_service (cfg) } }
};
}
