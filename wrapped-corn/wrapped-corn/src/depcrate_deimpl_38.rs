// Generated macro for impl_38 (impl)
macro_rules! Depcrate_deimpl_38 {
() => {
// Module: crate::de
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > { pub fn from_str (input : & 'de str) -> Result < Self > { let parsed = parse (input) ? ; Ok (Self :: from_value (parsed)) } fn from_value (value : Value < 'de >) -> Self { Self { value : Some (value) } } }
};
}
