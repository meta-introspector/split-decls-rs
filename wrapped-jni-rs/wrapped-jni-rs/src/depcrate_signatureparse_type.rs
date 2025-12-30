// Generated macro for parse_type (function)
macro_rules! Depcrate_signatureparse_type {
() => {
// Module: crate::signature
// Provides: {"parse_type"}
// Dependencies: {}
fn parse_type < 'a , S > (input : & mut S) -> StdParseResult < JavaType , S > where S : RangeStream < Token = char , Range = & 'a str > , S :: Error : ParseError < char , & 'a str , S :: Position > , { parser (parse_primitive) . map (JavaType :: Primitive) . or (parser (parse_array)) . or (parser (parse_object)) . parse_stream (input) . into () }
};
}
