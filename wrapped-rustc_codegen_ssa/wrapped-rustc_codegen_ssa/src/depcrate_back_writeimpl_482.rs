// Generated macro for impl_482 (impl)
macro_rules! Depcrate_back_writeimpl_482 {
() => {
// Module: crate::back::write
// Provides: {"impl_482"}
// Dependencies: {}
impl < B : ExtraBackendMethods > Drop for Coordinator < B > { fn drop (& mut self) { if let Some (future) = self . future . take () { drop (self . sender . send (Message :: CodegenAborted :: < B >)) ; drop (future . join ()) ; } } }
};
}
