// Generated macro for parse (function)
macro_rules! Depcrate_parseparse {
() => {
// Module: crate::parse
// Provides: {"parse"}
// Dependencies: {}
pub fn parse < B : Buffer > (input : B) -> Result < Literal < B > , ParseError > { let (first , rest) = input . as_bytes () . split_first () . ok_or (perr (None , Empty)) ? ; let second = input . as_bytes () . get (1) . copied () ; match first { b'f' if & * input == "false" => Ok (Literal :: Bool (BoolLit :: False)) , b't' if & * input == "true" => Ok (Literal :: Bool (BoolLit :: True)) , b'0' ..= b'9' => { match input . as_bytes () . get (1 + end_dec_digits (rest)) { Some (b'.') | Some (b'e') | Some (b'E') => FloatLit :: parse (input) . map (Literal :: Float) , _ => IntegerLit :: parse (input) . map (Literal :: Integer) , } } b'\'' => CharLit :: parse (input) . map (Literal :: Char) , b'"' | b'r' => StringLit :: parse (input) . map (Literal :: String) , b'b' if second == Some (b'\'') => ByteLit :: parse (input) . map (Literal :: Byte) , b'b' if second == Some (b'r') || second == Some (b'"') => { ByteStringLit :: parse (input) . map (Literal :: ByteString) } b'c' => CStringLit :: parse (input) . map (Literal :: CString) , _ => Err (perr (None , InvalidLiteral)) , } }
};
}
