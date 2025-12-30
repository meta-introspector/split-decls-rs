// Generated macro for extend_with_matching (function)
macro_rules! Depcrate_unnested_or_patternsextend_with_matching {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"extend_with_matching"}
// Dependencies: {}
fn extend_with_matching (target : & mut Pat , start : usize , alternatives : & mut ThinVec < Pat > , predicate : impl Fn (& PatKind) -> bool , extract : impl Fn (PatKind) -> Pat ,) -> bool { extend_with_tail_or (target , drain_matching (start , alternatives , predicate , extract)) }
};
}
