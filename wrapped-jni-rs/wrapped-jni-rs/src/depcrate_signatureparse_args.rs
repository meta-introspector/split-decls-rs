// Generated macro for parse_args (function)
macro_rules! Depcrate_signatureparse_args {
() => {
// Module: crate::signature
// Provides: {"parse_args"}
// Dependencies: {}
fn parse_args < 'a , S > (input : & mut S) -> StdParseResult < Vec < JavaType > , S > where S : RangeStream < Token = char , Range = & 'a str > , S :: Error : ParseError < char , S :: Range , S :: Position > , { between (token ('(') , token (')') , many (parser (parse_type))) . parse_stream (input) . into () }
};
}
