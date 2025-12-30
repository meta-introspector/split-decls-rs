// Generated macro for impl_89 (impl)
macro_rules! Depcrate_remuteximpl_89 {
() => {
// Module: crate::remutex
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < R , G , T > Serialize for ReentrantMutex < R , G , T > where R : RawMutex , G : GetThreadId , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . lock () . serialize (serializer) } }
};
}
