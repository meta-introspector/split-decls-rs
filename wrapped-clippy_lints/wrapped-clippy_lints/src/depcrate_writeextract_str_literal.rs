// Generated macro for extract_str_literal (function)
macro_rules! Depcrate_writeextract_str_literal {
() => {
// Module: crate::write
// Provides: {"extract_str_literal"}
// Dependencies: {}
# [doc = " Removes the raw marker, `#`s and quotes from a str, and returns if the literal is raw"] # [doc = ""] # [doc = " `r#\"a\"#` -> (`a`, true)"] # [doc = ""] # [doc = " `\"b\"` -> (`b`, false)"] fn extract_str_literal (literal : & str) -> Option < (String , bool) > { let (literal , raw) = match literal . strip_prefix ('r') { Some (stripped) => (stripped . trim_matches ('#') , true) , None => (literal , false) , } ; Some ((literal . strip_prefix ('"') ? . strip_suffix ('"') ? . to_string () , raw)) }
};
}
