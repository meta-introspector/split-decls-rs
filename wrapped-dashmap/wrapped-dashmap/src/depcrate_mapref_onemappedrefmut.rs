// Generated macro for MappedRefMut (struct)
macro_rules! Depcrate_mapref_oneMappedRefMut {
() => {
// Module: crate::mapref::one
// Provides: {"MappedRefMut"}
// Dependencies: {}
pub struct MappedRefMut < 'a , K , T : ? Sized > { _guard : RwLockWriteGuardDetached < 'a > , k : & 'a K , v : & 'a mut T , }
};
}
