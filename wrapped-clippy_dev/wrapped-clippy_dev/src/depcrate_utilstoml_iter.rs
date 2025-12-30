// Generated macro for toml_iter (function)
macro_rules! Depcrate_utilstoml_iter {
() => {
// Module: crate::utils
// Provides: {"toml_iter"}
// Dependencies: {}
fn toml_iter (s : & str) -> impl Iterator < Item = (usize , TomlPart < '_ >) > { let mut pos = 0 ; s . split ('\n') . map (move | s | { let x = pos ; pos += s . len () + 1 ; (x , s) }) . filter_map (| (pos , s) | { if let Some (s) = s . strip_prefix ('[') { s . split_once (']') . map (| (name , _) | (pos , TomlPart :: Table (name))) } else if matches ! (s . bytes () . next () , Some (b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' | b'_')) { s . split_once ('=') . map (| (key , value) | (pos , TomlPart :: Value (key , value))) } else { None } }) }
};
}
