// Generated macro for byte_parser (macro)
macro_rules! Depcrate_parser_bytebyte_parser {
() => {
// Module: crate::parser::byte
// Provides: {"byte_parser"}
// Dependencies: {}
macro_rules ! byte_parser { ($ name : ident , $ ty : ident , $ f : ident) => { { satisfy (| c : u8 | c .$ f ()) . expected (stringify ! ($ name)) } } ; ($ name : ident , $ ty : ident , $ f : ident $ ($ args : tt) +) => { { satisfy (| c : u8 | c .$ f $ ($ args) +) . expected (stringify ! ($ name)) } } ; }
};
}
