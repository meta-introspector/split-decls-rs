// Generated macro for impl_40 (impl)
macro_rules! Depcrate_deimpl_40 {
() => {
// Module: crate::de
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'de , T > DeserializeSeed < 'de > for erase :: DeserializeSeed < T > where T : serde :: de :: DeserializeSeed < 'de > , { fn erased_deserialize_seed (& mut self , deserializer : & mut dyn Deserializer < 'de > ,) -> Result < Out , Error > { unsafe { self . take () . deserialize (deserializer) . unsafe_map (Out :: new) } } }
};
}
