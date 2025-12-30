// Generated macro for ConfigureConnectionFn (type)
macro_rules! Depcrate_cloneConfigureConnectionFn {
() => {
// Module: crate::clone
// Provides: {"ConfigureConnectionFn"}
// Dependencies: {}
# [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] type ConfigureConnectionFn = Box < dyn FnMut (& mut remote :: Connection < '_ , '_ , Box < dyn Transport + Send > > ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > > , > ;
};
}
