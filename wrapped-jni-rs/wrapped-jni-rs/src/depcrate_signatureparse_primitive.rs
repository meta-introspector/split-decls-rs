// Generated macro for parse_primitive (function)
macro_rules! Depcrate_signatureparse_primitive {
() => {
// Module: crate::signature
// Provides: {"parse_primitive"}
// Dependencies: {}
fn parse_primitive < S : Stream < Token = char > > (input : & mut S) -> StdParseResult < Primitive , S > where S :: Error : ParseError < char , S :: Range , S :: Position > , { let boolean = token ('Z') . map (| _ | Primitive :: Boolean) ; let byte = token ('B') . map (| _ | Primitive :: Byte) ; let char_type = token ('C') . map (| _ | Primitive :: Char) ; let double = token ('D') . map (| _ | Primitive :: Double) ; let float = token ('F') . map (| _ | Primitive :: Float) ; let int = token ('I') . map (| _ | Primitive :: Int) ; let long = token ('J') . map (| _ | Primitive :: Long) ; let short = token ('S') . map (| _ | Primitive :: Short) ; let void = token ('V') . map (| _ | Primitive :: Void) ; (boolean . or (byte) . or (char_type) . or (double) . or (float) . or (int) . or (long) . or (short) . or (void)) . parse_stream (input) . into () }
};
}
