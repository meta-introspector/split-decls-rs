// Generated macro for impl_189 (impl)
macro_rules! Depcrateimpl_189 {
() => {
// Module: crate
// Provides: {"impl_189"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Raw { # [inline] fn deserialize < D > (de : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > { de . deserialize_any (RawVisitor) } }
};
}
