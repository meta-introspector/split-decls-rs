// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > std :: ops :: Index < & 'a PackageId > for Resolve { type Output = Node ; fn index (& self , idx : & 'a PackageId) -> & Self :: Output { self . nodes . iter () . find (| p | p . id == * idx) . unwrap_or_else (| | panic ! ("no Node with this id: {idx:?}")) } }
};
}
