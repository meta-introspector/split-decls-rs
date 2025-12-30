// Generated macro for impl_1195 (impl)
macro_rules! Depcrate_submoduleimpl_1195 {
() => {
// Module: crate::submodule
// Provides: {"impl_1195"}
// Dependencies: {}
impl < 'repo > Binding for Submodule < 'repo > { type Raw = * mut raw :: git_submodule ; unsafe fn from_raw (raw : * mut raw :: git_submodule) -> Submodule < 'repo > { Submodule { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_submodule { self . raw } }
};
}
