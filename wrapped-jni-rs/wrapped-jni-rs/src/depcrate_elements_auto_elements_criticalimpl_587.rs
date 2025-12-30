// Generated macro for impl_587 (impl)
macro_rules! Depcrate_elements_auto_elements_criticalimpl_587 {
() => {
// Module: crate::elements::auto_elements_critical
// Provides: {"impl_587"}
// Dependencies: {}
impl < 'array_local , T : TypeArray , TArrayRef > From < & AutoElementsCritical < 'array_local , T , TArrayRef > > for * mut T where TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { fn from (other : & AutoElementsCritical < 'array_local , T , TArrayRef >) -> * mut T { other . as_ptr () } }
};
}
