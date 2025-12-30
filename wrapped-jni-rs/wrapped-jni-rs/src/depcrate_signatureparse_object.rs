// Generated macro for parse_object (function)
macro_rules! Depcrate_signatureparse_object {
() => {
// Module: crate::signature
// Provides: {"parse_object"}
// Dependencies: {}
fn parse_object < 'a , S > (input : & mut S) -> StdParseResult < JavaType , S > where S : RangeStream < Token = char , Range = & 'a str > , S :: Error : ParseError < char , & 'a str , S :: Position > , { fn is_unqualified (c : char) -> bool { ! matches ! (c , '.' | ';' | '[' | '/') } let class_body = recognize ((skip_many1 (satisfy (is_unqualified)) , skip_many (token ('/') . with (skip_many1 (satisfy (is_unqualified)))) ,)) ; (token ('L') , class_body . map (| s : & 'a str | s . to_owned ()) , token (';') ,) . map (| (_ , _name , _) | JavaType :: Object) . parse_stream (input) . into () }
};
}
