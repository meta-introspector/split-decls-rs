// Generated macro for make_function_names_table (function)
macro_rules! Depcrate_prf_namesmake_function_names_table {
() => {
// Module: crate::prf_names
// Provides: {"make_function_names_table"}
// Dependencies: {}
# [doc = " Scans through the contents of an LLVM IR assembly file to find `__llvm_prf_names`"] # [doc = " entries, decodes them, and creates a table that maps name hash values to"] # [doc = " (demangled) function names."] pub (crate) fn make_function_names_table (llvm_ir : & str) -> anyhow :: Result < HashMap < u64 , String > > { fn prf_names_payload (line : & str) -> Option < & str > { let re = { static RE : OnceLock < Regex > = OnceLock :: new () ; RE . get_or_init (| | { Regex :: new (r#"^@__llvm_prf_nm =.*\[[0-9]+ x i8\] c"([^"]*)".*$"#) . unwrap () }) } ; let payload = re . captures (line) ? . get (1) . unwrap () . as_str () ; Some (payload) } fn demangle_if_able (symbol_name_bytes : & [u8]) -> anyhow :: Result < String > { let symbol_name_str = std :: str :: from_utf8 (symbol_name_bytes) ? ; match rustc_demangle :: try_demangle (symbol_name_str) { Ok (d) => Ok (format ! ("{d:#}")) , Err (_) => Ok (format ! ("(couldn't demangle) {symbol_name_str}")) , } } let mut map = HashMap :: new () ; for payload in llvm_ir . lines () . filter_map (prf_names_payload) . map (unescape_llvm_string_contents) { let mut parser = Parser :: new (& payload) ; let uncompressed_bytes = parser . read_chunk_to_uncompressed_bytes () ? ; parser . ensure_empty () ? ; for raw_name in uncompressed_bytes . split (| & b | b == 0x01) { let hash = truncated_md5 (raw_name) ; let demangled = demangle_if_able (raw_name) ? ; map . insert (hash , demangled) ; } } Ok (map) }
};
}
