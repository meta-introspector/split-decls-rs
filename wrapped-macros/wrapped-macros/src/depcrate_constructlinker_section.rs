// Generated macro for linker_section (function)
macro_rules! Depcrate_constructlinker_section {
() => {
// Module: crate::construct
// Provides: {"linker_section"}
// Dependencies: {}
# [doc = " work around restrictions on length and allowed characters imposed by macos linker"] # [doc = " returns (note the comma character for macos):"] # [doc = "   under macos: \".defmt,\" + 16 character hex digest of symbol's hash"] # [doc = "   otherwise:   \".defmt.\" + prefix + symbol"] pub (crate) fn linker_section (for_macos : bool , prefix : Option < & str > , symbol : & str) -> String { let mut sub_section = if let Some (prefix) = prefix { format ! (".{prefix}.{symbol}") } else { format ! (".{symbol}") } ; if for_macos { sub_section = format ! (",{:x}" , hash (& sub_section)) ; } format ! (".defmt{sub_section}") }
};
}
