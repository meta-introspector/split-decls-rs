// Generated macro for macro_459 (macro)
macro_rules! Depcrate_arbitrary__std_netmacro_459 {
() => {
// Module: crate::arbitrary::_std::net
// Provides: {"macro_459"}
// Dependencies: {}
arbitrary ! (Ipv6Addr , TupleUnion < (WA < SMapped < Ipv4Addr , Self >>, WA < MapInto < StrategyFor < [u16 ; 8] >, Self >>) >; prop_oneof ! [2 => static_map (any ::< Ipv4Addr > () , | ip | ip . to_ipv6_mapped ()) , 1 => any ::< [u16 ; 8] > () . prop_map_into ()]) ;
};
}
