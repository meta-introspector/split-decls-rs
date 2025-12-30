// Generated macro for is_whitespace (function)
macro_rules! Depcrate_fmt_rfc2822is_whitespace {
() => {
// Module: crate::fmt::rfc2822
// Provides: {"is_whitespace"}
// Dependencies: {}
# [doc = " Returns true if the given byte is \"whitespace\" as defined by RFC 2822."] # [doc = ""] # [doc = " From S2.2.2:"] # [doc = ""] # [doc = " > Many of these tokens are allowed (according to their syntax) to be"] # [doc = " > introduced or end with comments (as described in section 3.2.3) as well"] # [doc = " > as the space (SP, ASCII value 32) and horizontal tab (HTAB, ASCII value"] # [doc = " > 9) characters (together known as the white space characters, WSP), and"] # [doc = " > those WSP characters are subject to header \"folding\" and \"unfolding\" as"] # [doc = " > described in section 2.2.3."] # [doc = ""] # [doc = " In other words, ASCII space or tab."] # [doc = ""] # [doc = " With all that said, it seems odd to limit this to just spaces or tabs, so"] # [doc = " we relax this and let it absorb any kind of ASCII whitespace. This also"] # [doc = " handles, I believe, most cases of \"folding\" whitespace. (By treating `\\r`"] # [doc = " and `\\n` as whitespace.)"] fn is_whitespace (byte : u8) -> bool { byte . is_ascii_whitespace () }
};
}
