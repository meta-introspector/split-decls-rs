// Generated macro for extend_with_tail_or (function)
macro_rules! Depcrate_unnested_or_patternsextend_with_tail_or {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"extend_with_tail_or"}
// Dependencies: {}
# [doc = " Extend `target` as an or-pattern with the alternatives"] # [doc = " in `tail_or` if there are any and return if there were."] fn extend_with_tail_or (target : & mut Pat , tail_or : ThinVec < Pat >) -> bool { fn extend (target : & mut Pat , mut tail_or : ThinVec < Pat >) { if let Or (ps) = & mut target . kind { ps . append (& mut tail_or) ; } else { let mut init_or = thin_vec ! [take_pat (target)] ; init_or . append (& mut tail_or) ; target . kind = Or (init_or) ; } } let changed = ! tail_or . is_empty () ; if changed { extend (target , tail_or) ; } changed }
};
}
