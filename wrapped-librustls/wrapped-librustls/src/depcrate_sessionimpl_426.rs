// Generated macro for impl_426 (impl)
macro_rules! Depcrate_sessionimpl_426 {
() => {
// Module: crate::session
// Provides: {"impl_426"}
// Dependencies: {}
impl rustls :: server :: StoresServerSessions for SessionStoreBroker { fn put (& self , key : Vec < u8 > , value : Vec < u8 >) -> bool { self . store (key , value) } fn get (& self , key : & [u8]) -> Option < Vec < u8 > > { self . retrieve (key , false) } fn take (& self , key : & [u8]) -> Option < Vec < u8 > > { self . retrieve (key , true) } fn can_cache (& self) -> bool { true } }
};
}
