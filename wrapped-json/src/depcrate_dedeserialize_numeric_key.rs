// Generated macro for deserialize_numeric_key (macro)
macro_rules! Depcrate_dedeserialize_numeric_key {
() => {
// Module: crate::de
// Provides: {"deserialize_numeric_key"}
// Dependencies: {}
macro_rules ! deserialize_numeric_key { ($ method : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value > where V : de :: Visitor <'de >, { self . deserialize_number (visitor) } } ; ($ method : ident , $ delegate : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value > where V : de :: Visitor <'de >, { self . de . eat_char () ; match tri ! (self . de . peek ()) { Some (b'0' ..= b'9' | b'-') => { } _ => return Err (self . de . error (ErrorCode :: ExpectedNumericKey)) , } let value = tri ! (self . de .$ delegate (visitor)) ; match tri ! (self . de . peek ()) { Some (b'"') => self . de . eat_char () , _ => return Err (self . de . peek_error (ErrorCode :: ExpectedDoubleQuote)) , } Ok (value) } } ; }
};
}
