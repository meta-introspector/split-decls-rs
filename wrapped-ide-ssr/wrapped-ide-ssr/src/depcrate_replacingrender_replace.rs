// Generated macro for render_replace (function)
macro_rules! Depcrate_replacingrender_replace {
() => {
// Module: crate::replacing
// Provides: {"render_replace"}
// Dependencies: {}
fn render_replace < 'db > (db : & 'db dyn hir :: db :: ExpandDatabase , match_info : & Match , file_src : & str , rules : & [ResolvedRule < 'db >] , edition : Edition ,) -> String { let rule = & rules [match_info . rule_index] ; let template = rule . template . as_ref () . expect ("You called MatchFinder::edits after calling MatchFinder::add_search_pattern") ; let mut renderer = ReplacementRenderer { db , match_info , file_src , rules , rule , out : String :: new () , placeholder_tokens_requiring_parenthesis : FxHashSet :: default () , placeholder_tokens_by_range : FxHashMap :: default () , edition , } ; renderer . render_node (& template . node) ; renderer . maybe_rerender_with_extra_parenthesis (& template . node) ; for comment in & match_info . ignored_comments { renderer . out . push_str (& comment . syntax () . to_string ()) ; } renderer . out }
};
}
