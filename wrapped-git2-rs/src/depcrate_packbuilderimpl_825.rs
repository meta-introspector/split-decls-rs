// Generated macro for impl_825 (impl)
macro_rules! Depcrate_packbuilderimpl_825 {
() => {
// Module: crate::packbuilder
// Provides: {"impl_825"}
// Dependencies: {}
impl < 'repo > Binding for PackBuilder < 'repo > { type Raw = * mut raw :: git_packbuilder ; unsafe fn from_raw (ptr : * mut raw :: git_packbuilder) -> PackBuilder < 'repo > { PackBuilder { raw : ptr , _progress : None , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_packbuilder { self . raw } }
};
}
