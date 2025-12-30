// Generated macro for impl_304 (impl)
macro_rules! Depcrate_oddimpl_304 {
() => {
// Module: crate::odd
// Provides: {"impl_304"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > + Integer + Zero > Deserialize < 'de > for Odd < T > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let value : T = T :: deserialize (deserializer) ? ; Option :: < Self > :: from (Self :: new (value)) . ok_or (D :: Error :: invalid_value (Unexpected :: Other ("even") , & "a non-zero odd value" ,)) } }
};
}
