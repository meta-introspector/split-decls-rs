// Generated macro for gen_mock_ident (function)
macro_rules! Depcrategen_mock_ident {
() => {
// Module: crate
// Provides: {"gen_mock_ident"}
// Dependencies: {}
# [doc = " Generate a mock identifier from the regular one: eg \"Foo\" => \"MockFoo\""] fn gen_mock_ident (ident : & Ident) -> Ident { format_ident ! ("Mock{}" , ident) }
};
}
