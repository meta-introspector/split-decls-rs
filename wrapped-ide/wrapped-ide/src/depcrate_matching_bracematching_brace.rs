// Generated macro for matching_brace (function)
macro_rules! Depcrate_matching_bracematching_brace {
() => {
// Module: crate::matching_brace
// Provides: {"matching_brace"}
// Dependencies: {}
pub (crate) fn matching_brace (file : & SourceFile , offset : TextSize) -> Option < TextSize > { const BRACES : & [SyntaxKind] = & [T ! ['{'] , T ! ['}'] , T ! ['['] , T ! [']'] , T ! ['('] , T ! [')'] , T ! [<] , T ! [>] , T ! [|] , T ! [|]] ; let (brace_token , brace_idx) = file . syntax () . token_at_offset (offset) . filter_map (| node | { let idx = BRACES . iter () . position (| & brace | brace == node . kind ()) ? ; Some ((node , idx)) }) . last () ? ; let parent = brace_token . parent () ? ; if brace_token . kind () == T ! [|] && ! ast :: ParamList :: can_cast (parent . kind ()) { cov_mark :: hit ! (pipes_not_braces) ; return None ; } let matching_kind = BRACES [brace_idx ^ 1] ; let matching_node = parent . children_with_tokens () . filter_map (| it | it . into_token ()) . find (| node | node . kind () == matching_kind && node != & brace_token) ? ; Some (matching_node . text_range () . start ()) }
};
}
