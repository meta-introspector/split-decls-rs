// Generated macro for eq_pre_post (function)
macro_rules! Depcrate_unnested_or_patternseq_pre_post {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"eq_pre_post"}
// Dependencies: {}
# [doc = " Are the patterns in `ps1` and `ps2` equal save for `ps1[idx]` compared to `ps2[idx]`?"] fn eq_pre_post (ps1 : & [Pat] , ps2 : & [Pat] , idx : usize) -> bool { ps1 . len () == ps2 . len () && ps1 [idx] . is_rest () == ps2 [idx] . is_rest () && over (& ps1 [.. idx] , & ps2 [.. idx] , eq_pat) && over (& ps1 [idx + 1 ..] , & ps2 [idx + 1 ..] , eq_pat) }
};
}
