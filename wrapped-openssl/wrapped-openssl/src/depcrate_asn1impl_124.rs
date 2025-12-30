// Generated macro for impl_124 (impl)
macro_rules! Depcrate_asn1impl_124 {
() => {
// Module: crate::asn1
// Provides: {"impl_124"}
// Dependencies: {}
impl Asn1ObjectRef { # [doc = " Returns the NID associated with this OID."] pub fn nid (& self) -> Nid { unsafe { Nid :: from_raw (ffi :: OBJ_obj2nid (self . as_ptr ())) } } }
};
}
