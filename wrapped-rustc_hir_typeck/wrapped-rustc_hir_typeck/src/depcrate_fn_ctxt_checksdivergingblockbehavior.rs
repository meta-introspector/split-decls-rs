// Generated macro for DivergingBlockBehavior (enum)
macro_rules! Depcrate_fn_ctxt_checksDivergingBlockBehavior {
() => {
// Module: crate::fn_ctxt::checks
// Provides: {"DivergingBlockBehavior"}
// Dependencies: {}
# [derive (Clone , Copy , Default)] pub (crate) enum DivergingBlockBehavior { # [doc = " This is the current stable behavior:"] # [doc = ""] # [doc = " ```rust"] # [doc = " {"] # [doc = "     return;"] # [doc = " } // block has type = !, even though we are supposedly dropping it with `;`"] # [doc = " ```"] # [default] Never , # [doc = " Alternative behavior:"] # [doc = ""] # [doc = " ```ignore (very-unstable-new-attribute)"] # [doc = " #![rustc_never_type_options(diverging_block_default = \"unit\")]"] # [doc = " {"] # [doc = "     return;"] # [doc = " } // block has type = (), since we are dropping `!` from `return` with `;`"] # [doc = " ```"] Unit , }
};
}
