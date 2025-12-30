// Generated macro for impl_1085 (impl)
macro_rules! Depcrate_repoimpl_1085 {
() => {
// Module: crate::repo
// Provides: {"impl_1085"}
// Dependencies: {}
impl Binding for Repository { type Raw = * mut raw :: git_repository ; unsafe fn from_raw (ptr : * mut raw :: git_repository) -> Repository { Repository { raw : ptr } } fn raw (& self) -> * mut raw :: git_repository { self . raw } }
};
}
