// Generated macro for GuardIterMut (type)
macro_rules! Depcrate_iterGuardIterMut {
() => {
// Module: crate::iter
// Provides: {"GuardIterMut"}
// Dependencies: {}
type GuardIterMut < 'a , K , V > = (Arc < RwLockWriteGuardDetached < 'a > > , hash_table :: IterMut < 'a , (K , V) > ,) ;
};
}
