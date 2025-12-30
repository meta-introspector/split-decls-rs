// Generated macro for macro_572 (macro)
macro_rules! Depcrate_ntexapimacro_572 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_572"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtAddAtomEx (AtomName : PWSTR , Length : ULONG , Atom : PRTL_ATOM , Flags : ULONG ,) -> NTSTATUS ; fn NtFindAtom (AtomName : PWSTR , Length : ULONG , Atom : PRTL_ATOM ,) -> NTSTATUS ; fn NtDeleteAtom (Atom : RTL_ATOM ,) -> NTSTATUS ; } }
};
}
