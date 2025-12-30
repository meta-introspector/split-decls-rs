// Generated macro for impl_19 (impl)
macro_rules! Depcrate_connectionimpl_19 {
() => {
// Module: crate::connection
// Provides: {"impl_19"}
// Dependencies: {}
impl < C : AsRef < Channel > + Process > future :: Future for IOResource < C > { type Output = IOResourceError ; fn poll (mut self : pin :: Pin < & mut Self > , ctx : & mut task :: Context < '_ >) -> task :: Poll < Self :: Output > { match self . poll_internal (ctx) { Ok (()) => task :: Poll :: Pending , Err (e) => task :: Poll :: Ready (e) , } } }
};
}
