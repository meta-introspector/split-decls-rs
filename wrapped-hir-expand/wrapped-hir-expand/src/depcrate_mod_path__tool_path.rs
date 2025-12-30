// Generated macro for __tool_path (macro)
macro_rules! Depcrate_mod_path__tool_path {
() => {
// Module: crate::mod_path
// Provides: {"__tool_path"}
// Dependencies: {}
# [macro_export] macro_rules ! __tool_path { ($ start : ident $ (:: $ seg : ident) *) => ({ $ crate :: mod_path :: ModPath :: from_segments ($ crate :: mod_path :: PathKind :: Plain , vec ! [$ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym :: rust_analyzer) , $ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ start . clone ()) , $ ($ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ seg . clone ()) ,) *]) }) ; }
};
}
