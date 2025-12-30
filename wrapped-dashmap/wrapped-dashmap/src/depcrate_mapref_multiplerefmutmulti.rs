// Generated macro for RefMutMulti (struct)
macro_rules! Depcrate_mapref_multipleRefMutMulti {
() => {
// Module: crate::mapref::multiple
// Provides: {"RefMutMulti"}
// Dependencies: {}
pub struct RefMutMulti < 'a , K , V > { _guard : Arc < RwLockWriteGuardDetached < 'a > > , k : & 'a K , v : & 'a mut V , }
};
}
