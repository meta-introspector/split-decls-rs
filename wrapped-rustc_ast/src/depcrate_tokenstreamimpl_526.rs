// Generated macro for impl_526 (impl)
macro_rules! Depcrate_tokenstreamimpl_526 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_526"}
// Dependencies: {}
impl NodeRange { pub fn new (ParserRange (parser_range) : ParserRange , start_pos : u32) -> NodeRange { assert ! (! parser_range . is_empty ()) ; assert ! (parser_range . start >= start_pos) ; NodeRange ((parser_range . start - start_pos) .. (parser_range . end - start_pos)) } }
};
}
