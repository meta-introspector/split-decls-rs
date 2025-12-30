// Generated macro for impl_88 (impl)
macro_rules! Depcrate_asn1impl_88 {
() => {
// Module: crate::asn1
// Provides: {"impl_88"}
// Dependencies: {}
impl PartialEq < Asn1Time > for Asn1TimeRef { fn eq (& self , other : & Asn1Time) -> bool { self . diff (other) . map (| t | t . days == 0 && t . secs == 0) . unwrap_or (false) } }
};
}
