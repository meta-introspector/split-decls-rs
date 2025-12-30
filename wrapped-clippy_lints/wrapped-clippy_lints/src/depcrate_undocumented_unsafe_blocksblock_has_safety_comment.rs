// Generated macro for block_has_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksblock_has_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"block_has_safety_comment"}
// Dependencies: {}
# [doc = " Checks if the lines immediately preceding the block contain a safety comment."] fn block_has_safety_comment (cx : & LateContext < '_ > , span : Span , accept_comment_above_attributes : bool) -> bool { matches ! (span_from_macro_expansion_has_safety_comment (cx , span , accept_comment_above_attributes) , HasSafetyComment :: Yes (_ , _)) || span_has_safety_comment (cx , span , accept_comment_above_attributes) }
};
}
