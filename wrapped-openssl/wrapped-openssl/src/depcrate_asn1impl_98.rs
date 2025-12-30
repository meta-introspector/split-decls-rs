// Generated macro for impl_98 (impl)
macro_rules! Depcrate_asn1impl_98 {
() => {
// Module: crate::asn1
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a > PartialEq < & 'a Asn1TimeRef > for Asn1Time { fn eq (& self , other : & & 'a Asn1TimeRef) -> bool { self . diff (other) . map (| t | t . days == 0 && t . secs == 0) . unwrap_or (false) } }
};
}
