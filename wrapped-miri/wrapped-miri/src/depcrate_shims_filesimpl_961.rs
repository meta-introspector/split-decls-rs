// Generated macro for impl_961 (impl)
macro_rules! Depcrate_shims_filesimpl_961 {
() => {
// Module: crate::shims::files
// Provides: {"impl_961"}
// Dependencies: {}
impl FileDescription for NullOutput { fn name (& self) -> & 'static str { "stderr and stdout" } fn write < 'tcx > (self : FileDescriptionRef < Self > , _communicate_allowed : bool , _ptr : Pointer , len : usize , ecx : & mut MiriInterpCx < 'tcx > , finish : DynMachineCallback < 'tcx , Result < usize , IoError > > ,) -> InterpResult < 'tcx > { finish . call (ecx , Ok (len)) } }
};
}
