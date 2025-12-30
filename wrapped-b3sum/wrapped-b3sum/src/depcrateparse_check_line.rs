// Generated macro for parse_check_line (function)
macro_rules! Depcrateparse_check_line {
() => {
// Module: crate
// Provides: {"parse_check_line"}
// Dependencies: {}
fn parse_check_line (mut line : & str) -> anyhow :: Result < ParsedCheckLine > { line = line . trim_end_matches (['\r' , '\n']) ; let Some (first) = line . chars () . next () else { bail ! ("Empty line") ; } ; let line_after_slash ; let is_escaped ; if first == '\\' { is_escaped = true ; line_after_slash = & line [1 ..] ; } else { is_escaped = false ; line_after_slash = line ; } let hash_hex ; let file_str ; if let Some ((left , right)) = split_untagged_check_line (line_after_slash) { hash_hex = left ; file_str = right ; } else if let Some ((left , right)) = split_tagged_check_line (line_after_slash) { file_str = left ; hash_hex = right ; } else { bail ! ("Invalid check line format") ; } ensure ! (hash_hex . len () == 2 * blake3 :: OUT_LEN , "Invalid hash length") ; let mut hex_chars = hash_hex . chars () ; let mut hash_bytes = [0 ; blake3 :: OUT_LEN] ; for byte in & mut hash_bytes { let high_char = hex_chars . next () . unwrap () ; let low_char = hex_chars . next () . unwrap () ; * byte = 16 * hex_half_byte (high_char) ? + hex_half_byte (low_char) ? ; } let expected_hash : blake3 :: Hash = hash_bytes . into () ; let file_path_string = if is_escaped { unescape (file_str) ? } else { file_str . to_string () } ; ensure ! (! file_path_string . is_empty () , "empty file path") ; check_for_invalid_characters (& file_path_string) ? ; Ok (ParsedCheckLine { file_string : file_str . to_string () , is_escaped , file_path : file_path_string . into () , expected_hash , }) }
};
}
