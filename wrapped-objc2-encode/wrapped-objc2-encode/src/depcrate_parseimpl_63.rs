// Generated macro for impl_63 (impl)
macro_rules! Depcrate_parseimpl_63 {
() => {
// Module: crate::parse
// Provides: {"impl_63"}
// Dependencies: {}
impl ParseError { pub (crate) fn new (parser : Parser < '_ > , kind : ErrorKind) -> Self { Self { kind , data : parser . data . to_string () , split_point : parser . split_point , } } }
};
}
