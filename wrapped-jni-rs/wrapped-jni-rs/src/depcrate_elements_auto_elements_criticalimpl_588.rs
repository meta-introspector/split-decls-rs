// Generated macro for impl_588 (impl)
macro_rules! Depcrate_elements_auto_elements_criticalimpl_588 {
() => {
// Module: crate::elements::auto_elements_critical
// Provides: {"impl_588"}
// Dependencies: {}
impl < 'array_local , T : TypeArray , TArrayRef > std :: ops :: Deref for AutoElementsCritical < 'array_local , T , TArrayRef > where TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { type Target = [T] ; fn deref (& self) -> & Self :: Target { unsafe { std :: slice :: from_raw_parts (self . ptr . as_ptr () , self . len) } } }
};
}
