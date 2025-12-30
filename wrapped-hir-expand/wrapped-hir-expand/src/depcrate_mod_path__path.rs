// Generated macro for __path (macro)
macro_rules! Depcrate_mod_path__path {
() => {
// Module: crate::mod_path
// Provides: {"__path"}
// Dependencies: {}
# [macro_export] macro_rules ! __path { ($ start : ident $ (:: $ seg : ident) *) => ({ $ crate :: __known_path ! ($ start $ (:: $ seg) *) ; $ crate :: mod_path :: ModPath :: from_segments ($ crate :: mod_path :: PathKind :: Abs , vec ! [$ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ start . clone ()) , $ ($ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ seg . clone ()) ,) *]) }) ; }
};
}
