// Generated macro for impl_24 (impl)
macro_rules! Depcrate_deimpl_24 {
() => {
// Module: crate::de
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R > Deserializer < IoRead < R > > where R : io :: Read , { # [doc = " Constructs a `Deserializer` which reads from a `Read`er."] pub fn from_reader (reader : R) -> Deserializer < IoRead < R > > { Deserializer :: new (IoRead :: new (reader)) } }
};
}
