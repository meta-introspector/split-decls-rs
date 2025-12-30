// Generated macro for impl_517 (impl)
macro_rules! Depcrate_tokenstreamimpl_517 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_517"}
// Dependencies: {}
impl LazyAttrTokenStream { pub fn new_direct (stream : AttrTokenStream) -> LazyAttrTokenStream { LazyAttrTokenStream (Arc :: new (LazyAttrTokenStreamInner :: Direct (stream))) } pub fn new_pending (start_token : (Token , Spacing) , cursor_snapshot : TokenCursor , num_calls : u32 , break_last_token : u32 , node_replacements : ThinVec < NodeReplacement > ,) -> LazyAttrTokenStream { LazyAttrTokenStream (Arc :: new (LazyAttrTokenStreamInner :: Pending { start_token , cursor_snapshot , num_calls , break_last_token , node_replacements , })) } pub fn to_attr_token_stream (& self) -> AttrTokenStream { self . 0 . to_attr_token_stream () } }
};
}
