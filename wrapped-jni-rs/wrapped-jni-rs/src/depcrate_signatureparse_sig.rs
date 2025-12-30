// Generated macro for parse_sig (function)
macro_rules! Depcrate_signatureparse_sig {
() => {
// Module: crate::signature
// Provides: {"parse_sig"}
// Dependencies: {}
fn parse_sig < 'a , S > (input : & mut S) -> StdParseResult < TypeSignature , S > where S : RangeStream < Token = char , Range = & 'a str > , S :: Error : ParseError < char , S :: Range , S :: Position > , { (parser (parse_args) , parser (parse_type)) . map (| (a , r) | TypeSignature { args : a , ret : r }) . parse_stream (input) . into () }
};
}
