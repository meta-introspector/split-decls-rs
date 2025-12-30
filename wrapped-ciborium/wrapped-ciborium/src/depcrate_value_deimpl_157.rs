// Generated macro for impl_157 (impl)
macro_rules! Depcrate_value_deimpl_157 {
() => {
// Module: crate::value::de
// Provides: {"impl_157"}
// Dependencies: {}
impl Value { # [doc = " Deserializes the `Value` into an object"] # [inline] pub fn deserialized < 'de , T : de :: Deserialize < 'de > > (& self) -> Result < T , Error > { T :: deserialize (Deserializer (self)) } }
};
}
