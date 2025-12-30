// Generated macro for impl_137 (impl)
macro_rules! Depcrate_rwlockimpl_137 {
() => {
// Module: crate::rwlock
// Provides: {"impl_137"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < R , T > Serialize for RwLock < R , T > where R : RawRwLock , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . read () . serialize (serializer) } }
};
}
