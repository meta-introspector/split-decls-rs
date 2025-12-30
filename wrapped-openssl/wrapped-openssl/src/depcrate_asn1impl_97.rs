// Generated macro for impl_97 (impl)
macro_rules! Depcrate_asn1impl_97 {
() => {
// Module: crate::asn1
// Provides: {"impl_97"}
// Dependencies: {}
impl PartialEq < Asn1TimeRef > for Asn1Time { fn eq (& self , other : & Asn1TimeRef) -> bool { self . diff (other) . map (| t | t . days == 0 && t . secs == 0) . unwrap_or (false) } }
};
}
