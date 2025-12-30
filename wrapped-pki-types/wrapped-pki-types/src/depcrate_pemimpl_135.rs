// Generated macro for impl_135 (impl)
macro_rules! Depcrate_pemimpl_135 {
() => {
// Module: crate::pem
// Provides: {"impl_135"}
// Dependencies: {}
impl AsRef < [u8] > for SectionLabel { fn as_ref (& self) -> & [u8] { match self { Self :: Known (kind) => kind . as_slice () , Self :: Unknown (ty) => ty , } } }
};
}
