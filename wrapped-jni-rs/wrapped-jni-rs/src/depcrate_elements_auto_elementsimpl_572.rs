// Generated macro for impl_572 (impl)
macro_rules! Depcrate_elements_auto_elementsimpl_572 {
() => {
// Module: crate::elements::auto_elements
// Provides: {"impl_572"}
// Dependencies: {}
impl < 'array_local , T , TArrayRef > std :: ops :: DerefMut for AutoElements < 'array_local , T , TArrayRef > where T : TypeArray , TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > + Reference , { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { std :: slice :: from_raw_parts_mut (self . ptr . as_mut () , self . len) } } }
};
}
