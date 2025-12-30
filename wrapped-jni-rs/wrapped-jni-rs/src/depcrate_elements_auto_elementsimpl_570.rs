// Generated macro for impl_570 (impl)
macro_rules! Depcrate_elements_auto_elementsimpl_570 {
() => {
// Module: crate::elements::auto_elements
// Provides: {"impl_570"}
// Dependencies: {}
impl < 'array_local , T , TArrayRef > From < & AutoElements < 'array_local , T , TArrayRef > > for * mut T where T : TypeArray , TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > + Reference , { fn from (other : & AutoElements < 'array_local , T , TArrayRef >) -> * mut T { other . as_ptr () } }
};
}
