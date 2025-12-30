// Generated macro for impl_84 (impl)
macro_rules! Depcrate_lockfileimpl_84 {
() => {
// Module: crate::lockfile
// Provides: {"impl_84"}
// Dependencies: {}
impl std :: hash :: Hash for TomlLockfileSourceId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . kind . hash (state) ; self . url . hash (state) ; } }
};
}
