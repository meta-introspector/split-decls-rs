// Generated macro for ConfigureRemoteFn (type)
macro_rules! Depcrate_cloneConfigureRemoteFn {
() => {
// Module: crate::clone
// Provides: {"ConfigureRemoteFn"}
// Dependencies: {}
type ConfigureRemoteFn = Box < dyn FnMut (crate :: Remote < '_ >) -> Result < crate :: Remote < '_ > , Box < dyn std :: error :: Error + Send + Sync > > > ;
};
}
