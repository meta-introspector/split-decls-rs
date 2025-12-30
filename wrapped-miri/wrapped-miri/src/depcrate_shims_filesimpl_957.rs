// Generated macro for impl_957 (impl)
macro_rules! Depcrate_shims_filesimpl_957 {
() => {
// Module: crate::shims::files
// Provides: {"impl_957"}
// Dependencies: {}
impl FileDescription for io :: Stderr { fn name (& self) -> & 'static str { "stderr" } fn write < 'tcx > (self : FileDescriptionRef < Self > , _communicate_allowed : bool , ptr : Pointer , len : usize , ecx : & mut MiriInterpCx < 'tcx > , finish : DynMachineCallback < 'tcx , Result < usize , IoError > > ,) -> InterpResult < 'tcx > { let result = ecx . write_to_host (& * self , len , ptr) ? ; finish . call (ecx , result) } fn is_tty (& self , communicate_allowed : bool) -> bool { communicate_allowed && self . is_terminal () } }
};
}
