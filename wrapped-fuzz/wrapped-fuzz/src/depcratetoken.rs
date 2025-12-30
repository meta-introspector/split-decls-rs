// Generated macro for token (function)
macro_rules! Depcratetoken {
() => {
// Module: crate
// Provides: {"token"}
// Dependencies: {}
# [doc = " Parses a token as in RFC 7230 section 3.2.6."] # [doc = ""] # [doc = " ```text"] # [doc = "      token          = 1*tchar"] # [doc = ""] # [doc = "      tchar          = \"!\" / \"#\" / \"$\" / \"%\" / \"&\" / \"'\" / \"*\""] # [doc = "                     / \"+\" / \"-\" / \".\" / \"^\" / \"_\" / \"`\" / \"|\" / \"~\""] # [doc = "                     / DIGIT / ALPHA"] # [doc = "                     ; any VCHAR, except delimiters"] # [doc = " ```"] fn token (input : & str) -> nom :: IResult < & str , & str > { trace ! ("token attempt on {:?}" , input) ; is_a ("!#$%&'*+-.^_`|~0123456789abcdefghijklmnopqrstuvxwyzABCDEFGHIJKLMNOPQRSTUVWXYZ") (input) }
};
}
