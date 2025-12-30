// Generated macro for crate_to_string_for_macros (function)
macro_rules! Depcrate_pprustcrate_to_string_for_macros {
() => {
// Module: crate::pprust
// Provides: {"crate_to_string_for_macros"}
// Dependencies: {}
pub fn crate_to_string_for_macros (krate : & ast :: Crate) -> String { State :: to_string (| s | { s . print_inner_attributes (& krate . attrs) ; for item in & krate . items { s . print_item (item) ; } }) }
};
}
