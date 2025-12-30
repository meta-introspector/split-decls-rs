// Generated macro for impl_149 (impl)
macro_rules! Depcrate_serdeimpl_149 {
() => {
// Module: crate::serde
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'de , T , S > Deserialize < 'de > for ArcSwapAny < T , S > where T : RefCnt + Deserialize < 'de > , S : Strategy < T > + Default , { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self :: from (T :: deserialize (deserializer) ?)) } }
};
}
