// Generated macro for extend_with_matching_product (function)
macro_rules! Depcrate_unnested_or_patternsextend_with_matching_product {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"extend_with_matching_product"}
// Dependencies: {}
# [doc = " Like `extend_with_matching` but for products with > 1 factor, e.g., `C(p_0, ..., p_n)`."] # [doc = " Here, the idea is that we fixate on some `p_k` in `C`,"] # [doc = " allowing it to vary between two `targets` and `ps2` (returned by `extract`),"] # [doc = " while also requiring `ps1[..n] ~ ps2[..n]` (pre) and `ps1[n + 1..] ~ ps2[n + 1..]` (post),"] # [doc = " where `~` denotes semantic equality."] fn extend_with_matching_product (targets : & mut [Pat] , start : usize , alternatives : & mut ThinVec < Pat > , predicate : impl Fn (& PatKind , & [Pat] , usize) -> bool , extract : impl Fn (PatKind) -> ThinVec < Pat > ,) -> bool { (0 .. targets . len ()) . any (| idx | { let tail_or = drain_matching (start , alternatives , | k | predicate (k , targets , idx) , | k | extract (k) . swap_remove (idx) ,) ; extend_with_tail_or (& mut targets [idx] , tail_or) }) }
};
}
