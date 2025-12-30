// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl < T > UnknownTransportParameter < T > { # [doc = " Checks whether an unknown Transport Parameter's ID is in the reserved"] # [doc = " space."] # [doc = ""] # [doc = " See Section 18.1 in [RFC9000](https://datatracker.ietf.org/doc/html/rfc9000#name-reserved-transport-paramete)."] pub fn is_reserved (& self) -> bool { let n = (self . id - 27) / 31 ; self . id == 31 * n + 27 } }
};
}
