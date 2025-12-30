// Generated macro for RefMulti (struct)
macro_rules! Depcrate_mapref_multipleRefMulti {
() => {
// Module: crate::mapref::multiple
// Provides: {"RefMulti"}
// Dependencies: {}
pub struct RefMulti < 'a , K , V > { _guard : Arc < RwLockReadGuardDetached < 'a > > , k : & 'a K , v : & 'a V , }
};
}
