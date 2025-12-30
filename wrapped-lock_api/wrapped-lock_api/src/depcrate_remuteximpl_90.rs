// Generated macro for impl_90 (impl)
macro_rules! Depcrate_remuteximpl_90 {
() => {
// Module: crate::remutex
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , R , G , T > Deserialize < 'de > for ReentrantMutex < R , G , T > where R : RawMutex , G : GetThreadId , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (ReentrantMutex :: new) } }
};
}
