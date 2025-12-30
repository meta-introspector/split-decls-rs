// Generated macro for deserialize_numeric_key (macro)
macro_rules! Depcrate_value_dedeserialize_numeric_key {
() => {
// Module: crate::value::de
// Provides: {"deserialize_numeric_key"}
// Dependencies: {}
macro_rules ! deserialize_numeric_key { ($ method : ident) => { deserialize_numeric_key ! ($ method , deserialize_number) ; } ; ($ method : ident , $ using : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { let mut de = crate :: Deserializer :: from_str (& self . key) ; match tri ! (de . peek ()) { Some (b'0' ..= b'9' | b'-') => { } _ => return Err (Error :: syntax (ErrorCode :: ExpectedNumericKey , 0 , 0)) , } let number = tri ! (de .$ using (visitor)) ; if tri ! (de . peek ()) . is_some () { return Err (Error :: syntax (ErrorCode :: ExpectedNumericKey , 0 , 0)) ; } Ok (number) } } ; }
};
}
