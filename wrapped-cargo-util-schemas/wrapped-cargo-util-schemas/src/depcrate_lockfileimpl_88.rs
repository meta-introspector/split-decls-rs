// Generated macro for impl_88 (impl)
macro_rules! Depcrate_lockfileimpl_88 {
() => {
// Module: crate::lockfile
// Provides: {"impl_88"}
// Dependencies: {}
impl Ord for TomlLockfileSourceId { fn cmp (& self , other : & TomlLockfileSourceId) -> Ordering { self . kind . cmp (& other . kind) . then_with (| | self . url . cmp (& other . url)) } }
};
}
