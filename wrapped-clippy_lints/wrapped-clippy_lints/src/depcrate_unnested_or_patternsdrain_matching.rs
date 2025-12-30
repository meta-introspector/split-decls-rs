// Generated macro for drain_matching (function)
macro_rules! Depcrate_unnested_or_patternsdrain_matching {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"drain_matching"}
// Dependencies: {}
fn drain_matching (start : usize , alternatives : & mut ThinVec < Pat > , predicate : impl Fn (& PatKind) -> bool , extract : impl Fn (PatKind) -> Pat ,) -> ThinVec < Pat > { let mut tail_or = ThinVec :: new () ; let mut idx = 0 ; for pat in ExtractIf :: new (alternatives , | p | { idx += 1 ; idx > start && predicate (& p . kind) }) { tail_or . push (extract (pat . kind)) ; } tail_or }
};
}
