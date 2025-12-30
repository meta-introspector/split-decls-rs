// Generated macro for macro_1852 (macro)
macro_rules! Depcrate_ntpsapimacro_1852 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1852"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateProcess (ProcessHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , ParentProcess : HANDLE , InheritObjectTable : BOOLEAN , SectionHandle : HANDLE , DebugPort : HANDLE , ExceptionPort : HANDLE ,) -> NTSTATUS ; } }
};
}
