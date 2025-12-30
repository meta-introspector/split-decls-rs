// Generated macro for macro_464 (macro)
macro_rules! Depcrate_arbitrary__std_netmacro_464 {
() => {
// Module: crate::arbitrary::_std::net
// Provides: {"macro_464"}
// Dependencies: {}
arbitrary ! (SocketAddr , TupleUnion < (WA < MapInto < StrategyFor < SocketAddrV4 >, Self >>, WA < MapInto < StrategyFor < SocketAddrV6 >, Self >>) >; prop_oneof ! [any ::< SocketAddrV4 > () . prop_map_into () , any ::< SocketAddrV6 > () . prop_map_into ()]) ;
};
}
