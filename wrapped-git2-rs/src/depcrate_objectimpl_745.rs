// Generated macro for impl_745 (impl)
macro_rules! Depcrate_objectimpl_745 {
() => {
// Module: crate::object
// Provides: {"impl_745"}
// Dependencies: {}
impl < 'repo > Binding for Object < 'repo > { type Raw = * mut raw :: git_object ; unsafe fn from_raw (raw : * mut raw :: git_object) -> Object < 'repo > { Object { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_object { self . raw } }
};
}
