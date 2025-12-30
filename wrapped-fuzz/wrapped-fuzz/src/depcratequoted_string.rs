// Generated macro for quoted_string (function)
macro_rules! Depcratequoted_string {
() => {
// Module: crate
// Provides: {"quoted_string"}
// Dependencies: {}
# [doc = " Parses `quoted-string` as in [RFC 7230 section 3.2.6](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6)."] # [doc = ""] # [doc = " ```text"] # [doc = " quoted-string  = DQUOTE *( qdtext / quoted-pair ) DQUOTE"] # [doc = " qdtext         = HTAB / SP /%x21 / %x23-5B / %x5D-7E / obs-text"] # [doc = " obs-text       = %x80-FF"] # [doc = " quoted-pair    = \"\\\" ( HTAB / SP / VCHAR / obs-text )"] # [doc = " VCHAR          =  %x21-7E"] # [doc = "                ; visible (printing) characters"] # [doc = " ```"] fn quoted_string (input : & str) -> nom :: IResult < & str , ParamValue < '_ > > { trace ! ("quoted_string attempt on {:?}" , input) ; let is_qdtext = | c | match c { '\t' | ' ' | '\x21' | '\x23' ..= '\x5B' | '\x5D' ..= '\x7E' => true , _ => false , } ; let is_escapable = | c | match c { '\t' | ' ' | '\x21' ..= '\x7E' => true , _ => false , } ; delimited (char ('"') , map (consumed (fold_many0 (alt ((value (0 , many1 (satisfy (is_qdtext))) , value (1 , pair (char ('\\') , satisfy (is_escapable))) ,)) , | | 0 , | acc : usize , item : usize | acc + item ,)) , | (raw , escapes) | ParamValue :: new (escapes , raw) ,) , char ('"') ,) (input) }
};
}
