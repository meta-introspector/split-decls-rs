// Generated macro for impl_96 (impl)
macro_rules! Depcrate_asn1impl_96 {
() => {
// Module: crate::asn1
// Provides: {"impl_96"}
// Dependencies: {}
impl PartialEq for Asn1Time { fn eq (& self , other : & Asn1Time) -> bool { self . diff (other) . map (| t | t . days == 0 && t . secs == 0) . unwrap_or (false) } }
};
}
