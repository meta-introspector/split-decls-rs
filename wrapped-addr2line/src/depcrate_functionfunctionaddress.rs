// Generated macro for FunctionAddress (struct)
macro_rules! Depcrate_functionFunctionAddress {
() => {
// Module: crate::function
// Provides: {"FunctionAddress"}
// Dependencies: {}
# [doc = " A single address range for a function."] # [doc = ""] # [doc = " It is possible for a function to have multiple address ranges; this"] # [doc = " is handled by having multiple `FunctionAddress` entries with the same"] # [doc = " `function` field."] pub (crate) struct FunctionAddress { range : gimli :: Range , # [doc = " An index into `Functions::functions`."] pub (crate) function : usize , }
};
}
