// Generated macro for impl_87 (impl)
macro_rules! Depcrate_asn1impl_87 {
() => {
// Module: crate::asn1
// Provides: {"impl_87"}
// Dependencies: {}
impl PartialEq for Asn1TimeRef { fn eq (& self , other : & Asn1TimeRef) -> bool { self . diff (other) . map (| t | t . days == 0 && t . secs == 0) . unwrap_or (false) } }
};
}
