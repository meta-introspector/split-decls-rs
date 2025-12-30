// Generated macro for impl_862 (impl)
macro_rules! Depcrate_serverimpl_862 {
() => {
// Module: crate::server
// Provides: {"impl_862"}
// Dependencies: {}
impl < T , B > Future for Flush < T , B > where T : AsyncWrite + Unpin , B : Buf , { type Output = Result < Codec < T , B > , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { ready ! (self . codec . as_mut () . unwrap () . flush (cx)) . map_err (crate :: Error :: from_io) ? ; Poll :: Ready (Ok (self . codec . take () . unwrap ())) } }
};
}
