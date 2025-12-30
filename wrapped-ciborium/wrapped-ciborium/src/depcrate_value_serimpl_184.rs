// Generated macro for impl_184 (impl)
macro_rules! Depcrate_value_serimpl_184 {
() => {
// Module: crate::value::ser
// Provides: {"impl_184"}
// Dependencies: {}
impl Value { # [doc = " Serializes an object into a `Value`"] # [inline] pub fn serialized < T : ? Sized + ser :: Serialize > (value : & T) -> Result < Self , Error > { value . serialize (Serializer (())) } }
};
}
