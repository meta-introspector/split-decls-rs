// Generated macro for impl_589 (impl)
macro_rules! Depcrate_elements_auto_elements_criticalimpl_589 {
() => {
// Module: crate::elements::auto_elements_critical
// Provides: {"impl_589"}
// Dependencies: {}
impl < 'array_local , T : TypeArray , TArrayRef > std :: ops :: DerefMut for AutoElementsCritical < 'array_local , T , TArrayRef > where TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { std :: slice :: from_raw_parts_mut (self . ptr . as_mut () , self . len) } } }
};
}
