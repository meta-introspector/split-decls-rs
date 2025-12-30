// Generated macro for impl_587 (impl)
macro_rules! Depcrate_server_gracefulimpl_587 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_587"}
// Dependencies: {}
# [cfg (feature = "http1")] impl < I , B , S > GracefulConnection for hyper :: server :: conn :: http1 :: Connection < I , S > where S : hyper :: service :: HttpService < hyper :: body :: Incoming , ResBody = B > , S :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , I : hyper :: rt :: Read + hyper :: rt :: Write + Unpin + 'static , B : hyper :: body :: Body + 'static , B :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , { type Error = hyper :: Error ; fn graceful_shutdown (self : Pin < & mut Self >) { hyper :: server :: conn :: http1 :: Connection :: graceful_shutdown (self) ; } }
};
}
