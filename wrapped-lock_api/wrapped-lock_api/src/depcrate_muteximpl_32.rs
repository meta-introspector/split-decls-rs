// Generated macro for impl_32 (impl)
macro_rules! Depcrate_muteximpl_32 {
() => {
// Module: crate::mutex
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , R , T > Deserialize < 'de > for Mutex < R , T > where R : RawMutex , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (Mutex :: new) } }
};
}
