// Generated macro for ReplacementRenderer (struct)
macro_rules! Depcrate_replacingReplacementRenderer {
() => {
// Module: crate::replacing
// Provides: {"ReplacementRenderer"}
// Dependencies: {}
struct ReplacementRenderer < 'a , 'db > { db : & 'db dyn hir :: db :: ExpandDatabase , match_info : & 'a Match , file_src : & 'a str , rules : & 'a [ResolvedRule < 'db >] , rule : & 'a ResolvedRule < 'db > , out : String , placeholder_tokens_by_range : FxHashMap < TextRange , SyntaxToken > , placeholder_tokens_requiring_parenthesis : FxHashSet < SyntaxToken > , edition : Edition , }
};
}
