// Generated macro for interned_string (function)
macro_rules! Depcrate_constructinterned_string {
() => {
// Module: crate::construct
// Provides: {"interned_string"}
// Dependencies: {}
pub (crate) fn interned_string (string : & str , tag : & str , is_log_statement : bool , prefix : Option < & str > , defmt_path : & syn :: Path ,) -> TokenStream2 { let var_name = if is_log_statement { format_ident ! ("DEFMT_LOG_STATEMENT") } else { format_ident ! ("S") } ; let var_addr = if cfg ! (feature = "unstable-test") { quote ! ({ # defmt_path :: export :: fetch_add_string_index () }) } else { let var_item = static_variable (& var_name , string , tag , prefix) ; quote ! ({ # var_item &# var_name as * const u8 as u16 }) } ; quote ! ({ # defmt_path :: export :: make_istr (# var_addr) }) }
};
}
