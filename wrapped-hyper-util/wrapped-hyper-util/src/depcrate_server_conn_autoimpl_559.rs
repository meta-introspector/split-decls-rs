// Generated macro for impl_559 (impl)
macro_rules! Depcrate_server_conn_autoimpl_559 {
() => {
// Module: crate::server::conn::auto
// Provides: {"impl_559"}
// Dependencies: {}
impl < I , S , E , B > Future for Connection < '_ , I , S , E > where S : Service < Request < Incoming > , Response = Response < B > > , S :: Future : 'static , S :: Error : Into < Box < dyn StdError + Send + Sync > > , B : Body + 'static , B :: Error : Into < Box < dyn StdError + Send + Sync > > , I : Read + Write + Unpin + 'static , E : HttpServerConnExec < S :: Future , B > , { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let mut this = self . as_mut () . project () ; match this . state . as_mut () . project () { ConnStateProj :: ReadVersion { read_version , builder , service , } => { let (version , io) = ready ! (read_version . poll (cx)) ? ; let service = service . take () . unwrap () ; match version { # [cfg (feature = "http1")] Version :: H1 => { let conn = builder . http1 . serve_connection (io , service) ; this . state . set (ConnState :: H1 { conn }) ; } # [cfg (feature = "http2")] Version :: H2 => { let conn = builder . http2 . serve_connection (io , service) ; this . state . set (ConnState :: H2 { conn }) ; } # [cfg (any (not (feature = "http1") , not (feature = "http2")))] _ => return Poll :: Ready (Err (version . unsupported ())) , } } # [cfg (feature = "http1")] ConnStateProj :: H1 { conn } => { return conn . poll (cx) . map_err (Into :: into) ; } # [cfg (feature = "http2")] ConnStateProj :: H2 { conn } => { return conn . poll (cx) . map_err (Into :: into) ; } # [cfg (any (not (feature = "http1") , not (feature = "http2")))] _ => unreachable ! () , } } } }
};
}
