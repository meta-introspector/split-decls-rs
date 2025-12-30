// Generated macro for impl_1212 (impl)
macro_rules! Depcrate_tagimpl_1212 {
() => {
// Module: crate::tag
// Provides: {"impl_1212"}
// Dependencies: {}
impl < 'repo > Binding for Tag < 'repo > { type Raw = * mut raw :: git_tag ; unsafe fn from_raw (raw : * mut raw :: git_tag) -> Tag < 'repo > { Tag { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tag { self . raw } }
};
}
