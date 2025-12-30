// Generated macro for impl_138 (impl)
macro_rules! Depcrate_rwlockimpl_138 {
() => {
// Module: crate::rwlock
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , R , T > Deserialize < 'de > for RwLock < R , T > where R : RawRwLock , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (RwLock :: new) } }
};
}
