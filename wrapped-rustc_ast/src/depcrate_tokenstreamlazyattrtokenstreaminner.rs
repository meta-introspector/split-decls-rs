// Generated macro for LazyAttrTokenStreamInner (enum)
macro_rules! Depcrate_tokenstreamLazyAttrTokenStreamInner {
() => {
// Module: crate::tokenstream
// Provides: {"LazyAttrTokenStreamInner"}
// Dependencies: {}
enum LazyAttrTokenStreamInner { Direct (AttrTokenStream) , Pending { start_token : (Token , Spacing) , cursor_snapshot : TokenCursor , num_calls : u32 , break_last_token : u32 , node_replacements : ThinVec < NodeReplacement > , } , }
};
}
