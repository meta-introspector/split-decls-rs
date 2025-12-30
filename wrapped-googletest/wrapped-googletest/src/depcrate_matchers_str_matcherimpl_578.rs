// Generated macro for impl_578 (impl)
macro_rules! Depcrate_matchers_str_matcherimpl_578 {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"impl_578"}
// Dependencies: {}
impl MatchMode { fn to_diff_mode (& self) -> edit_distance :: Mode { match self { MatchMode :: StartsWith | MatchMode :: EndsWith => edit_distance :: Mode :: Prefix , MatchMode :: Contains => edit_distance :: Mode :: Contains , MatchMode :: Equals => edit_distance :: Mode :: Exact , } } }
};
}
