// Generated macro for impl_31 (impl)
macro_rules! Depcrate_muteximpl_31 {
() => {
// Module: crate::mutex
// Provides: {"impl_31"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < R , T > Serialize for Mutex < R , T > where R : RawMutex , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . lock () . serialize (serializer) } }
};
}
