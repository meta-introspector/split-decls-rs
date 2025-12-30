// Generated macro for entrypoint_deprecated (macro)
macro_rules! Depcrate_entrypoint_deprecatedentrypoint_deprecated {
() => {
// Module: crate::entrypoint_deprecated
// Provides: {"entrypoint_deprecated"}
// Dependencies: {}
# [doc = " Declare the program entrypoint."] # [doc = ""] # [doc = " Deserialize the program input arguments and call"] # [doc = " the user defined `process_instruction` function."] # [doc = " Users must call this macro otherwise an entrypoint for"] # [doc = " their program will not be created."] # [macro_export] macro_rules ! entrypoint_deprecated { ($ process_instruction : ident) => { # [doc = " # Safety"] # [no_mangle] pub unsafe extern "C" fn entrypoint (input : * mut u8) -> u64 { let (program_id , accounts , instruction_data) = unsafe { $ crate :: entrypoint_deprecated :: deserialize (input) } ; match $ process_instruction (& program_id , & accounts , & instruction_data) { Ok (()) => $ crate :: entrypoint_deprecated :: SUCCESS , Err (error) => error . into () , } } } ; }
};
}
