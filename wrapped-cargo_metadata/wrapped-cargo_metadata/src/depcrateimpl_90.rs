// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > std :: ops :: Index < & 'a PackageId > for Metadata { type Output = Package ; fn index (& self , idx : & 'a PackageId) -> & Self :: Output { self . packages . iter () . find (| p | p . id == * idx) . unwrap_or_else (| | panic ! ("no package with this id: {idx:?}")) } }
};
}
