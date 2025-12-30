// Generated macro for impl_569 (impl)
macro_rules! Depcrate_elements_auto_elementsimpl_569 {
() => {
// Module: crate::elements::auto_elements
// Provides: {"impl_569"}
// Dependencies: {}
impl < 'array_local , T , TArrayRef > Drop for AutoElements < 'array_local , T , TArrayRef > where T : TypeArray , TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { fn drop (& mut self) { let res = unsafe { self . release_array_elements (self . mode as i32) } ; match res { Ok (()) => { } Err (e) => error ! ("error releasing array: {:#?}" , e) , } } }
};
}
