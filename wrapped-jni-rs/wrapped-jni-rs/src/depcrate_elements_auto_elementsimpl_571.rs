// Generated macro for impl_571 (impl)
macro_rules! Depcrate_elements_auto_elementsimpl_571 {
() => {
// Module: crate::elements::auto_elements
// Provides: {"impl_571"}
// Dependencies: {}
impl < 'array_local , T , TArrayRef > std :: ops :: Deref for AutoElements < 'array_local , T , TArrayRef > where T : TypeArray , TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > + Reference , { type Target = [T] ; fn deref (& self) -> & Self :: Target { unsafe { std :: slice :: from_raw_parts (self . ptr . as_ptr () , self . len) } } }
};
}
