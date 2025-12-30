// Generated macro for impl_541 (impl)
macro_rules! Depcrate_interpret_memoryimpl_541 {
() => {
// Module: crate::interpret::memory
// Provides: {"impl_541"}
// Dependencies: {}
impl < 'tcx , Other > FnVal < 'tcx , Other > { pub fn as_instance (self) -> InterpResult < 'tcx , Instance < 'tcx > > { match self { FnVal :: Instance (instance) => interp_ok (instance) , FnVal :: Other (_) => { throw_unsup_format ! ("'foreign' function pointers are not supported in this context") } } } }
};
}
