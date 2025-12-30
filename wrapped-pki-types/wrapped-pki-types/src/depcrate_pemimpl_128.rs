// Generated macro for impl_128 (impl)
macro_rules! Depcrate_pemimpl_128 {
() => {
// Module: crate::pem
// Provides: {"impl_128"}
// Dependencies: {}
impl PemObject for (SectionKind , Vec < u8 >) { fn from_pem (kind : SectionKind , der : Vec < u8 >) -> Option < Self > { Some ((kind , der)) } }
};
}
