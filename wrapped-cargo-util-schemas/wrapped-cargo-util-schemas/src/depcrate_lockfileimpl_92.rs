// Generated macro for impl_92 (impl)
macro_rules! Depcrate_lockfileimpl_92 {
() => {
// Module: crate::lockfile
// Provides: {"impl_92"}
// Dependencies: {}
impl ser :: Serialize for TomlLockfilePackageId { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { s . collect_str (self) } }
};
}
