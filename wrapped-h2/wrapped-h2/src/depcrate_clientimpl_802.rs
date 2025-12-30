// Generated macro for impl_802 (impl)
macro_rules! Depcrate_clientimpl_802 {
() => {
// Module: crate::client
// Provides: {"impl_802"}
// Dependencies: {}
impl < B > Future for ReadySendRequest < B > where B : Buf , { type Output = Result < SendRequest < B > , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match & mut self . inner { Some (send_request) => { ready ! (send_request . poll_ready (cx)) ? ; } None => panic ! ("called `poll` after future completed") , } Poll :: Ready (Ok (self . inner . take () . unwrap ())) } }
};
}
