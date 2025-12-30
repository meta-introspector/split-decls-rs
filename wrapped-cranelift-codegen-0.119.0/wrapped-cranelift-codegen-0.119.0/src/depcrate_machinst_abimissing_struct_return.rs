// Generated macro for missing_struct_return (function)
macro_rules! Depcrate_machinst_abimissing_struct_return {
() => {
// Module: crate::machinst::abi
// Provides: {"missing_struct_return"}
// Dependencies: {}
# [doc = " Returns true if the signature needs to be legalized."] fn missing_struct_return (sig : & ir :: Signature) -> bool { sig . uses_special_param (ArgumentPurpose :: StructReturn) && ! sig . uses_special_return (ArgumentPurpose :: StructReturn) }
};
}
