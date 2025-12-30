// Generated macro for RefMut (struct)
macro_rules! Depcrate_mapref_oneRefMut {
() => {
// Module: crate::mapref::one
// Provides: {"RefMut"}
// Dependencies: {}
pub struct RefMut < 'a , K , V > { guard : RwLockWriteGuardDetached < 'a > , k : & 'a K , v : & 'a mut V , }
};
}
