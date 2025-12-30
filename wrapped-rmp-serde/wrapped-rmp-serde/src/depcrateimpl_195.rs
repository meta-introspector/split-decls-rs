// Generated macro for impl_195 (impl)
macro_rules! Depcrateimpl_195 {
() => {
// Module: crate
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for RawRef < 'de > { # [inline] fn deserialize < D > (de : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > { de . deserialize_any (RawRefVisitor) } }
};
}
