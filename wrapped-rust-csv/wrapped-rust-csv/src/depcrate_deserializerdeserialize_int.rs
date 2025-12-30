// Generated macro for deserialize_int (macro)
macro_rules! Depcrate_deserializerdeserialize_int {
() => {
// Module: crate::deserializer
// Provides: {"deserialize_int"}
// Dependencies: {}
macro_rules ! deserialize_int { ($ method : ident , $ visit : ident , $ inttype : ty) => { fn $ method < V : Visitor <'de >> (self , visitor : V ,) -> Result < V :: Value , Self :: Error > { let field = self . next_field () ?; let num = if let Some (digits) = field . strip_prefix ("0x") { <$ inttype >:: from_str_radix (digits , 16) } else { field . parse () } ; visitor .$ visit (num . map_err (| err | self . error (DEK :: ParseInt (err))) ?) } } ; }
};
}
