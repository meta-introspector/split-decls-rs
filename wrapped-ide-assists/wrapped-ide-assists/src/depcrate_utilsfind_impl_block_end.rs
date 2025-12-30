// Generated macro for find_impl_block_end (function)
macro_rules! Depcrate_utilsfind_impl_block_end {
() => {
// Module: crate::utils
// Provides: {"find_impl_block_end"}
// Dependencies: {}
# [doc = " Find the end of the `impl` block for the given `ast::Impl`."] pub (crate) fn find_impl_block_end (impl_def : ast :: Impl , buf : & mut String) -> Option < TextSize > { buf . push ('\n') ; let end = impl_def . assoc_item_list () . and_then (| it | it . r_curly_token ()) ? . prev_sibling_or_token () ? . text_range () . end () ; Some (end) }
};
}
