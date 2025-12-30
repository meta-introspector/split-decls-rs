// Generated macro for impl_54 (impl)
macro_rules! Depcrate_de_mapimpl_54 {
() => {
// Module: crate::de::map
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "overlapped-lists")] impl < 'de , 'd , 'm , R , E > Drop for MapValueSeqAccess < 'de , 'd , 'm , R , E > where R : XmlRead < 'de > , E : EntityResolver , { fn drop (& mut self) { self . map . de . start_replay (self . checkpoint) ; } }
};
}
