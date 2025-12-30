// Generated macro for impl_956 (impl)
macro_rules! Depcrate_shims_filesimpl_956 {
() => {
// Module: crate::shims::files
// Provides: {"impl_956"}
// Dependencies: {}
impl FileDescription for io :: Stdout { fn name (& self) -> & 'static str { "stdout" } fn write < 'tcx > (self : FileDescriptionRef < Self > , _communicate_allowed : bool , ptr : Pointer , len : usize , ecx : & mut MiriInterpCx < 'tcx > , finish : DynMachineCallback < 'tcx , Result < usize , IoError > > ,) -> InterpResult < 'tcx > { let result = ecx . write_to_host (& * self , len , ptr) ? ; io :: stdout () . flush () . unwrap () ; finish . call (ecx , result) } fn is_tty (& self , communicate_allowed : bool) -> bool { communicate_allowed && self . is_terminal () } }
};
}
