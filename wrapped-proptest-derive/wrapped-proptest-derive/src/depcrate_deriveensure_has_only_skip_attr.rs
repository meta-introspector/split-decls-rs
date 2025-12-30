// Generated macro for ensure_has_only_skip_attr (function)
macro_rules! Depcrate_deriveensure_has_only_skip_attr {
() => {
// Module: crate::derive
// Provides: {"ensure_has_only_skip_attr"}
// Dependencies: {}
# [doc = " Ensures that no other attributes than skip are present."] fn ensure_has_only_skip_attr (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if attrs . params . is_set () { error :: skipped_variant_has_param (ctx , item) ; } if attrs . strategy . is_set () { error :: skipped_variant_has_strat (ctx , item) ; } if attrs . weight . is_some () { error :: skipped_variant_has_weight (ctx , item) ; } if ! attrs . filter . is_empty () { error :: skipped_variant_has_filter (ctx , item) ; } }
};
}
