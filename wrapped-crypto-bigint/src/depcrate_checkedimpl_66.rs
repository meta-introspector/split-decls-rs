// Generated macro for impl_66 (impl)
macro_rules! Depcrate_checkedimpl_66 {
() => {
// Module: crate::checked
// Provides: {"impl_66"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Default + Deserialize < 'de > > Deserialize < 'de > for Checked < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let value = Option :: < T > :: deserialize (deserializer) ? ; let choice = Choice :: from (value . is_some () as u8) ; Ok (Self (CtOption :: new (value . unwrap_or_default () , choice))) } }
};
}
