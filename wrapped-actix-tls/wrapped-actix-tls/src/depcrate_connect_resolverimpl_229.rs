// Generated macro for impl_229 (impl)
macro_rules! Depcrate_connect_resolverimpl_229 {
() => {
// Module: crate::connect::resolver
// Provides: {"impl_229"}
// Dependencies: {}
impl < R : Host > ServiceFactory < ConnectInfo < R > > for Resolver { type Response = ConnectInfo < R > ; type Error = ConnectError ; type Config = () ; type Service = ResolverService ; type InitError = () ; type Future = Ready < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { ok (self . resolver . clone ()) } }
};
}
