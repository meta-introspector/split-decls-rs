// Generated macro for impl_955 (impl)
macro_rules! Depcrate_shims_filesimpl_955 {
() => {
// Module: crate::shims::files
// Provides: {"impl_955"}
// Dependencies: {}
impl FileDescription for io :: Stdin { fn name (& self) -> & 'static str { "stdin" } fn read < 'tcx > (self : FileDescriptionRef < Self > , communicate_allowed : bool , ptr : Pointer , len : usize , ecx : & mut MiriInterpCx < 'tcx > , finish : DynMachineCallback < 'tcx , Result < usize , IoError > > ,) -> InterpResult < 'tcx > { if ! communicate_allowed { helpers :: isolation_abort_error ("`read` from stdin") ? ; } let result = ecx . read_from_host (& * self , len , ptr) ? ; finish . call (ecx , result) } fn is_tty (& self , communicate_allowed : bool) -> bool { communicate_allowed && self . is_terminal () } }
};
}
