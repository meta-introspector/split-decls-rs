// Generated macro for unnest_or_patterns (function)
macro_rules! Depcrate_unnested_or_patternsunnest_or_patterns {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"unnest_or_patterns"}
// Dependencies: {}
# [doc = " Unnest or-patterns `p0 | ... | p1` in the pattern `pat`."] # [doc = " For example, this would transform `Some(0) | FOO | Some(2)` into `Some(0 | 2) | FOO`."] fn unnest_or_patterns (pat : & mut Pat) -> bool { struct Visitor { changed : bool , } impl MutVisitor for Visitor { fn visit_pat (& mut self , p : & mut Pat) { walk_pat (self , p) ; let Or (alternatives) = & mut p . kind else { return } ; let mut idx = 0 ; let mut this_level_changed = false ; while idx < alternatives . len () { let inner = if let Or (ps) = & mut alternatives [idx] . kind { mem :: take (ps) } else { idx += 1 ; continue ; } ; this_level_changed = true ; alternatives . splice (idx ..= idx , inner) ; } let mut focus_idx = 0 ; while focus_idx < alternatives . len () { this_level_changed |= transform_with_focus_on_idx (alternatives , focus_idx) ; focus_idx += 1 ; } self . changed |= this_level_changed ; if this_level_changed { walk_pat (self , p) ; } } } let mut visitor = Visitor { changed : false } ; visitor . visit_pat (pat) ; visitor . changed }
};
}
