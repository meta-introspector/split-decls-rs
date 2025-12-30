// Generated macro for make_external_symbol (function)
macro_rules! Depcrate_symbol_namemake_external_symbol {
() => {
// Module: crate::symbol_name
// Provides: {"make_external_symbol"}
// Dependencies: {}
# [doc = " encode symbol as alphanumeric by hex-encoding special characters"] pub fn make_external_symbol (module_name : & str , name : & str , variant : abi :: AbiVariant) -> String { if module_name . is_empty () || module_name == "$root" { make_external_component (name) } else { let mut res = make_external_component (module_name) ; res . push_str (if matches ! (variant , abi :: AbiVariant :: GuestExport) { "X23" } else { "X00" }) ; res . push_str (& make_external_component (name)) ; res } }
};
}
