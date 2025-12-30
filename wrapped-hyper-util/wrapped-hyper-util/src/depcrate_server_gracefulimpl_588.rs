// Generated macro for impl_588 (impl)
macro_rules! Depcrate_server_gracefulimpl_588 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_588"}
// Dependencies: {}
# [cfg (feature = "http2")] impl < I , B , S , E > GracefulConnection for hyper :: server :: conn :: http2 :: Connection < I , S , E > where S : hyper :: service :: HttpService < hyper :: body :: Incoming , ResBody = B > , S :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , I : hyper :: rt :: Read + hyper :: rt :: Write + Unpin + 'static , B : hyper :: body :: Body + 'static , B :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , E : hyper :: rt :: bounds :: Http2ServerConnExec < S :: Future , B > , { type Error = hyper :: Error ; fn graceful_shutdown (self : Pin < & mut Self >) { hyper :: server :: conn :: http2 :: Connection :: graceful_shutdown (self) ; } }
};
}
