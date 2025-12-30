// Generated macro for impl_82 (impl)
macro_rules! Depcrate_lockfileimpl_82 {
() => {
// Module: crate::lockfile
// Provides: {"impl_82"}
// Dependencies: {}
impl ser :: Serialize for TomlLockfileSourceId { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { s . collect_str (& self . as_url ()) } }
};
}
