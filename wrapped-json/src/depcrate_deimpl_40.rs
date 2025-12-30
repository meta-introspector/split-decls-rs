// Generated macro for impl_40 (impl)
macro_rules! Depcrate_deimpl_40 {
() => {
// Module: crate::de
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > Deserializer < read :: StrRead < 'a > > { # [doc = " Creates a JSON deserializer from a `&str`."] pub fn from_str (s : & 'a str) -> Self { Deserializer :: new (read :: StrRead :: new (s)) } }
};
}
