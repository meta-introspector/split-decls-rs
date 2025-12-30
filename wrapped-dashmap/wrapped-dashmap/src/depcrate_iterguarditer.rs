// Generated macro for GuardIter (type)
macro_rules! Depcrate_iterGuardIter {
() => {
// Module: crate::iter
// Provides: {"GuardIter"}
// Dependencies: {}
type GuardIter < 'a , K , V > = (Arc < RwLockReadGuardDetached < 'a > > , hash_table :: Iter < 'a , (K , V) > ,) ;
};
}
