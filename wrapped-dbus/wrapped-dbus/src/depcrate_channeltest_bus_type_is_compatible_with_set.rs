// Generated macro for test_bus_type_is_compatible_with_set (function)
macro_rules! Depcrate_channeltest_bus_type_is_compatible_with_set {
() => {
// Module: crate::channel
// Provides: {"test_bus_type_is_compatible_with_set"}
// Dependencies: {}
# [test] fn test_bus_type_is_compatible_with_set () { use std :: collections :: HashSet ; let mut set : HashSet < BusType > = HashSet :: new () ; set . insert (BusType :: Starter) ; set . insert (BusType :: Starter) ; assert_eq ! (set . len () , 1) ; assert ! (! set . contains (& BusType :: Session)) ; assert ! (! set . contains (& BusType :: System)) ; assert ! (set . contains (& BusType :: Starter)) ; }
};
}
