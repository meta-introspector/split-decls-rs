// Generated macro for impl_586 (impl)
macro_rules! Depcrate_elements_auto_elements_criticalimpl_586 {
() => {
// Module: crate::elements::auto_elements_critical
// Provides: {"impl_586"}
// Dependencies: {}
impl < 'array_local , T : TypeArray , TArrayRef > Drop for AutoElementsCritical < 'array_local , T , TArrayRef > where TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { fn drop (& mut self) { let res = unsafe { self . release_primitive_array_critical (self . mode as i32) } ; match res { Ok (()) => { } Err (e) => error ! ("error releasing primitive array: {:#?}" , e) , } } }
};
}
