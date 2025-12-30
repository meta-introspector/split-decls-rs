// Generated macro for impl_134 (impl)
macro_rules! Depcrate_pemimpl_134 {
() => {
// Module: crate::pem
// Provides: {"impl_134"}
// Dependencies: {}
impl From < & [u8] > for SectionLabel { fn from (value : & [u8]) -> Self { match SectionKind :: try_from (value) { Ok (kind) => Self :: Known (kind) , Err (_) => Self :: Unknown (value . to_vec ()) , } } }
};
}
