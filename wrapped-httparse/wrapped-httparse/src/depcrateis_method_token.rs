// Generated macro for is_method_token (function)
macro_rules! Depcrateis_method_token {
() => {
// Module: crate
// Provides: {"is_method_token"}
// Dependencies: {}
# [doc = " Determines if byte is a method token char."] # [doc = ""] # [doc = " > ```notrust"] # [doc = " > token          = 1*tchar"] # [doc = " >"] # [doc = " > tchar          = \"!\" / \"#\" / \"$\" / \"%\" / \"&\" / \"'\" / \"*\""] # [doc = " >                / \"+\" / \"-\" / \".\" / \"^\" / \"_\" / \"`\" / \"|\" / \"~\""] # [doc = " >                / DIGIT / ALPHA"] # [doc = " >                ; any VCHAR, except delimiters"] # [doc = " > ```"] # [inline] fn is_method_token (b : u8) -> bool { match b { b'A' ..= b'Z' => true , _ => TOKEN_MAP [b as usize] , } }
};
}
