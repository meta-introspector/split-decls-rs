// Generated macro for impl_101 (impl)
macro_rules! Depcrate_asn1impl_101 {
() => {
// Module: crate::asn1
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a > PartialOrd < & 'a Asn1TimeRef > for Asn1Time { fn partial_cmp (& self , other : & & 'a Asn1TimeRef) -> Option < Ordering > { self . compare (other) . ok () } }
};
}
