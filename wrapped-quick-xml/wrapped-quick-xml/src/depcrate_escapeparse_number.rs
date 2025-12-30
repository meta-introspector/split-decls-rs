// Generated macro for parse_number (function)
macro_rules! Depcrate_escapeparse_number {
() => {
// Module: crate::escape
// Provides: {"parse_number"}
// Dependencies: {}
pub (crate) fn parse_number (num : & str) -> Result < char , ParseCharRefError > { let code = if let Some (hex) = num . strip_prefix ('x') { from_str_radix (hex , 16) ? } else { from_str_radix (num , 10) ? } ; if code == 0 { return Err (ParseCharRefError :: IllegalCharacter (code)) ; } match std :: char :: from_u32 (code) { Some (c) => Ok (c) , None => Err (ParseCharRefError :: InvalidCodepoint (code)) , } }
};
}
