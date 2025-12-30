// Generated macro for impl_325 (impl)
macro_rules! Depcrate_re_bytesimpl_325 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_325"}
// Dependencies: {}
impl < F > Replacer for F where F : FnMut (& Captures) -> Vec < u8 > { fn replace_append (& mut self , caps : & Captures , dst : & mut Vec < u8 >) { extend_from_slice (dst , & (* self) (caps)) ; } }
};
}
