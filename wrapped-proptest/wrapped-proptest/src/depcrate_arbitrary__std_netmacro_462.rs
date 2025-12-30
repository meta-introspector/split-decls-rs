// Generated macro for macro_462 (macro)
macro_rules! Depcrate_arbitrary__std_netmacro_462 {
() => {
// Module: crate::arbitrary::_std::net
// Provides: {"macro_462"}
// Dependencies: {}
arbitrary ! (IpAddr , TupleUnion < (WA < MapInto < StrategyFor < Ipv4Addr >, Self >>, WA < MapInto < StrategyFor < Ipv6Addr >, Self >>) >; prop_oneof ! [any ::< Ipv4Addr > () . prop_map_into () , any ::< Ipv6Addr > () . prop_map_into ()]) ;
};
}
