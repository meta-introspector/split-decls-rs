// Generated macro for macro_1139 (macro)
macro_rules! Depcrate_ntldrmacro_1139 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1139"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrVerifyImageMatchesChecksum (ImageFileHandle : HANDLE , ImportCallbackRoutine : PLDR_IMPORT_MODULE_CALLBACK , ImportCallbackParameter : PVOID , ImageCharacteristics : PUSHORT ,) -> NTSTATUS ; } }
};
}
