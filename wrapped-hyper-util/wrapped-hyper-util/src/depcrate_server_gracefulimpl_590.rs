// Generated macro for impl_590 (impl)
macro_rules! Depcrate_server_gracefulimpl_590 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_590"}
// Dependencies: {}
# [cfg (feature = "server-auto")] impl < I , B , S , E > GracefulConnection for crate :: server :: conn :: auto :: UpgradeableConnection < '_ , I , S , E > where S : hyper :: service :: Service < http :: Request < hyper :: body :: Incoming > , Response = http :: Response < B > > , S :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , S :: Future : 'static , I : hyper :: rt :: Read + hyper :: rt :: Write + Unpin + Send + 'static , B : hyper :: body :: Body + 'static , B :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , E : hyper :: rt :: bounds :: Http2ServerConnExec < S :: Future , B > , { type Error = Box < dyn std :: error :: Error + Send + Sync > ; fn graceful_shutdown (self : Pin < & mut Self >) { crate :: server :: conn :: auto :: UpgradeableConnection :: graceful_shutdown (self) ; } }
};
}
