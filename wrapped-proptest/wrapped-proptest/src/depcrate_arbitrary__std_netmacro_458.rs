// Generated macro for macro_458 (macro)
macro_rules! Depcrate_arbitrary__std_netmacro_458 {
() => {
// Module: crate::arbitrary::_std::net
// Provides: {"macro_458"}
// Dependencies: {}
arbitrary ! (Ipv4Addr , TupleUnion < (WA < Just < Self >>, WA < Just < Self >>, WA < MapInto < StrategyFor < u32 >, Self >>) >; prop_oneof ! [1 => Just (Self :: new (0 , 0 , 0 , 0)) , 4 => Just (Self :: new (127 , 0 , 0 , 1)) , 10 => any ::< u32 > () . prop_map_into ()]) ;
};
}
