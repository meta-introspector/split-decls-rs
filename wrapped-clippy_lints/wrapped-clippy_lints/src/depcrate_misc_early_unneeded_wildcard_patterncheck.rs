// Generated macro for check (function)
macro_rules! Depcrate_misc_early_unneeded_wildcard_patterncheck {
() => {
// Module: crate::misc_early::unneeded_wildcard_pattern
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , pat : & Pat) { if let PatKind :: TupleStruct (_ , _ , ref patterns) | PatKind :: Tuple (ref patterns) = pat . kind && let Some (rest_index) = patterns . iter () . position (Pat :: is_rest) { if let Some ((left_index , left_pat)) = patterns [.. rest_index] . iter () . rev () . take_while (| pat | matches ! (pat . kind , PatKind :: Wild)) . enumerate () . last () { span_lint (cx , left_pat . span . until (patterns [rest_index] . span) , left_index == 0) ; } if let Some ((right_index , right_pat)) = patterns [rest_index + 1 ..] . iter () . take_while (| pat | matches ! (pat . kind , PatKind :: Wild)) . enumerate () . last () { span_lint (cx , patterns [rest_index] . span . shrink_to_hi () . to (right_pat . span) , right_index == 0 ,) ; } } }
};
}
