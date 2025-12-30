// Generated macro for parse_array (function)
macro_rules! Depcrate_signatureparse_array {
() => {
// Module: crate::signature
// Provides: {"parse_array"}
// Dependencies: {}
fn parse_array < 'a , S > (input : & mut S) -> StdParseResult < JavaType , S > where S : RangeStream < Token = char , Range = & 'a str > , S :: Error : ParseError < char , S :: Range , S :: Position > , { let marker = token ('[') ; (marker , parser (parse_type)) . map (| (_ , _ty) | JavaType :: Array) . parse_stream (input) . into () }
};
}
