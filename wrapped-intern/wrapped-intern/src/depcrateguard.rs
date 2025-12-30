// Generated macro for Guard (type)
macro_rules! DepcrateGuard {
() => {
// Module: crate
// Provides: {"Guard"}
// Dependencies: {}
type Guard < T > = dashmap :: RwLockWriteGuard < 'static , RawTable < (Arc < T > , SharedValue < () >) > > ;
};
}
